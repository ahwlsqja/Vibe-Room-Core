use monad_types::{AccountInfo, EvmError, Address, Bytes, U256, B256};

/// Core abstraction for reading blockchain state.
///
/// This trait is consumed by the EVM executor to read account information,
/// storage slots, contract code, and block hashes during transaction execution.
///
/// ## Design Notes
///
/// - **Sync for S01:** The trait is synchronous because revm's `Database` trait
///   is sync. The roadmap calls for `AsyncStateProvider` (EVM-04) with async
///   bridging (tokio `block_on` or dedicated I/O threads) when actual async
///   backends (MonadDb) are introduced.
///
/// - **Send + Sync:** Required so the trait object can be shared across threads
///   in future slices (parallel execution, MVHashMap).
///
/// - **Fallible:** All methods return `Result` to propagate state access errors
///   cleanly through the execution pipeline.
///
/// ## Implementations
///
/// - `InMemoryState` (S01) — HashMap-backed mock for testing
/// - MVHashMap adapter (S03) — optimistic concurrency control
/// - MonadDb adapter (future) — persistent storage backend
pub trait StateProvider: Send + Sync {
    /// Returns the account info for the given address, or `None` if the
    /// account does not exist in state.
    fn basic_account(&self, address: Address) -> Result<Option<AccountInfo>, EvmError>;

    /// Returns the storage value at the given slot for the given address.
    /// Returns `U256::ZERO` if the slot has not been written.
    fn storage(&self, address: Address, slot: U256) -> Result<U256, EvmError>;

    /// Returns the bytecode for the given code hash.
    /// Returns empty `Bytes` if the code hash is not found.
    fn code_by_hash(&self, code_hash: B256) -> Result<Bytes, EvmError>;

    /// Returns the block hash for the given block number.
    /// Returns `B256::ZERO` if the block number is not known.
    fn block_hash(&self, number: u64) -> Result<B256, EvmError>;
}
