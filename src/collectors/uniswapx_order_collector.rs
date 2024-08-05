use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use futures::{stream, StreamExt};
use reqwest::Client;
use serde::Deserialize;
use tokio::time::Duration;
use tokio_stream::wrappers::IntervalStream;

static UNISWAPX_API_URL: &str = "https://api.uniswap.org/v2";
static POLL_INTERVAL_SECS: u64 = 5;

#[derive(Debug, PartialEq, Eq)]
pub enum OrderType {
    Dutch,
    Priority,
}

impl OrderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderType::Dutch => "Dutch_V2",
            OrderType::Priority => "Priority",
        }
    }

    pub fn from_str(s: &str) -> Option<OrderType> {
        match s {
            "Dutch_V2" => Some(OrderType::Dutch),
            "Priority" => Some(OrderType::Priority),
            _ => None,
        }
    }
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Dutch
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UniswapXOrder {
    #[serde(rename = "encodedOrder")]
    pub encoded_order: String,
    pub signature: String,
    #[serde(rename = "orderStatus")]
    pub order_status: String,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    #[serde(rename = "orderHash")]
    pub order_hash: String,
}

/// A new order event, containing the internal order.
#[derive(Debug, Clone, Deserialize)]
pub struct UniswapXOrderResponse {
    pub orders: Vec<UniswapXOrder>,
}

/// A collector that listens for new orders on UniswapX, and generates a stream of
/// [events](UniswapXOrder) which contain the order.
#[derive(Default)]
pub struct UniswapXOrderCollector {
    pub client: Client,
    pub base_url: String,
    pub chain_id: u64,
    pub order_type: OrderType,
}

impl UniswapXOrderCollector {
    pub fn new(chain_id: u64, order_type: OrderType) -> Self {
        Self {
            client: Client::new(),
            base_url: UNISWAPX_API_URL.to_string(),
            chain_id,
            order_type,
        }
    }
}

/// Implementation of the [Collector](Collector) trait for the
/// [UniswapXOrderCollector](UniswapXOrderCollector).
// TODO: implement order deduplication
#[async_trait]
impl Collector<UniswapXOrder> for UniswapXOrderCollector {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, UniswapXOrder>> {
        let url = format!(
            "{}/orders?orderStatus=open&chainId={}&orderType={}",
            self.base_url,
            self.chain_id,
            self.order_type.as_str()
        );

        // stream that polls the UniswapX API every 5 seconds
        let stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(
            POLL_INTERVAL_SECS,
        )))
        .then(move |_| {
            let url = url.clone();
            let client = self.client.clone();
            async move {
                let response = client.get(url).send().await?;
                let data = response.json::<UniswapXOrderResponse>().await?;
                Ok(data.orders)
            }
        })
        .flat_map(
            |values_result: Result<Vec<UniswapXOrder>>| match values_result {
                Ok(values) => stream::iter(values.into_iter().map(Ok)).left_stream(),
                Err(e) => stream::once(async { Err(e) }).right_stream(),
            },
        )
        .filter_map(|result| async {
            match result {
                Ok(value) => Some(value),
                Err(_) => None, // if Err, ignore the value
            }
        });

        Ok(Box::pin(stream))
    }
}

#[cfg(test)]
mod tests {
    use crate::collectors::uniswapx_order_collector::UniswapXOrderCollector;
    use artemis_core::types::Collector;
    use ethers::utils::hex;
    use futures::StreamExt;
    use mockito::{Mock, Server, ServerGuard};
    use uniswapx_rs::order::V2DutchOrder;

    async fn get_collector(mock_response: &str) -> (UniswapXOrderCollector, ServerGuard, Mock) {
        let mut server = Server::new_async().await;
        let url = server.url();
        let mock = server
            .mock("GET", "/orders")
            .match_query(mockito::Matcher::UrlEncoded(
                "orderStatus".into(),
                "open".into(),
            ))
            .with_body(mock_response)
            .create_async()
            .await;
        let res = UniswapXOrderCollector {
            client: reqwest::Client::new(),
            base_url: url.clone(),
            chain_id: 1,
            order_type: super::OrderType::Dutch,
        };

        (res, server, mock)
    }

    #[tokio::test]
    async fn creates_order_stream() {
        let response = r#"
{"orders":[{"outputs":[{"recipient":"0xa7152fad7467857dc2d4060fecaadf9f6b8227d3","startAmount":"49170578098130169","endAmount":"48924725207639518","token":"0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619"}],"encodedOrder":"0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000012000000000000000000000000000000000000000000000000000000000647cb78400000000000000000000000000000000000000000000000000000000647cb7c0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002791bca1f2de4661ed88a30c99a7a9449aa841740000000000000000000000000000000000000000000000000000000005915ddf0000000000000000000000000000000000000000000000000000000005915ddf0000000000000000000000000000000000000000000000000000000000000200000000000000000000000000bd7f9d0239f81c94b728d827a87b9864972661ec000000000000000000000000a7152fad7467857dc2d4060fecaadf9f6b8227d304683201ee09ab48f5120a626b494a18097ae556b98be2a2b837f27680c3c10100000000000000000000000000000000000000000000000000000000647cb7c0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000007ceb23fd6bc0add59e62ac25578270cff1b9f61900000000000000000000000000000000000000000000000000aeb06158f08cf900000000000000000000000000000000000000000000000000add0c742bc25de000000000000000000000000a7152fad7467857dc2d4060fecaadf9f6b8227d3","signature":"0xcbb1cf009667108a4b67a4cff8a4383f024e2a2ab8474390cd542af51fc73fd304ad3ec568e93975c6db3b58ffef3878a2265df0c245079b7c6886ca316d3ae41b","input":{"endAmount":"93412831","token":"0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174","startAmount":"93412831"},"orderStatus":"expired","createdAt":1685895015,"chainId":137,"orderHash":"0x3097f9cf452520c6e8f598f0765a7a19249a7355223664cacf9a86b7c5a46a4a","offerer":"0xa7152fad7467857dc2d4060fecaadf9f6b8227d3","type":"DutchLimit"},{"outputs":[{"recipient":"0xa7152fad7467857dc2d4060fecaadf9f6b8227d3","startAmount":"90372873217533917628","endAmount":"89921008851446248039","token":"0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063"}],"encodedOrder":"0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000120000000000000000000000000000000000000000000000000000000006478f005000000000000000000000000000000000000000000000000000000006478f041000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007ceb23fd6bc0add59e62ac25578270cff1b9f61900000000000000000000000000000000000000000000000000ab5539be549ebf00000000000000000000000000000000000000000000000000ab5539be549ebf0000000000000000000000000000000000000000000000000000000000000200000000000000000000000000bd7f9d0239f81c94b728d827a87b9864972661ec000000000000000000000000a7152fad7467857dc2d4060fecaadf9f6b8227d3046832d7b44716690e184f259e40d9f91b68acd9ab920781e23904f0f02c4b01000000000000000000000000000000000000000000000000000000006478f041000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000008f3cf7ad23cd3cadbd9735aff958023239c6a063000000000000000000000000000000000000000000000004e62cf1601685b9bc000000000000000000000000000000000000000000000004dfe79920e335b267000000000000000000000000a7152fad7467857dc2d4060fecaadf9f6b8227d3","signature":"0xb35f6531121423f0a61dfe1939460ebddc3f078743f91f0f2eda209424eff20239069b5743615143a475ae6922305ca8c8fd9e35a18fc200297f61c5e76676271c","input":{"endAmount":"48225927512235711","token":"0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619","startAmount":"48225927512235711"},"orderStatus":"expired","createdAt":1685647336,"chainId":137,"orderHash":"0x0ea53d4ce1524dda9d667e6ba2e0bf3e630d72ebc8946f1528e4a693f2b8b2e9","offerer":"0xa7152fad7467857dc2d4060fecaadf9f6b8227d3","type":"DutchLimit"}]}
        "#;
        let (collector, _server, mock) = get_collector(response).await;
        // get event stream and parse events
        let stream = collector.get_event_stream().await.unwrap();
        let (first_order, stream) = stream.into_future().await;
        assert!(first_order.is_some());
        assert_eq!(
            first_order.unwrap().order_hash,
            "0x3097f9cf452520c6e8f598f0765a7a19249a7355223664cacf9a86b7c5a46a4a"
        );

        let (second_order, _) = stream.into_future().await;
        assert!(second_order.is_some());
        assert_eq!(
            second_order.unwrap().order_hash,
            "0x0ea53d4ce1524dda9d667e6ba2e0bf3e630d72ebc8946f1528e4a693f2b8b2e9"
        );
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn decodes_v2_order() {
        let response = r#"
{"orders":[{"type":"Dutch_V2","orderStatus":"open","signature":"0x6eb32e7912d333e9c1ab162db02ed1656cdc8fbea2e21e70cd3634e8a3bd85d0582b46cacb584412ef3e035837b005b70f67897969426f9795128ea52de3a8cf1b","encodedOrder":"0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001000000000000000000000000004449cd34d1eb1fedcf02a1be3834ffde8e6a61800000000000000000000000006982508145454ce325ddbe47a25d4ec3d23119330000000000000000000000000000000000000000000422ca8b0a00a4250000000000000000000000000000000000000000000000000422ca8b0a00a42500000000000000000000000000000000000000000000000000000000000000000001e00000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000000000042000000000000000000000000000000011f84b9aa48e5f8aa8b9897600006289be000000000000000000000000c9838bbf85ad068136e8da07021e9e131201901904683298fe8b71446644eba514e387688690bde85b7bcaf8de44455a6aaf7a3000000000000000000000000000000000000000000000000000000000669adac5000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003c330a127f1ec70000000000000000000000000000000000000000000000000034be9ca1484989000000000000000000000000c9838bbf85ad068136e8da07021e9e131201901900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000269fc8de5047000000000000000000000000000000000000000000000000000021d754744fbe000000000000000000000000000000fee13a103a10d593b9ae06b3e05f2e7e1c00000000000000000000000000000000000000000000000000000000669ad9b600000000000000000000000000000000000000000000000000000000669ad9f20000000000000000000000006f1cdbbb4d53d226cf4b917bf768b94acbab61680000000000000000000000000000000000000000000000000000000000000064000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000003c64146542c1fd00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000041d90e87f6f9e84487bfbb5170e856a332769359664c72f90250ee8917baf3a5920e87d331fcf97456e5d4d88761c552a9115569861aa96120b56d882339bbaac91c00000000000000000000000000000000000000000000000000000000000000","chainId":1,"nonce":"1993352701105935839386570705396248068916924096291549856616269381900329515568","orderHash":"0x382f612930c2121ed91fcdc00972f76b4adbef8d111830e1d135ac944a144876","swapper":"0xC9838Bbf85Ad068136E8DA07021E9e1312019019","input":{"token":"0x6982508145454Ce325dDbE47a25d4ec3d2311933","startAmount":"5000000000000000000000000","endAmount":"5000000000000000000000000"},"outputs":[{"token":"0x0000000000000000000000000000000000000000","startAmount":"16944616955649735","endAmount":"14846278718998921","recipient":"0xC9838Bbf85Ad068136E8DA07021E9e1312019019"},{"token":"0x0000000000000000000000000000000000000000","startAmount":"42467711668295","endAmount":"37208718593982","recipient":"0x000000fee13a103A10D593b9AE06b3e05F2E7E1c"}],"cosignerData":{"decayStartTime":1721424310,"decayEndTime":1721424370,"exclusiveFiller":"0x6F1cDbBb4d53d226CF4B917bF768B94acbAB6168","inputOverride":"0","outputOverrides":["16998537363636733","0"]},"cosignature":"0xd90e87f6f9e84487bfbb5170e856a332769359664c72f90250ee8917baf3a5920e87d331fcf97456e5d4d88761c552a9115569861aa96120b56d882339bbaac91c","quoteId":"221f421a-455d-4358-8376-6b4fb0ffb0f1","requestId":"775eea31-3173-4f1c-b7d2-bcd6fbcf2301","createdAt":1721424286}]}        "#;
        let (collector, _server, _) = get_collector(response).await;
        // get event stream and parse events
        let stream = collector.get_event_stream().await.unwrap();
        let (first_order, _) = stream.into_future().await;
        assert!(first_order.is_some());
        assert_eq!(
            first_order.clone().unwrap().order_hash,
            "0x382f612930c2121ed91fcdc00972f76b4adbef8d111830e1d135ac944a144876"
        );
        let encoded_order = &first_order.unwrap().encoded_order;
        let encoded_order = if encoded_order.starts_with("0x") {
            &encoded_order[2..]
        } else {
            encoded_order
        };
        let order_hex: Vec<u8> = hex::decode(encoded_order).unwrap();

        let result = V2DutchOrder::_decode(&order_hex, false);
        match result {
            Err(e) => panic!("Error decoding order: {:?}", e),
            _ => (),
        }
    }
}
