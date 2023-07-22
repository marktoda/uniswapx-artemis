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
}

pub fn decode_order(encoded_order: &str) -> Result<ExclusiveDutchOrder> {
    let encoded_order = if encoded_order.starts_with("0x") {
        &encoded_order[2..]
    } else {
        encoded_order
    };
    let order_hex = hex::decode(encoded_order)?;

    Ok(ExclusiveDutchOrder::decode_single(&order_hex, false)?)
}

pub fn encode_order(order: &ExclusiveDutchOrder) -> Vec<u8> {
    ExclusiveDutchOrder::encode_single(order)
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
