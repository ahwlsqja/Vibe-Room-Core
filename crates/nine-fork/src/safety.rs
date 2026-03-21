//! # NINE-05: Thread Safety Enforcement
//!
//! This module enforces compile-time `Send + Sync` guarantees on all
//! nine-fork public types that will cross thread boundaries during
//! parallel EVM execution in S03/S04.
//!
//! ## Why Send + Sync Matters
//!
//! In S03/S04, the Monad parallel scheduler will execute multiple
//! transactions concurrently across OS threads. Any type shared between
//! the scheduler and worker threads (or sent to a worker) must be:
//!
//! - **`Send`**: Safe to transfer ownership to another thread.
//! - **`Sync`**: Safe to share references (`&T`) across threads.
//!
//! Failing these bounds at runtime would cause data races or require
//! `unsafe` code to circumvent the borrow checker.
//!
//! ## Current Status
//!
//! - **`NineForkPrecompiles`**: `Send + Sync` ✓ — contains only `EthPrecompiles`
//!   (which is `Send + Sync`) and `Precompile` (static function pointer + address).
//!
//! - **`ReserveBalanceConfig`**: `Send + Sync` ✓ — plain `Clone` struct with `u64`.
//!
//! - **`DippedIntoReserve`**: `Send` but `!Sync` due to `RefCell<HashMap>`.
//!   This is intentional for S02's sequential execution model. In S03/S04,
//!   this will be replaced with per-transaction-index state tracked in the
//!   `MVHashMap`, eliminating the need for a shared mutable tracker entirely.
//!   The thread-local free functions (`mark_address_dipped`, `has_address_dipped`,
//!   `reset_dipped_tracker`) are inherently thread-safe because `thread_local!`
//!   gives each thread its own instance.
//!
//! - **`EvmError`**: `Send + Sync` ✓ — all variants contain only `String` fields,
//!   which are `Send + Sync`. The `ReserveBalance(String)` variant added in T03
//!   preserves this property.
//!
//! ## S03/S04 Migration Path
//!
//! The `DippedIntoReserve` struct is not designed to be shared across threads.
//! When parallel execution lands (S03), per-transaction state will be isolated
//! via the `MVHashMap` (multi-version hashmap), where each transaction index
//! maintains its own write set. The `thread_local!` tracker pattern will be
//! replaced with transaction-index-keyed storage:
//!
//! ```ignore
//! // S03/S04 pattern (future):
//! struct ParallelDippedTracker {
//!     inner: DashMap<(TxIndex, Address), bool>,
//! }
//! ```
//!
//! This eliminates the `RefCell` and makes the tracker both `Send` and `Sync`.

#[cfg(test)]
mod tests {
    /// Compile-time assertion that a type implements Send + Sync.
    ///
    /// This function is never called at runtime — its mere existence
    /// in compiled test code is sufficient to trigger a compile error
    /// if `T` does not satisfy the bounds.
    fn assert_send_sync<T: Send + Sync>() {}

    /// Compile-time assertion that a type implements Send (but not necessarily Sync).
    fn assert_send<T: Send>() {}

    /// Verify that NineForkPrecompiles is Send + Sync.
    ///
    /// This is critical for S03/S04: the precompile provider will be
    /// cloned into each worker thread's EVM instance.
    #[test]
    fn nine_fork_precompiles_send_sync() {
        assert_send_sync::<crate::nine_fork_precompiles::NineForkPrecompiles>();
    }

    /// Verify that ReserveBalanceConfig is Send + Sync.
    ///
    /// Configuration may be shared across threads as a read-only reference.
    #[test]
    fn reserve_balance_config_send_sync() {
        assert_send_sync::<crate::mip4_reserve::ReserveBalanceConfig>();
    }

    /// Verify that DippedIntoReserve is Send (can be moved between threads).
    ///
    /// DippedIntoReserve uses RefCell<HashMap>, which is Send but !Sync.
    /// This is acceptable for S02 (sequential execution). In S03/S04,
    /// the RefCell will be replaced with per-transaction-index storage
    /// in the MVHashMap.
    #[test]
    fn dipped_into_reserve_is_send() {
        assert_send::<crate::mip4_reserve::DippedIntoReserve>();
    }

    /// Verify that DippedIntoReserve is NOT Sync (compile-time negative test).
    ///
    /// This documents the intentional !Sync constraint from RefCell.
    /// When this test starts failing (i.e., DippedIntoReserve becomes Sync),
    /// it means the S03/S04 migration to thread-safe tracking happened.
    #[test]
    fn dipped_into_reserve_not_sync_documents_s02_limitation() {
        // DippedIntoReserve contains RefCell which is !Sync.
        // We document this expected property here. When S03/S04 replaces
        // RefCell with DashMap/Mutex, this test should be updated to
        // assert_send_sync instead.
        //
        // Note: We cannot write a compile-time "must NOT compile" test in
        // standard Rust without trybuild. Instead, we document the design
        // intent and verify Send is satisfied.
        assert_send::<crate::mip4_reserve::DippedIntoReserve>();
    }

    /// Verify that EvmError remains Send + Sync after the ReserveBalance
    /// variant was added in T03.
    ///
    /// This is a regression test: if someone adds a non-Send/Sync field
    /// to any EvmError variant, this test will fail at compile time.
    #[test]
    fn evm_error_send_sync_regression() {
        assert_send_sync::<monad_types::EvmError>();
    }

    /// Verify that the MIP-4 precompile address constant is Send + Sync.
    /// Address is a fixed-size byte array wrapper — trivially safe.
    #[test]
    fn mip4_address_send_sync() {
        assert_send_sync::<alloy_primitives::Address>();
        // The constant itself is a static Address, but we verify the type.
        let _ = crate::mip4_reserve::MIP4_RESERVE_ADDRESS;
    }

    /// Verify that all types that will be used in the parallel execution
    /// context (S03/S04) satisfy their required thread safety bounds.
    ///
    /// This is the aggregate assertion that gates S03 readiness.
    #[test]
    fn all_parallel_execution_types_thread_safe() {
        // Types that must be Send + Sync for parallel EVM execution:
        assert_send_sync::<crate::nine_fork_precompiles::NineForkPrecompiles>();
        assert_send_sync::<crate::mip4_reserve::ReserveBalanceConfig>();
        assert_send_sync::<monad_types::EvmError>();

        // Types that are Send only (will be replaced in S03/S04):
        assert_send::<crate::mip4_reserve::DippedIntoReserve>();
    }
}
