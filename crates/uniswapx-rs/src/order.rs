use std::error::Error;

use alloy_primitives::Uint;
use alloy_sol_types::{sol, SolType};
use anyhow::Result;

sol! {
    #[derive(Debug)]
    struct OrderInfo {
        address reactor;
        address swapper;
        uint256 nonce;
        uint256 deadline;
        address additionalValidationContract;
        bytes additionalValidationData;
    }

    #[derive(Debug)]
    struct DutchOutput {
        address token;
        uint256 startAmount;
        uint256 endAmount;
        address recipient;
    }

    #[derive(Debug)]
    struct DutchInput {
        address token;
        uint256 startAmount;
        uint256 endAmount;
    }

    #[derive(Debug)]
    struct ExclusiveDutchOrder {
        OrderInfo info;
        uint256 decayStartTime;
        uint256 decayEndTime;
        address exclusiveFiller;
        uint256 exclusivityOverrideBps;
        DutchInput input;
        DutchOutput[] outputs;
    }
    
    #[derive(Debug)]
    struct PriorityInput {
        address token;
        uint256 amount;
        uint256 mpsPerPriorityFeeWei;
    }

    #[derive(Debug)]
    struct PriorityOutput {
        address token;
        uint256 amount;
        uint256 mpsPerPriorityFeeWei;
        address recipient;
    }

    #[derive(Debug)]
    struct PriorityCosignerData {
        uint256 auctionTargetBlock;
    }

    #[derive(Debug)]
    struct PriorityOrder {
        OrderInfo info;
        address cosigner;
        uint256 auctionStartBlock;
        uint256 baselinePriorityFeeWei;
        PriorityInput input;
        PriorityOutput[] outputs;
        PriorityCosignerData cosignerData;
        bytes cosignature;
    }
}

pub trait Order: Sized {
    fn _decode(order_hex: &[u8], validate: bool) -> Result<Self, Box<dyn Error>>;
    fn _encode(&self) -> Vec<u8>;
}

// Generic function to decode orders
pub fn decode_order<T: Order>(encoded_order: &str) -> Result<T, Box<dyn Error>> {
    let encoded_order = if encoded_order.starts_with("0x") {
        &encoded_order[2..]
    } else {
        encoded_order
    };
    let order_hex = hex::decode(encoded_order)?;

    T::_decode(&order_hex, false)
}

// Generic function to encode orders
pub fn encode_order<T: Order>(order: &T) -> Vec<u8> {
    order._encode()
}

#[derive(Debug, Clone)]
pub struct ResolvedInput {
    pub token: String,
    pub amount: Uint<256, 4>,
}

#[derive(Debug, Clone)]
pub struct ResolvedOutput {
    pub token: String,
    pub amount: Uint<256, 4>,
    pub recipient: String,
}

#[derive(Debug, Clone)]
pub struct ResolvedOrder {
    pub input: ResolvedInput,
    pub outputs: Vec<ResolvedOutput>,
}

#[derive(Debug)]
pub enum OrderResolution {
    Resolved(ResolvedOrder),
    Expired,
    Invalid,
}

impl Order for ExclusiveDutchOrder {
    fn _decode(order_hex: &[u8], validate: bool) -> Result<Self, Box<dyn Error>> {
        Ok(ExclusiveDutchOrder::decode(order_hex, validate)?)
    }

    fn _encode(&self) -> Vec<u8> {
        ExclusiveDutchOrder::encode(self)
    }
}

impl ExclusiveDutchOrder {
    pub fn resolve(&self, timestamp: u64) -> OrderResolution {
        let timestamp = Uint::from(timestamp);

        if self.info.deadline.lt(&timestamp) {
            return OrderResolution::Expired;
        };

        // resolve over the decay curve

        let input = ResolvedInput {
            token: self.input.token.to_string(),
            amount: resolve_decay(
                timestamp,
                self.decayStartTime,
                self.decayEndTime,
                self.input.startAmount,
                self.input.endAmount,
            ),
        };

        let outputs = self
            .outputs
            .iter()
            .map(|output| {
                let mut amount = resolve_decay(
                    timestamp,
                    self.decayStartTime,
                    self.decayEndTime,
                    output.startAmount,
                    output.endAmount,
                );

                // add exclusivity override to amount
                if self.decayStartTime.gt(&timestamp) && !self.exclusiveFiller.is_zero() {
                    let exclusivity = self.exclusivityOverrideBps.wrapping_add(Uint::from(10000));
                    let exclusivity = exclusivity.wrapping_mul(amount);
                    amount = exclusivity.wrapping_div(Uint::from(10000));
                };

                ResolvedOutput {
                    token: output.token.to_string(),
                    amount,
                    recipient: output.recipient.to_string(),
                }
            })
            .collect();

        OrderResolution::Resolved(ResolvedOrder { input, outputs })
    }
}

impl Order for PriorityOrder {
    fn _decode(order_hex: &[u8], validate: bool) -> Result<Self, Box<dyn Error>> {
        Ok(PriorityOrder::decode(order_hex, validate)?)
    }

    fn _encode(&self) -> Vec<u8> {
        PriorityOrder::encode(self)
    }
}

impl PriorityOrder {
    pub fn resolve(&self, priority_fee: Uint<256, 4>) -> OrderResolution {
        let input = self.input.scale(priority_fee);
        let outputs = self
            .outputs
            .iter()
            .map(|output| output.scale(priority_fee))
            .collect();

        OrderResolution::Resolved(ResolvedOrder { input, outputs })
    }
}

impl PriorityInput {
    pub fn scale(&self, priority_fee: Uint<256, 4>) -> ResolvedInput {
        let amount = self.amount.wrapping_mul(Uint::from(1e7).wrapping_add(priority_fee.wrapping_mul(self.mpsPerPriorityFeeWei))).wrapping_div(Uint::from(1e7));
        ResolvedInput {
            token: self.token.to_string(),
            amount,
        }
    }
}

impl PriorityOutput {
    pub fn scale(&self, priority_fee: Uint<256, 4>) -> ResolvedOutput {
        let amount = self.amount.wrapping_mul(Uint::from(1e7).saturating_sub(priority_fee.wrapping_mul(self.mpsPerPriorityFeeWei))).wrapping_div(Uint::from(1e7));
        ResolvedOutput {
            token: self.token.to_string(),
            amount,
            recipient: self.recipient.to_string(),
        }
    }

}

fn resolve_decay(
    at_time: Uint<256, 4>,
    start_time: Uint<256, 4>,
    end_time: Uint<256, 4>,
    start_amount: Uint<256, 4>,
    end_amount: Uint<256, 4>,
) -> Uint<256, 4> {
    if end_time.le(&at_time) {
        return end_amount;
    }

    if at_time.le(&start_time) {
        return start_amount;
    }

    if end_time.eq(&start_time) {
        return start_amount;
    }

    if start_amount.eq(&end_amount) {
        return start_amount;
    }

    let duration = end_time.wrapping_sub(start_time);
    let elapsed = at_time.wrapping_sub(start_time);
    // TODO: better handle overflows
    if start_amount.gt(&end_amount) {
        // decaying downward
        let decay = start_amount
            .wrapping_sub(end_amount)
            .wrapping_mul(elapsed)
            .wrapping_div(duration);
        return start_amount.wrapping_sub(decay);
    } else {
        // decaying upward
        let decay = end_amount
            .wrapping_sub(start_amount)
            .wrapping_mul(elapsed)
            .wrapping_div(duration);
        return start_amount.wrapping_add(decay);
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay_after_end_time() {
        let start_time = Uint::from(1);
        let end_time = Uint::from(10);
        let start_amount = Uint::from(100000);
        let end_amount = Uint::from(100000000);

        let at_time = Uint::from(11);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, end_amount);
    }

    #[test]
    fn test_decay_at_end_time() {
        let start_time = Uint::from(1);
        let end_time = Uint::from(10);
        let start_amount = Uint::from(100000);
        let end_amount = Uint::from(100000000);

        let at_time = Uint::from(10);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, end_amount);
    }

    #[test]
    fn test_decay_before_start_time() {
        let start_time = Uint::from(10);
        let end_time = Uint::from(100);
        let start_amount = Uint::from(100000);
        let end_amount = Uint::from(100000000);

        let at_time = Uint::from(5);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, start_amount);
    }

    #[test]
    fn test_decay_at_start_time() {
        let start_time = Uint::from(10);
        let end_time = Uint::from(100);
        let start_amount = Uint::from(100000);
        let end_amount = Uint::from(100000000);

        let at_time = Uint::from(10);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, start_amount);
    }

    #[test]
    fn test_upwards_decay() {
        let start_time = Uint::from(10);
        let end_time = Uint::from(20);
        let start_amount = Uint::from(100000);
        let end_amount = Uint::from(200000);

        let at_time = Uint::from(15);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, Uint::from(150000));
    }

    #[test]
    fn test_downwards_decay() {
        let start_time = Uint::from(10);
        let end_time = Uint::from(20);
        let start_amount = Uint::from(200000);
        let end_amount = Uint::from(100000);

        let at_time = Uint::from(15);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, Uint::from(150000));
    }
}
