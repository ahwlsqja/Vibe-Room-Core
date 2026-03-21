use alloy_primitives::{Address, Bytes, B256};
use serde::{Deserialize, Serialize};

/// Result of EVM transaction execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionResult {
    /// Transaction executed successfully.
    Success {
        /// Gas units consumed.
        gas_used: u64,
        /// Return data from the transaction.
        output: Bytes,
        /// Logs emitted during execution.
        logs: Vec<Log>,
    },
    /// Transaction reverted (REVERT opcode).
    Revert {
        /// Gas units consumed before revert.
        gas_used: u64,
        /// Revert reason data.
        output: Bytes,
    },
    /// Transaction halted due to an exceptional condition (out of gas, invalid opcode, etc.).
    Halt {
        /// Gas units consumed before halt.
        gas_used: u64,
        /// Human-readable reason for the halt.
        reason: String,
    },
}

impl ExecutionResult {
    /// Returns the gas consumed by this execution.
    pub fn gas_used(&self) -> u64 {
        match self {
            Self::Success { gas_used, .. } => *gas_used,
            Self::Revert { gas_used, .. } => *gas_used,
            Self::Halt { gas_used, .. } => *gas_used,
        }
    }

    /// Returns `true` if the execution was successful.
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }
}

/// An Ethereum log entry emitted by the LOG0..LOG4 opcodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Log {
    /// Address of the contract that emitted the log.
    pub address: Address,
    /// Indexed log topics (0 to 4).
    pub topics: Vec<B256>,
    /// Non-indexed log data.
    pub data: Bytes,
}

/// Transaction receipt summarizing execution outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Receipt {
    /// Whether the transaction executed successfully.
    pub success: bool,
    /// Cumulative gas used in the block up to and including this transaction.
    pub cumulative_gas_used: u64,
    /// Logs emitted during this transaction.
    pub logs: Vec<Log>,
    /// Contract address created (if this was a CREATE transaction).
    pub contract_address: Option<Address>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_result_success() {
        let result = ExecutionResult::Success {
            gas_used: 21_000,
            output: Bytes::new(),
            logs: vec![],
        };
        assert!(result.is_success());
        assert_eq!(result.gas_used(), 21_000);
    }

    #[test]
    fn test_execution_result_revert() {
        let result = ExecutionResult::Revert {
            gas_used: 15_000,
            output: Bytes::from(vec![0x08, 0xc3, 0x79, 0xa0]), // Error(string) selector
        };
        assert!(!result.is_success());
        assert_eq!(result.gas_used(), 15_000);
    }

    #[test]
    fn test_execution_result_halt() {
        let result = ExecutionResult::Halt {
            gas_used: 100_000,
            reason: "OutOfGas".to_string(),
        };
        assert!(!result.is_success());
        assert_eq!(result.gas_used(), 100_000);
    }

    #[test]
    fn test_log_creation() {
        let log = Log {
            address: Address::with_last_byte(0x01),
            topics: vec![B256::ZERO],
            data: Bytes::from(vec![0xAB, 0xCD]),
        };
        assert_eq!(log.topics.len(), 1);
        assert_eq!(log.data.len(), 2);
    }

    #[test]
    fn test_receipt_creation() {
        let receipt = Receipt {
            success: true,
            cumulative_gas_used: 21_000,
            logs: vec![],
            contract_address: None,
        };
        assert!(receipt.success);
        assert_eq!(receipt.cumulative_gas_used, 21_000);
        assert!(receipt.contract_address.is_none());
    }

    #[test]
    fn test_receipt_with_contract_address() {
        let receipt = Receipt {
            success: true,
            cumulative_gas_used: 53_000,
            logs: vec![],
            contract_address: Some(Address::with_last_byte(0xCC)),
        };
        assert!(receipt.contract_address.is_some());
    }
}
