//! MVHashMap — the multi-version concurrent data structure for Block-STM.
//!
//! Stores versioned values keyed by `(LocationKey, TxIndex, Incarnation)`.
//! When a transaction at index N reads a location, the MVHashMap returns the
//! value written by the highest transaction index < N. If that entry is an
//! ESTIMATE marker, the read signals "suspend this transaction."

use std::collections::BTreeMap;

use dashmap::DashMap;

use crate::types::{
    Incarnation, LocationKey, MvReadResult, TxIndex, VersionedValue, WriteValue,
};

/// Concurrent multi-version hash map for Block-STM parallel execution.
///
/// Internally stores `DashMap<LocationKey, BTreeMap<(TxIndex, Incarnation), VersionedValue>>`.
/// The BTreeMap provides ordered access so we can efficiently find the highest
/// version below a given transaction index.
pub struct MVHashMap {
    data: DashMap<LocationKey, BTreeMap<(TxIndex, Incarnation), VersionedValue>>,
}

impl MVHashMap {
    /// Create a new, empty MVHashMap.
    pub fn new() -> Self {
        Self {
            data: DashMap::new(),
        }
    }

    /// Read the value at `location` visible to transaction `reader_tx_index`.
    ///
    /// Finds the highest `(tx_idx, incarnation)` where `tx_idx < reader_tx_index`.
    /// Returns:
    /// - `MvReadResult::Value` if a concrete value is found
    /// - `MvReadResult::Estimate` if the entry is an ESTIMATE marker
    /// - `MvReadResult::NotFound` if no prior transaction wrote to this location
    pub fn read(&self, location: &LocationKey, reader_tx_index: TxIndex) -> MvReadResult {
        let entry = match self.data.get(location) {
            Some(entry) => entry,
            None => return MvReadResult::NotFound,
        };

        let versions = entry.value();

        // Find the highest (tx_idx, incarnation) where tx_idx < reader_tx_index.
        // Using (reader_tx_index, 0) as exclusive upper bound ensures we only see
        // entries with tx_idx strictly less than reader_tx_index.
        match versions.range(..(reader_tx_index, 0)).next_back() {
            Some((&(tx_idx, incarnation), versioned_value)) => match versioned_value {
                VersionedValue::Value(write_value) => {
                    MvReadResult::Value(write_value.clone(), tx_idx, incarnation)
                }
                VersionedValue::Estimate => MvReadResult::Estimate(tx_idx),
            },
            None => MvReadResult::NotFound,
        }
    }

    /// Write a value at the given location for `(tx_index, incarnation)`.
    pub fn write(
        &self,
        location: LocationKey,
        tx_index: TxIndex,
        incarnation: Incarnation,
        value: WriteValue,
    ) {
        self.data
            .entry(location)
            .or_default()
            .insert((tx_index, incarnation), VersionedValue::Value(value));
    }

    /// Mark all entries written by `tx_index` as ESTIMATE across all locations.
    ///
    /// Called when a transaction is about to be re-executed — any concurrent reader
    /// that hits these entries will know to suspend.
    pub fn mark_estimate(&self, tx_index: TxIndex) {
        for mut entry in self.data.iter_mut() {
            let versions = entry.value_mut();
            let keys_to_mark: Vec<(TxIndex, Incarnation)> = versions
                .keys()
                .filter(|(tx, _)| *tx == tx_index)
                .copied()
                .collect();

            for key in keys_to_mark {
                versions.insert(key, VersionedValue::Estimate);
            }
        }
    }

    /// Remove all entries written by `tx_index` across all locations.
    ///
    /// Cleans up after a transaction is aborted or before re-execution
    /// writes fresh values. Empty BTreeMaps are removed from the DashMap.
    pub fn clear(&self, tx_index: TxIndex) {
        // Collect keys of entries that become empty after clearing, so we can
        // remove them outside the iter_mut borrow.
        let mut empty_keys = Vec::new();

        for mut entry in self.data.iter_mut() {
            let versions = entry.value_mut();
            let keys_to_remove: Vec<(TxIndex, Incarnation)> = versions
                .keys()
                .filter(|(tx, _)| *tx == tx_index)
                .copied()
                .collect();

            for key in keys_to_remove {
                versions.remove(&key);
            }

            if versions.is_empty() {
                empty_keys.push(entry.key().clone());
            }
        }

        // Remove empty BTreeMaps from DashMap.
        for key in empty_keys {
            // Re-check emptiness under the entry lock — another thread may have
            // written between iter_mut and here.
            self.data.remove_if(&key, |_, versions| versions.is_empty());
        }
    }

    /// Number of distinct LocationKey entries in the map (for testing/diagnostics).
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the map contains no location entries.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for MVHashMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, B256, U256};
    use std::sync::Arc;

    fn test_addr() -> alloy_primitives::Address {
        address!("0x0000000000000000000000000000000000000001")
    }

    fn storage_loc(slot: u64) -> LocationKey {
        LocationKey::Storage(test_addr(), U256::from(slot))
    }

    // ---- Basic write + read ----

    #[test]
    fn write_then_read_from_higher_tx() {
        let map = MVHashMap::new();
        let loc = storage_loc(0);

        map.write(loc.clone(), 0, 0, WriteValue::Storage(U256::from(42)));

        // tx=1 should see tx=0's value.
        match map.read(&loc, 1) {
            MvReadResult::Value(WriteValue::Storage(v), tx, inc) => {
                assert_eq!(v, U256::from(42));
                assert_eq!(tx, 0);
                assert_eq!(inc, 0);
            }
            other => panic!("expected Value, got {:?}", other),
        }
    }

    #[test]
    fn read_from_same_tx_sees_not_found() {
        let map = MVHashMap::new();
        let loc = storage_loc(0);

        map.write(loc.clone(), 0, 0, WriteValue::Storage(U256::from(42)));

        // tx=0 should NOT see its own write (strict < comparison).
        match map.read(&loc, 0) {
            MvReadResult::NotFound => {}
            other => panic!("expected NotFound, got {:?}", other),
        }
    }

    #[test]
    fn read_empty_map_returns_not_found() {
        let map = MVHashMap::new();
        let loc = storage_loc(0);

        match map.read(&loc, 5) {
            MvReadResult::NotFound => {}
            other => panic!("expected NotFound, got {:?}", other),
        }
    }

    // ---- Version ordering ----

    #[test]
    fn read_finds_highest_tx_below_reader() {
        let map = MVHashMap::new();
        let loc = storage_loc(0);

        map.write(loc.clone(), 0, 0, WriteValue::Storage(U256::from(10)));
        map.write(loc.clone(), 2, 0, WriteValue::Storage(U256::from(20)));

        // tx=3 should see tx=2's value (highest below 3).
        match map.read(&loc, 3) {
            MvReadResult::Value(WriteValue::Storage(v), tx, _) => {
                assert_eq!(v, U256::from(20));
                assert_eq!(tx, 2);
            }
            other => panic!("expected tx=2's value, got {:?}", other),
        }

        // tx=1 should see tx=0's value (highest below 1).
        match map.read(&loc, 1) {
            MvReadResult::Value(WriteValue::Storage(v), tx, _) => {
                assert_eq!(v, U256::from(10));
                assert_eq!(tx, 0);
            }
            other => panic!("expected tx=0's value, got {:?}", other),
        }
    }

    #[test]
    fn higher_incarnation_overwrites_same_tx() {
        let map = MVHashMap::new();
        let loc = storage_loc(0);

        map.write(loc.clone(), 0, 0, WriteValue::Storage(U256::from(10)));
        map.write(loc.clone(), 0, 1, WriteValue::Storage(U256::from(20)));

        // Both (0,0) and (0,1) exist. The BTreeMap orders (0,0) < (0,1),
        // so `range(..(1, 0)).next_back()` returns (0, 1) — the latest incarnation.
        match map.read(&loc, 1) {
            MvReadResult::Value(WriteValue::Storage(v), tx, inc) => {
                assert_eq!(v, U256::from(20));
                assert_eq!(tx, 0);
                assert_eq!(inc, 1);
            }
            other => panic!("expected inc=1 value, got {:?}", other),
        }
    }

    // ---- ESTIMATE markers ----

    #[test]
    fn mark_estimate_replaces_value() {
        let map = MVHashMap::new();
        let loc = storage_loc(0);

        map.write(loc.clone(), 0, 0, WriteValue::Storage(U256::from(42)));
        map.mark_estimate(0);

        match map.read(&loc, 1) {
            MvReadResult::Estimate(tx) => assert_eq!(tx, 0),
            other => panic!("expected Estimate, got {:?}", other),
        }
    }

    #[test]
    fn mark_estimate_does_not_affect_other_txs() {
        let map = MVHashMap::new();
        let loc = storage_loc(0);

        map.write(loc.clone(), 0, 0, WriteValue::Storage(U256::from(10)));
        map.write(loc.clone(), 1, 0, WriteValue::Storage(U256::from(20)));
        map.mark_estimate(0);

        // tx=2 should see tx=1's value (still a concrete value).
        match map.read(&loc, 2) {
            MvReadResult::Value(WriteValue::Storage(v), tx, _) => {
                assert_eq!(v, U256::from(20));
                assert_eq!(tx, 1);
            }
            other => panic!("expected tx=1's value, got {:?}", other),
        }

        // tx=1 should see tx=0's ESTIMATE.
        match map.read(&loc, 1) {
            MvReadResult::Estimate(tx) => assert_eq!(tx, 0),
            other => panic!("expected Estimate for tx=0, got {:?}", other),
        }
    }

    // ---- Clear ----

    #[test]
    fn clear_removes_all_entries_for_tx() {
        let map = MVHashMap::new();
        let loc_a = storage_loc(0);
        let loc_b = storage_loc(1);

        map.write(loc_a.clone(), 0, 0, WriteValue::Storage(U256::from(10)));
        map.write(loc_b.clone(), 0, 0, WriteValue::Storage(U256::from(20)));
        map.write(loc_a.clone(), 1, 0, WriteValue::Storage(U256::from(30)));

        map.clear(0);

        // tx=0's entries are gone.
        match map.read(&loc_a, 1) {
            MvReadResult::NotFound => {}
            other => panic!("expected NotFound after clear, got {:?}", other),
        }
        match map.read(&loc_b, 1) {
            MvReadResult::NotFound => {}
            other => panic!("expected NotFound after clear, got {:?}", other),
        }

        // tx=1's entry is still present.
        match map.read(&loc_a, 2) {
            MvReadResult::Value(WriteValue::Storage(v), tx, _) => {
                assert_eq!(v, U256::from(30));
                assert_eq!(tx, 1);
            }
            other => panic!("expected tx=1's value, got {:?}", other),
        }
    }

    #[test]
    fn clear_removes_empty_btreemaps() {
        let map = MVHashMap::new();
        let loc = storage_loc(0);

        map.write(loc.clone(), 0, 0, WriteValue::Storage(U256::from(10)));
        assert_eq!(map.len(), 1);

        map.clear(0);
        // The empty BTreeMap should be removed from DashMap.
        assert_eq!(map.len(), 0);
    }

    // ---- Concurrent access ----

    #[test]
    fn concurrent_writes_to_different_locations() {
        let map = Arc::new(MVHashMap::new());
        let mut handles = Vec::new();

        for i in 0..4u32 {
            let map = Arc::clone(&map);
            handles.push(std::thread::spawn(move || {
                let loc = LocationKey::Balance(
                    alloy_primitives::Address::with_last_byte(i as u8),
                );
                map.write(loc, i, 0, WriteValue::Balance(U256::from(i * 100)));
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        // Verify each thread's write is visible.
        for i in 0..4u32 {
            let loc = LocationKey::Balance(
                alloy_primitives::Address::with_last_byte(i as u8),
            );
            match map.read(&loc, i + 1) {
                MvReadResult::Value(WriteValue::Balance(v), tx, _) => {
                    assert_eq!(v, U256::from(i * 100));
                    assert_eq!(tx, i);
                }
                other => panic!("thread {} write not found: {:?}", i, other),
            }
        }
    }

    #[test]
    fn concurrent_write_and_read() {
        let map = Arc::new(MVHashMap::new());

        // Pre-populate some values.
        for i in 0..10u32 {
            let loc = storage_loc(i as u64);
            map.write(loc, i, 0, WriteValue::Storage(U256::from(i)));
        }

        let mut handles = Vec::new();

        // Writers: overwrite with new incarnation.
        for i in 0..10u32 {
            let map = Arc::clone(&map);
            handles.push(std::thread::spawn(move || {
                let loc = storage_loc(i as u64);
                map.write(loc, i, 1, WriteValue::Storage(U256::from(i * 10)));
            }));
        }

        // Readers: read from tx=10 (sees all prior txs).
        for i in 0..10u32 {
            let map = Arc::clone(&map);
            handles.push(std::thread::spawn(move || {
                let loc = storage_loc(i as u64);
                // Read may see either incarnation 0 or 1 — both are valid
                // during concurrent execution. Just verify we get a Value.
                match map.read(&loc, i + 1) {
                    MvReadResult::Value(WriteValue::Storage(_), tx, _) => {
                        assert_eq!(tx, i);
                    }
                    other => panic!("expected Value for slot {}, got {:?}", i, other),
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
    }

    // ---- LocationKey variants ----

    #[test]
    fn all_location_key_variants_work_independently() {
        let map = MVHashMap::new();
        let addr = test_addr();

        let storage = LocationKey::Storage(addr, U256::from(1));
        let balance = LocationKey::Balance(addr);
        let nonce = LocationKey::Nonce(addr);
        let code_hash = LocationKey::CodeHash(addr);

        map.write(storage.clone(), 0, 0, WriteValue::Storage(U256::from(100)));
        map.write(balance.clone(), 0, 0, WriteValue::Balance(U256::from(200)));
        map.write(nonce.clone(), 0, 0, WriteValue::Nonce(5));
        map.write(code_hash.clone(), 0, 0, WriteValue::CodeHash(B256::ZERO));

        // Each location is independent.
        assert_eq!(map.len(), 4);

        match map.read(&storage, 1) {
            MvReadResult::Value(WriteValue::Storage(v), _, _) => {
                assert_eq!(v, U256::from(100));
            }
            other => panic!("expected Storage value, got {:?}", other),
        }

        match map.read(&balance, 1) {
            MvReadResult::Value(WriteValue::Balance(v), _, _) => {
                assert_eq!(v, U256::from(200));
            }
            other => panic!("expected Balance value, got {:?}", other),
        }

        match map.read(&nonce, 1) {
            MvReadResult::Value(WriteValue::Nonce(v), _, _) => {
                assert_eq!(v, 5);
            }
            other => panic!("expected Nonce value, got {:?}", other),
        }

        match map.read(&code_hash, 1) {
            MvReadResult::Value(WriteValue::CodeHash(v), _, _) => {
                assert_eq!(v, B256::ZERO);
            }
            other => panic!("expected CodeHash value, got {:?}", other),
        }
    }

    #[test]
    fn mark_estimate_across_multiple_locations() {
        let map = MVHashMap::new();
        let loc_a = storage_loc(0);
        let loc_b = LocationKey::Balance(test_addr());

        map.write(loc_a.clone(), 0, 0, WriteValue::Storage(U256::from(10)));
        map.write(loc_b.clone(), 0, 0, WriteValue::Balance(U256::from(20)));

        map.mark_estimate(0);

        // Both locations for tx=0 should be ESTIMATE.
        match map.read(&loc_a, 1) {
            MvReadResult::Estimate(0) => {}
            other => panic!("expected Estimate for loc_a, got {:?}", other),
        }
        match map.read(&loc_b, 1) {
            MvReadResult::Estimate(0) => {}
            other => panic!("expected Estimate for loc_b, got {:?}", other),
        }
    }

    #[test]
    fn len_and_is_empty() {
        let map = MVHashMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);

        map.write(storage_loc(0), 0, 0, WriteValue::Storage(U256::ZERO));
        assert!(!map.is_empty());
        assert_eq!(map.len(), 1);

        map.write(storage_loc(1), 0, 0, WriteValue::Storage(U256::ZERO));
        assert_eq!(map.len(), 2);
    }
}
