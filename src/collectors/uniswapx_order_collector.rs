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
pub const CHAIN_ID: u64 = 1;

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
}

impl UniswapXOrderCollector {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: UNISWAPX_API_URL.to_string(),
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
            "{}/orders?orderStatus=open&chainId={}",
            self.base_url, CHAIN_ID
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
    use crate::types::Collector;
    use futures::StreamExt;
    use mockito::{Mock, Server, ServerGuard};

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
}
