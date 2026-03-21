//! Lazy update tracking for Block-STM parallel execution.
//!
//! The [`LazyBeneficiaryTracker`] prevents false conflicts on the coinbase
//! (block beneficiary) address. Without lazy tracking, every transaction would
//! write its gas fee to the coinbase balance in the MVHashMap, causing every
//! pair of transactions to conflict on that location — destroying parallelism.
//!
//! Instead, gas fee credits are accumulated in a side-channel `BTreeMap` and
//! applied during the merge phase (S05) after all transactions have been
//! validated. The MVHashMap never sees coinbase balance writes during execution.

use std::collections::BTreeMap;

use alloy_primitives::U256;

use crate::types::TxIndex;

/// Accumulates gas fee credits for the block beneficiary without writing
/// to the MVHashMap.
///
/// Each transaction records its gas fee here rather than writing to
/// `LocationKey::Balance(coinbase)` in the MVHashMap. During the merge
/// phase, all accumulated fees are applied to the coinbase balance in
/// one atomic update.
///
/// Fees are stored in a `BTreeMap` for deterministic iteration order
/// (matching the rest of the Block-STM data structures).
#[derive(Debug)]
pub struct LazyBeneficiaryTracker {
    /// Maps tx_index → accumulated gas fee for that transaction.
    fees: BTreeMap<TxIndex, U256>,
}

impl LazyBeneficiaryTracker {
    /// Create a new, empty tracker.
    pub fn new() -> Self {
        Self {
            fees: BTreeMap::new(),
        }
    }

    /// Record a gas fee for the given transaction.
    ///
    /// Accumulates: if called multiple times for the same `tx_index`,
    /// the fees are summed. This supports scenarios where a transaction
    /// produces multiple fee components (e.g., base fee + priority fee).
    pub fn record_gas_fee(&mut self, tx_index: TxIndex, gas_fee: U256) {
        let entry = self.fees.entry(tx_index).or_insert(U256::ZERO);
        *entry += gas_fee;
    }

    /// Get the accumulated fee for a specific transaction.
    ///
    /// Returns `None` if no fee has been recorded for this tx_index.
    pub fn get_fee(&self, tx_index: TxIndex) -> Option<U256> {
        self.fees.get(&tx_index).copied()
    }

    /// Get a reference to the full accumulated fees map.
    ///
    /// Used during the merge phase (S05) to apply all fees to the
    /// coinbase balance in one pass.
    pub fn get_accumulated_fees(&self) -> &BTreeMap<TxIndex, U256> {
        &self.fees
    }

    /// Compute the total of all accumulated fees across all transactions.
    pub fn total_fees(&self) -> U256 {
        self.fees.values().fold(U256::ZERO, |acc, fee| acc + *fee)
    }

    /// Clear the fee record for a specific transaction.
    ///
    /// Called when a transaction is aborted or about to be re-executed —
    /// the new execution will record fresh fees.
    pub fn clear_tx(&mut self, tx_index: TxIndex) {
        self.fees.remove(&tx_index);
    }

    /// Clear all accumulated fees.
    ///
    /// Called when starting a new block or resetting the tracker.
    pub fn clear_all(&mut self) {
        self.fees.clear();
    }

    /// Number of transactions with recorded fees.
    pub fn len(&self) -> usize {
        self.fees.len()
    }

    /// Returns `true` if no fees have been recorded.
    pub fn is_empty(&self) -> bool {
        self.fees.is_empty()
    }
}

impl Default for LazyBeneficiaryTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tracker_is_empty() {
        let tracker = LazyBeneficiaryTracker::new();
        assert!(tracker.is_empty());
        assert_eq!(tracker.len(), 0);
        assert_eq!(tracker.total_fees(), U256::ZERO);
    }

    #[test]
    fn record_single_fee() {
        let mut tracker = LazyBeneficiaryTracker::new();
        tracker.record_gas_fee(0, U256::from(1000));

        assert_eq!(tracker.len(), 1);
        assert_eq!(tracker.get_fee(0), Some(U256::from(1000)));
        assert_eq!(tracker.total_fees(), U256::from(1000));
    }

    #[test]
    fn record_accumulates_fees_for_same_tx() {
        let mut tracker = LazyBeneficiaryTracker::new();
        tracker.record_gas_fee(0, U256::from(500));
        tracker.record_gas_fee(0, U256::from(300));

        // Accumulates: 500 + 300 = 800
        assert_eq!(tracker.len(), 1);
        assert_eq!(tracker.get_fee(0), Some(U256::from(800)));
        assert_eq!(tracker.total_fees(), U256::from(800));
    }

    #[test]
    fn record_multiple_txs() {
        let mut tracker = LazyBeneficiaryTracker::new();
        tracker.record_gas_fee(0, U256::from(100));
        tracker.record_gas_fee(1, U256::from(200));
        tracker.record_gas_fee(2, U256::from(300));

        assert_eq!(tracker.len(), 3);
        assert_eq!(tracker.get_fee(0), Some(U256::from(100)));
        assert_eq!(tracker.get_fee(1), Some(U256::from(200)));
        assert_eq!(tracker.get_fee(2), Some(U256::from(300)));
        assert_eq!(tracker.total_fees(), U256::from(600));
    }

    #[test]
    fn get_fee_returns_none_for_unrecorded_tx() {
        let tracker = LazyBeneficiaryTracker::new();
        assert_eq!(tracker.get_fee(42), None);
    }

    #[test]
    fn get_accumulated_fees_returns_ordered_map() {
        let mut tracker = LazyBeneficiaryTracker::new();
        // Insert in reverse order — BTreeMap should iterate in ascending order.
        tracker.record_gas_fee(3, U256::from(30));
        tracker.record_gas_fee(1, U256::from(10));
        tracker.record_gas_fee(2, U256::from(20));

        let fees = tracker.get_accumulated_fees();
        let keys: Vec<TxIndex> = fees.keys().copied().collect();
        assert_eq!(keys, vec![1, 2, 3], "BTreeMap should iterate in order");
    }

    #[test]
    fn clear_tx_removes_single_entry() {
        let mut tracker = LazyBeneficiaryTracker::new();
        tracker.record_gas_fee(0, U256::from(100));
        tracker.record_gas_fee(1, U256::from(200));

        tracker.clear_tx(0);

        assert_eq!(tracker.len(), 1);
        assert_eq!(tracker.get_fee(0), None);
        assert_eq!(tracker.get_fee(1), Some(U256::from(200)));
        assert_eq!(tracker.total_fees(), U256::from(200));
    }

    #[test]
    fn clear_tx_no_op_for_missing() {
        let mut tracker = LazyBeneficiaryTracker::new();
        tracker.record_gas_fee(0, U256::from(100));
        tracker.clear_tx(99); // no-op
        assert_eq!(tracker.len(), 1);
    }

    #[test]
    fn clear_all_removes_everything() {
        let mut tracker = LazyBeneficiaryTracker::new();
        tracker.record_gas_fee(0, U256::from(100));
        tracker.record_gas_fee(1, U256::from(200));
        tracker.record_gas_fee(2, U256::from(300));

        tracker.clear_all();

        assert!(tracker.is_empty());
        assert_eq!(tracker.len(), 0);
        assert_eq!(tracker.total_fees(), U256::ZERO);
    }

    #[test]
    fn clear_tx_then_re_record() {
        let mut tracker = LazyBeneficiaryTracker::new();
        tracker.record_gas_fee(0, U256::from(100));
        tracker.clear_tx(0);
        tracker.record_gas_fee(0, U256::from(999));

        // Re-recorded with fresh value (not accumulated with old).
        assert_eq!(tracker.get_fee(0), Some(U256::from(999)));
    }

    #[test]
    fn default_trait_creates_empty() {
        let tracker = LazyBeneficiaryTracker::default();
        assert!(tracker.is_empty());
    }
}
