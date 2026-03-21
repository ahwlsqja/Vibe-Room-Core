//! Standard Ethereum precompile registry for monad-core.
//!
//! This crate documents and verifies the 9 standard EVM precompiles that are
//! automatically activated by revm when using `SpecId::CANCUN` (or any spec
//! from Istanbul onwards for all 9):
//!
//! | Address | Name          | Added In    |
//! |---------|---------------|-------------|
//! | 0x01    | ecrecover     | Homestead   |
//! | 0x02    | SHA-256       | Homestead   |
//! | 0x03    | RIPEMD-160    | Homestead   |
//! | 0x04    | identity      | Homestead   |
//! | 0x05    | modexp        | Byzantium   |
//! | 0x06    | ecAdd (BN128) | Byzantium   |
//! | 0x07    | ecMul (BN128) | Byzantium   |
//! | 0x08    | ecPairing     | Byzantium   |
//! | 0x09    | blake2f       | Istanbul    |
//!
//! ## Architecture Note
//!
//! revm v36 activates all standard precompiles automatically based on the
//! `SpecId` passed to the EVM context. Our `EvmExecutor` in `monad-evm` uses
//! `SpecId::CANCUN`, which includes all 9 precompiles above (Cancun inherits
//! Berlin → Istanbul → Byzantium → Homestead). No explicit wiring is needed.
//!
//! This crate provides:
//! 1. `registry` module — documents the precompile set and provides a
//!    verification function that confirms all 9 are present.
//! 2. Test vectors (`tests/precompile_vectors.rs`) proving each precompile
//!    returns correct output for known inputs.

pub mod registry;
