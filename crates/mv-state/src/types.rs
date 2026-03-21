//! Core types for multi-version state tracking in Block-STM parallel execution.
//!
//! These types define the granularity of state access tracking (LocationKey),
//! versioned storage entries (VersionedValue), and read results (MvReadResult).

use alloy_primitives::{Address, B256, U256};

/// Index of a transaction within a block (0-based ordering).
pub type TxIndex = u32;

/// Incarnation counter — incremented each time a transaction is re-executed
/// after validation failure.
pub type Incarnation = u32;

/// A key identifying a specific state location.
///
/// Each variant represents a distinct granularity of state access. Two transactions
/// that access the same location (e.g. same storage slot) may conflict, but accessing
/// different locations (e.g. one reads balance, another reads nonce) does not conflict.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LocationKey {
    /// A storage slot: (contract address, slot index).
    Storage(Address, U256),
    /// An account balance.
    Balance(Address),
    /// An account nonce.
    Nonce(Address),
    /// An account code hash.
    CodeHash(Address),
}

/// A versioned entry in the MVHashMap.
///
/// `Value` holds the actual written data. `Estimate` is a placeholder indicating
/// that the transaction that previously wrote this location is being re-executed,
/// and any reader should suspend until the re-execution completes.
#[derive(Debug, Clone)]
pub enum VersionedValue {
    /// A concrete value written by a transaction.
    Value(WriteValue),
    /// Placeholder: the writing transaction is being re-executed.
    /// Readers must suspend (return `MvReadResult::Estimate`).
    Estimate,
}

/// The concrete value written to a state location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteValue {
    /// A storage slot value.
    Storage(U256),
    /// An account balance.
    Balance(U256),
    /// An account nonce.
    Nonce(u64),
    /// An account code hash.
    CodeHash(B256),
    /// Full account info (used when multiple fields are written together).
    AccountInfo {
        balance: U256,
        nonce: u64,
        code_hash: B256,
    },
}

/// Where a read value originated from, recorded in the ReadSet for validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadOrigin {
    /// Value was read from the MVHashMap (written by a prior transaction).
    MvHashMap {
        tx_index: TxIndex,
        incarnation: Incarnation,
    },
    /// Value was read from the base storage (no prior transaction wrote it).
    Storage,
    /// The location had no value in either MVHashMap or base storage.
    NotFound,
}

/// The result of reading a location from the MVHashMap.
#[derive(Debug, Clone)]
pub enum MvReadResult {
    /// A concrete value was found, written by `(TxIndex, Incarnation)`.
    Value(WriteValue, TxIndex, Incarnation),
    /// The location was written by `TxIndex` but is currently marked as ESTIMATE
    /// (that transaction is being re-executed). The reader should suspend.
    Estimate(TxIndex),
    /// No prior transaction wrote to this location in the MVHashMap.
    NotFound,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::address;

    #[test]
    fn location_key_ord_is_deterministic() {
        let a = LocationKey::Balance(address!("0x0000000000000000000000000000000000000001"));
        let b = LocationKey::Balance(address!("0x0000000000000000000000000000000000000002"));
        let c = LocationKey::Nonce(address!("0x0000000000000000000000000000000000000001"));
        // Balance < Nonce in enum variant order (Storage < Balance < Nonce < CodeHash)
        assert!(a < b, "same variant, lower address should be less");
        assert!(a < c, "Balance variant should be less than Nonce variant");
    }

    #[test]
    fn location_key_storage_variant_uses_both_fields() {
        let addr = address!("0x0000000000000000000000000000000000000001");
        let slot_a = LocationKey::Storage(addr, U256::from(1));
        let slot_b = LocationKey::Storage(addr, U256::from(2));
        assert_ne!(slot_a, slot_b);
    }

    #[test]
    fn versioned_value_estimate_is_distinct() {
        let v = VersionedValue::Value(WriteValue::Balance(U256::from(100)));
        let e = VersionedValue::Estimate;
        // They are structurally distinct — pattern matching differentiates them.
        match (&v, &e) {
            (VersionedValue::Value(_), VersionedValue::Estimate) => {} // expected
            _ => panic!("Estimate and Value must be distinct variants"),
        }
    }

    #[test]
    fn write_value_variants_are_all_constructible() {
        let _ = WriteValue::Storage(U256::from(42));
        let _ = WriteValue::Balance(U256::from(1000));
        let _ = WriteValue::Nonce(7);
        let _ = WriteValue::CodeHash(B256::ZERO);
        let _ = WriteValue::AccountInfo {
            balance: U256::from(500),
            nonce: 3,
            code_hash: B256::ZERO,
        };
    }
}
