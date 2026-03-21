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
//! - **`DippedIntoReserve`**: `Send + Sync` ✓ — migrated from `RefCell<HashMap>`
//!   to `Mutex<HashMap<(u32, Address), bool>>` in S03. Supports per-transaction-index
//!   isolation for parallel execution. Thread-local free functions remain for
//!   backward compatibility.
//!
//! - **`EvmError`**: `Send + Sync` ✓ — all variants contain only `String` fields,
//!   which are `Send + Sync`. The `ReserveBalance(String)` variant added in T03
//!   preserves this property.

#[cfg(test)]
mod tests {
    /// Compile-time assertion that a type implements Send + Sync.
    ///
    /// This function is never called at runtime — its mere existence
    /// in compiled test code is sufficient to trigger a compile error
    /// if `T` does not satisfy the bounds.
    fn assert_send_sync<T: Send + Sync>() {}

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

    /// Verify that DippedIntoReserve is Send + Sync after S03 migration.
    ///
    /// DippedIntoReserve was migrated from `RefCell<HashMap>` (Send, !Sync)
    /// to `Mutex<HashMap<(u32, Address), bool>>` (Send + Sync) in S03.
    /// This enables safe sharing across worker threads in parallel execution.
    #[test]
    fn dipped_into_reserve_is_send() {
        assert_send_sync::<crate::mip4_reserve::DippedIntoReserve>();
    }

    /// Verify that DippedIntoReserve is now Sync after S03 migration.
    ///
    /// Previously documented the !Sync limitation from RefCell. Now that
    /// S03 replaced RefCell with Mutex, DippedIntoReserve is fully
    /// thread-safe (Send + Sync).
    #[test]
    fn dipped_into_reserve_not_sync_documents_s02_limitation() {
        // S03 migration complete: DippedIntoReserve now uses Mutex<HashMap>
        // instead of RefCell<HashMap>, making it both Send and Sync.
        assert_send_sync::<crate::mip4_reserve::DippedIntoReserve>();
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
        // All types are now Send + Sync for parallel EVM execution:
        assert_send_sync::<crate::nine_fork_precompiles::NineForkPrecompiles>();
        assert_send_sync::<crate::mip4_reserve::ReserveBalanceConfig>();
        assert_send_sync::<monad_types::EvmError>();
        assert_send_sync::<crate::mip4_reserve::DippedIntoReserve>();
    }
}
