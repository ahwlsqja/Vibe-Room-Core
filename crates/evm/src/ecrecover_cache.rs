//! Concurrent cache for ecrecover (precompile 0x01) results.
//!
//! Ecrecover performs secp256k1 signature recovery — the most expensive
//! state-independent computation in EVM execution. When a transaction is
//! re-executed after an OCC conflict, ecrecover calls produce identical
//! results because they depend only on input data, not blockchain state.
//!
//! `EcrecoverCache` stores `(msg_hash, v, r, s) → Address` mappings in a
//! concurrent [`DashMap`], eliminating redundant signature recovery on
//! re-execution.
//!
//! ## Design
//!
//! - **Full key**: The cache key is `(B256, u8, U256, U256)` representing
//!   `(msg_hash, recovery_id_v, r, s)`. All four fields are required to
//!   avoid false cache hits when different messages share signature
//!   components.
//! - **Thread-safe**: `DashMap` is `Send + Sync`, so multiple EVM workers
//!   can share a single `EcrecoverCache` via `Arc`.
//! - **Block-scoped**: Callers should create a fresh `EcrecoverCache` per
//!   block execution. The struct does not enforce this — it's the caller's
//!   responsibility.
//!
//! ## Observability
//!
//! Call [`EcrecoverCache::stats()`] to retrieve `(hits, misses)` counts.
//! These are monotonically increasing `AtomicU64` counters — safe to read
//! concurrently while workers are still executing.

use std::sync::atomic::{AtomicU64, Ordering};

use alloy_primitives::{Address, B256, U256};
use dashmap::DashMap;

/// Concurrent cache for ecrecover precompile results.
///
/// Keyed by the full ecrecover input parameters `(msg_hash, v, r, s)` to
/// avoid false cache hits. Values are the recovered `Address`.
pub struct EcrecoverCache {
    cache: DashMap<(B256, u8, U256, U256), Address>,
    hits: AtomicU64,
    misses: AtomicU64,
}

impl EcrecoverCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self {
            cache: DashMap::new(),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }

    /// Look up a cached ecrecover result.
    ///
    /// Returns `Some(address)` if the result is cached, `None` otherwise.
    /// Increments the hit counter on cache hit, miss counter on cache miss.
    pub fn lookup(&self, msg_hash: B256, v: u8, r: U256, s: U256) -> Option<Address> {
        let key = (msg_hash, v, r, s);
        match self.cache.get(&key) {
            Some(entry) => {
                self.hits.fetch_add(1, Ordering::Relaxed);
                Some(*entry)
            }
            None => {
                self.misses.fetch_add(1, Ordering::Relaxed);
                None
            }
        }
    }

    /// Insert an ecrecover result into the cache.
    ///
    /// If the key already exists, the value is overwritten (idempotent for
    /// correct callers, since the same inputs always produce the same address).
    pub fn insert(&self, msg_hash: B256, v: u8, r: U256, s: U256, address: Address) {
        let key = (msg_hash, v, r, s);
        self.cache.insert(key, address);
    }

    /// Returns `(hits, misses)` counts.
    ///
    /// Counters use `Relaxed` ordering — suitable for diagnostics, not
    /// synchronization.
    pub fn stats(&self) -> (u64, u64) {
        (
            self.hits.load(Ordering::Relaxed),
            self.misses.load(Ordering::Relaxed),
        )
    }

    /// Returns the number of cached entries.
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Returns `true` if the cache contains no entries.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

impl Default for EcrecoverCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    /// Helper: create a deterministic B256 from a byte value.
    fn hash(b: u8) -> B256 {
        B256::from([b; 32])
    }

    /// Helper: create a deterministic Address from a byte value.
    fn addr(b: u8) -> Address {
        Address::from([b; 20])
    }

    #[test]
    fn cache_miss_then_hit_on_same_key() {
        let cache = EcrecoverCache::new();
        let msg_hash = hash(0xAA);
        let v = 27u8;
        let r = U256::from(111u64);
        let s = U256::from(222u64);
        let address = addr(0x01);

        // First lookup → miss
        assert_eq!(cache.lookup(msg_hash, v, r, s), None);
        assert_eq!(cache.stats(), (0, 1));

        // Insert the result
        cache.insert(msg_hash, v, r, s, address);

        // Second lookup → hit
        assert_eq!(cache.lookup(msg_hash, v, r, s), Some(address));
        assert_eq!(cache.stats(), (1, 1));
    }

    #[test]
    fn cache_miss_on_different_key() {
        let cache = EcrecoverCache::new();
        let address = addr(0x02);

        // Insert with one msg_hash
        cache.insert(hash(0xAA), 27, U256::from(1u64), U256::from(2u64), address);

        // Lookup with different msg_hash → miss (different first field)
        assert_eq!(
            cache.lookup(hash(0xBB), 27, U256::from(1u64), U256::from(2u64)),
            None
        );

        // Lookup with different v → miss (different second field)
        assert_eq!(
            cache.lookup(hash(0xAA), 28, U256::from(1u64), U256::from(2u64)),
            None
        );

        // Lookup with different r → miss (different third field)
        assert_eq!(
            cache.lookup(hash(0xAA), 27, U256::from(99u64), U256::from(2u64)),
            None
        );

        // Lookup with different s → miss (different fourth field)
        assert_eq!(
            cache.lookup(hash(0xAA), 27, U256::from(1u64), U256::from(99u64)),
            None
        );

        // 4 misses total
        assert_eq!(cache.stats(), (0, 4));
    }

    #[test]
    fn stats_counting_accumulates() {
        let cache = EcrecoverCache::new();
        let msg = hash(0xCC);
        let v = 27u8;
        let r = U256::from(10u64);
        let s = U256::from(20u64);
        let address = addr(0x03);

        // 3 misses
        for _ in 0..3 {
            cache.lookup(msg, v, r, s);
        }
        assert_eq!(cache.stats(), (0, 3));

        cache.insert(msg, v, r, s, address);

        // 5 hits
        for _ in 0..5 {
            cache.lookup(msg, v, r, s);
        }
        assert_eq!(cache.stats(), (5, 3));
    }

    #[test]
    fn len_tracks_distinct_entries() {
        let cache = EcrecoverCache::new();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());

        cache.insert(hash(0x01), 27, U256::from(1u64), U256::from(1u64), addr(0x10));
        assert_eq!(cache.len(), 1);

        cache.insert(hash(0x02), 28, U256::from(2u64), U256::from(2u64), addr(0x20));
        assert_eq!(cache.len(), 2);

        // Re-insert same key — len stays 2
        cache.insert(hash(0x01), 27, U256::from(1u64), U256::from(1u64), addr(0x10));
        assert_eq!(cache.len(), 2);
        assert!(!cache.is_empty());
    }

    #[test]
    fn concurrent_insert_and_lookup() {
        let cache = Arc::new(EcrecoverCache::new());
        let threads: Vec<_> = (0..8u8)
            .map(|i| {
                let cache = Arc::clone(&cache);
                std::thread::spawn(move || {
                    let msg = hash(i);
                    let v = 27u8;
                    let r = U256::from(i as u64);
                    let s = U256::from(i as u64 + 100);
                    let address = addr(i);

                    // Insert from this thread
                    cache.insert(msg, v, r, s, address);

                    // Lookup from this thread — should hit
                    let result = cache.lookup(msg, v, r, s);
                    assert_eq!(result, Some(address));
                })
            })
            .collect();

        for t in threads {
            t.join().expect("thread panicked");
        }

        // 8 distinct entries, 8 hits, 0 misses
        assert_eq!(cache.len(), 8);
        assert_eq!(cache.stats().0, 8); // 8 hits
    }

    #[test]
    fn send_sync_static_assertion() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<EcrecoverCache>();
    }
}
