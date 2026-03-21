use alloy_primitives::{Address, U256};
use serde::{Deserialize, Serialize};

/// Block environment parameters for EVM execution.
///
/// Contains the block-level context that the EVM needs during transaction execution
/// (e.g., for COINBASE, TIMESTAMP, NUMBER, BASEFEE opcodes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockEnv {
    /// Block number.
    pub number: u64,
    /// Block beneficiary (miner/validator) address.
    pub coinbase: Address,
    /// Block timestamp (seconds since Unix epoch).
    pub timestamp: u64,
    /// Block gas limit.
    pub gas_limit: u64,
    /// Base fee per gas (EIP-1559).
    pub base_fee: U256,
    /// Block difficulty (legacy PoW) / prevrandao (PoS).
    pub difficulty: U256,
}

impl Default for BlockEnv {
    fn default() -> Self {
        Self {
            number: 0,
            coinbase: Address::ZERO,
            timestamp: 0,
            gas_limit: 30_000_000,
            base_fee: U256::ZERO,
            difficulty: U256::ZERO,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_env_default() {
        let env = BlockEnv::default();
        assert_eq!(env.number, 0);
        assert_eq!(env.coinbase, Address::ZERO);
        assert_eq!(env.gas_limit, 30_000_000);
        assert_eq!(env.base_fee, U256::ZERO);
    }

    #[test]
    fn test_block_env_custom() {
        let env = BlockEnv {
            number: 100,
            coinbase: Address::with_last_byte(0xFF),
            timestamp: 1_700_000_000,
            gas_limit: 15_000_000,
            base_fee: U256::from(7_000_000_000u64), // 7 gwei
            difficulty: U256::ZERO,
        };
        assert_eq!(env.number, 100);
        assert_eq!(env.timestamp, 1_700_000_000);
        assert_eq!(env.gas_limit, 15_000_000);
    }
}
