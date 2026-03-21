use alloy_primitives::{Address, Bytes, U256};
use serde::{Deserialize, Serialize};

/// Represents an Ethereum transaction for EVM execution.
///
/// `to` is `None` for contract creation transactions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction sender address.
    pub sender: Address,
    /// Recipient address. `None` indicates contract creation.
    pub to: Option<Address>,
    /// Value transferred in wei.
    pub value: U256,
    /// Transaction input data (calldata or init code).
    pub data: Bytes,
    /// Maximum gas units this transaction can consume.
    pub gas_limit: u64,
    /// Sender's transaction sequence number.
    pub nonce: u64,
    /// Gas price in wei per gas unit.
    pub gas_price: U256,
}

impl Transaction {
    /// Returns `true` if this is a contract creation transaction.
    pub fn is_create(&self) -> bool {
        self.to.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_transfer_transaction() {
        let tx = Transaction {
            sender: Address::ZERO,
            to: Some(Address::with_last_byte(1)),
            value: U256::from(1000),
            data: Bytes::new(),
            gas_limit: 21_000,
            nonce: 0,
            gas_price: U256::from(1_000_000_000u64), // 1 gwei
        };
        assert!(!tx.is_create());
        assert_eq!(tx.gas_limit, 21_000);
        assert_eq!(tx.value, U256::from(1000));
    }

    #[test]
    fn test_contract_creation_transaction() {
        let tx = Transaction {
            sender: Address::ZERO,
            to: None,
            value: U256::ZERO,
            data: Bytes::from(vec![0x60, 0x00, 0x60, 0x00, 0xf3]), // minimal contract bytecode
            gas_limit: 100_000,
            nonce: 1,
            gas_price: U256::from(1_000_000_000u64),
        };
        assert!(tx.is_create());
        assert_eq!(tx.nonce, 1);
    }

    #[test]
    fn test_transaction_clone_and_eq() {
        let tx = Transaction {
            sender: Address::with_last_byte(0xAA),
            to: Some(Address::with_last_byte(0xBB)),
            value: U256::from(42),
            data: Bytes::from(vec![1, 2, 3]),
            gas_limit: 50_000,
            nonce: 5,
            gas_price: U256::from(2_000_000_000u64),
        };
        let tx2 = tx.clone();
        assert_eq!(tx, tx2);
    }
}
