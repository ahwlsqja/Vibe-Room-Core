//! Standard precompile registry and verification.
//!
//! Provides functions to enumerate and verify the standard Ethereum precompile
//! set that revm activates for `SpecId::CANCUN`.

use alloy_primitives::Address;
use revm::precompile::{Precompiles, PrecompileSpecId};

/// All 9 standard precompile addresses.
pub const STANDARD_PRECOMPILE_ADDRESSES: [Address; 9] = [
    addr(1),  // ecrecover
    addr(2),  // SHA-256
    addr(3),  // RIPEMD-160
    addr(4),  // identity
    addr(5),  // modexp
    addr(6),  // ecAdd (BN128)
    addr(7),  // ecMul (BN128)
    addr(8),  // ecPairing (BN128)
    addr(9),  // blake2f
];

/// Human-readable names for the 9 standard precompiles, indexed by address - 1.
pub const PRECOMPILE_NAMES: [&str; 9] = [
    "ecrecover",
    "SHA-256",
    "RIPEMD-160",
    "identity",
    "modexp",
    "ecAdd",
    "ecMul",
    "ecPairing",
    "blake2f",
];

/// Returns `true` if the given address is a standard precompile (0x01..=0x09).
pub fn is_precompile(address: &Address) -> bool {
    STANDARD_PRECOMPILE_ADDRESSES.contains(address)
}

/// Verifies that revm's CANCUN precompile set contains all 9 standard
/// precompiles. Returns a list of any missing addresses (should be empty).
///
/// This serves as a compile-time + runtime verification that our EVM
/// configuration includes all required precompiles.
pub fn verify_cancun_precompiles() -> Vec<Address> {
    let precompiles = Precompiles::new(PrecompileSpecId::CANCUN);
    let mut missing = Vec::new();

    for &address in &STANDARD_PRECOMPILE_ADDRESSES {
        if !precompiles.contains(&address) {
            missing.push(address);
        }
    }

    missing
}

/// Constructs an Ethereum address from a low byte (e.g., `addr(1)` → `0x0000...0001`).
const fn addr(n: u8) -> Address {
    let mut bytes = [0u8; 20];
    bytes[19] = n;
    Address::new(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_9_precompiles_present_in_cancun() {
        let missing = verify_cancun_precompiles();
        assert!(
            missing.is_empty(),
            "Missing precompiles in CANCUN spec: {:?}",
            missing
        );
    }

    #[test]
    fn is_precompile_detects_standard_addresses() {
        for &addr in &STANDARD_PRECOMPILE_ADDRESSES {
            assert!(is_precompile(&addr), "Expected {:?} to be a precompile", addr);
        }

        // Non-precompile address
        let non_precompile = Address::with_last_byte(0x10);
        assert!(!is_precompile(&non_precompile));
    }

    #[test]
    fn precompile_names_match_count() {
        assert_eq!(PRECOMPILE_NAMES.len(), STANDARD_PRECOMPILE_ADDRESSES.len());
    }
}
