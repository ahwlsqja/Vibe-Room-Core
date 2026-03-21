//! # monad-nine-fork
//!
//! NINE FORK compliance verification for the Monad EVM.
//!
//! This crate implements and verifies compliance with the NINE FORK specification
//! (MIP-3, MIP-4, MIP-5) through the monad-core EVM execution pipeline.
//!
//! # NINE FORK Requirement Coverage
//!
//! | Req ID | Requirement | Module | Verification |
//! |--------|-------------|--------|-------------|
//! | NINE-01 (MIP-3) | Linear memory pool with watermark-stack semantics | [`mip3_memory`] | Nested CALL/REVERT tests at depths 1–6 verify memory isolation. revm's `SharedMemory` checkpoint stack satisfies the single-slab watermark model. |
//! | NINE-02 (MIP-4) | Reserve balance precompile at `0x20` | [`mip4_reserve`], [`nine_fork_precompiles`] | 15 unit tests + 2 integration tests verify ABI encoding, gas cost (100), dipped/not-dipped semantics, and `NineForkPrecompiles` provider delegation via STATICCALL. |
//! | NINE-03 (MIP-5) | CLZ opcode at `0x1E` via OSAKA spec | [`mip5_clz`] | 13 tests deploy EVM bytecode with opcode `0x1E`, verify `CLZ(0)=256`, `CLZ(MAX)=0`, `CLZ(1)=255`, arbitrary values, and gas accounting through `EvmExecutor`. |
//! | NINE-04 (modexp EIP-7823) | 1024-byte modexp input bounds | [`modexp_validate`] | 11 tests call modexp precompile at `0x05` with inputs at and above 1024 bytes, verifying `PrecompileError::ModexpEip7823LimitSize` rejection and valid-input acceptance. |
//! | NINE-05 (thread safety) | Send+Sync for parallel execution readiness | [`safety`] | Compile-time assertions verify `NineForkPrecompiles`, `ReserveBalanceConfig`, and `EvmError` are `Send+Sync`. `DippedIntoReserve` is `Send` only (intentional `RefCell` limitation for S02; replaced in S03/S04). |
//!
//! # Architecture
//!
//! All verification flows through `EvmExecutor::execute_tx_with_state_changes()`,
//! proving that the features work through the complete EVM pipeline (bytecode →
//! interpreter → state changes), not just at the instruction/precompile level.
//!
//! The [`nine_fork_precompiles::NineForkPrecompiles`] provider wraps revm's
//! `EthPrecompiles` and extends it with the MIP-4 precompile at address `0x20`,
//! using `Evm::with_precompiles()` to swap in at construction time.
//!
//! # Thread Safety
//!
//! All types intended for parallel execution are `Send + Sync`, enforced by
//! compile-time assertions in the [`safety`] module. The only exception is
//! [`mip4_reserve::DippedIntoReserve`], which uses `RefCell<HashMap>` (S02
//! sequential mode). This will be replaced with per-transaction-index tracking
//! in S03/S04's `MVHashMap`.

pub mod mip3_memory;
pub mod mip4_reserve;
pub mod mip5_clz;
pub mod modexp_validate;
pub mod nine_fork_precompiles;
pub mod safety;
