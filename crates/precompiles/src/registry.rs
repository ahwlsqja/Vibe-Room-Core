//! Standard precompile registry and verification.
//!
//! Provides functions to enumerate and verify the standard Ethereum precompile
//! sets that revm activates. Supports both CANCUN (legacy) and OSAKA (current
//! default) specifications.
//!
//! OSAKA extends Prague (which adds BLS12-381 at 0x0b-0x11) with:
//! - modexp OSAKA variant (EIP-7823 input bounds, replaces BERLIN at 0x05)
//! - P256VERIFY_OSAKA at address 0x100 (secp256r1)
//!
//! The full OSAKA set is: 9 standard (0x01-0x09) + KZG point eval (0x0a)
//! + 7 BLS12-381 (0x0b-0x11) + P256VERIFY (0x100) = 18 precompiles.

use alloy_primitives::Address;
use revm::precompile::{Precompiles, PrecompileSpecId};

/// All 9 standard precompile addresses (present since Byzantium/Istanbul).
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

/// KZG point evaluation precompile address (EIP-4844), added in Cancun.
pub const KZG_POINT_EVALUATION_ADDRESS: Address = addr(0x0a);

/// BLS12-381 precompile addresses added in Prague (0x0b-0x11).
pub const BLS12_381_PRECOMPILE_ADDRESSES: [Address; 7] = [
    addr(0x0b), // g1Add
    addr(0x0c), // g1Msm
    addr(0x0d), // g2Add
    addr(0x0e), // g2Msm
    addr(0x0f), // pairing
    addr(0x10), // mapFpToG1
    addr(0x11), // mapFp2ToG2
];

/// P256VERIFY precompile address (secp256r1) at 0x100, added in OSAKA.
pub const P256VERIFY_ADDRESS: Address = addr_u64(0x100);

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

/// Returns `true` if the given address is any OSAKA-era precompile
/// (standard 0x01-0x09, BLS12-381 0x0b-0x11, or P256VERIFY at 0x100).
pub fn is_osaka_precompile(address: &Address) -> bool {
    STANDARD_PRECOMPILE_ADDRESSES.contains(address)
        || *address == KZG_POINT_EVALUATION_ADDRESS
        || BLS12_381_PRECOMPILE_ADDRESSES.contains(address)
        || *address == P256VERIFY_ADDRESS
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

/// Verifies that revm's OSAKA precompile set contains all expected precompiles:
/// - 9 standard precompiles (0x01-0x09)
/// - 7 BLS12-381 precompiles (0x0b-0x11, added in Prague)
/// - P256VERIFY at 0x100 (added in OSAKA)
///
/// Returns a list of any missing addresses (should be empty).
///
/// Since our executor uses `Context::mainnet()` which defaults to `SpecId::OSAKA`,
/// this verifies the actual precompile set available during execution.
pub fn verify_osaka_precompiles() -> Vec<Address> {
    let precompiles = Precompiles::new(PrecompileSpecId::OSAKA);
    let mut missing = Vec::new();

    // Standard precompiles (0x01-0x09)
    for &address in &STANDARD_PRECOMPILE_ADDRESSES {
        if !precompiles.contains(&address) {
            missing.push(address);
        }
    }

    // KZG point evaluation (0x0a, from Cancun)
    if !precompiles.contains(&KZG_POINT_EVALUATION_ADDRESS) {
        missing.push(KZG_POINT_EVALUATION_ADDRESS);
    }

    // BLS12-381 precompiles (0x0b-0x11, from Prague)
    for &address in &BLS12_381_PRECOMPILE_ADDRESSES {
        if !precompiles.contains(&address) {
            missing.push(address);
        }
    }

    // P256VERIFY (0x100, OSAKA)
    if !precompiles.contains(&P256VERIFY_ADDRESS) {
        missing.push(P256VERIFY_ADDRESS);
    }

    missing
}

/// Constructs an Ethereum address from a low byte (e.g., `addr(1)` → `0x0000...0001`).
const fn addr(n: u8) -> Address {
    let mut bytes = [0u8; 20];
    bytes[19] = n;
    Address::new(bytes)
}

/// Constructs an Ethereum address from a u64 value (for addresses > 0xFF like P256VERIFY at 0x100).
const fn addr_u64(n: u64) -> Address {
    let mut bytes = [0u8; 20];
    let n_bytes = n.to_be_bytes();
    // Copy the last 8 bytes of the u64 into the last 8 bytes of the address
    bytes[12] = n_bytes[0];
    bytes[13] = n_bytes[1];
    bytes[14] = n_bytes[2];
    bytes[15] = n_bytes[3];
    bytes[16] = n_bytes[4];
    bytes[17] = n_bytes[5];
    bytes[18] = n_bytes[6];
    bytes[19] = n_bytes[7];
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
    fn all_precompiles_present_in_osaka() {
        let missing = verify_osaka_precompiles();
        assert!(
            missing.is_empty(),
            "Missing precompiles in OSAKA spec: {:?}",
            missing
        );
    }

    #[test]
    fn osaka_is_superset_of_cancun() {
        let osaka = Precompiles::new(PrecompileSpecId::OSAKA);
        let cancun = Precompiles::new(PrecompileSpecId::CANCUN);

        // Every CANCUN precompile should be in OSAKA
        for addr in cancun.addresses() {
            assert!(
                osaka.contains(addr),
                "OSAKA should contain CANCUN precompile {:?}",
                addr
            );
        }

        // OSAKA should have strictly more precompiles than CANCUN
        let osaka_count = osaka.addresses().count();
        let cancun_count = cancun.addresses().count();
        assert!(
            osaka_count > cancun_count,
            "OSAKA ({}) should have more precompiles than CANCUN ({})",
            osaka_count, cancun_count
        );
    }

    #[test]
    fn osaka_includes_bls12_381() {
        let osaka = Precompiles::new(PrecompileSpecId::OSAKA);
        for &addr in &BLS12_381_PRECOMPILE_ADDRESSES {
            assert!(
                osaka.contains(&addr),
                "OSAKA should contain BLS12-381 precompile at {:?}",
                addr
            );
        }
    }

    #[test]
    fn osaka_includes_p256verify() {
        let osaka = Precompiles::new(PrecompileSpecId::OSAKA);
        assert!(
            osaka.contains(&P256VERIFY_ADDRESS),
            "OSAKA should contain P256VERIFY at {:?}",
            P256VERIFY_ADDRESS
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
    fn is_osaka_precompile_detects_all() {
        // Standard precompiles
        for &addr in &STANDARD_PRECOMPILE_ADDRESSES {
            assert!(is_osaka_precompile(&addr));
        }
        // KZG point evaluation
        assert!(is_osaka_precompile(&KZG_POINT_EVALUATION_ADDRESS));
        // BLS12-381 precompiles
        for &addr in &BLS12_381_PRECOMPILE_ADDRESSES {
            assert!(is_osaka_precompile(&addr));
        }
        // P256VERIFY
        assert!(is_osaka_precompile(&P256VERIFY_ADDRESS));
        // Non-precompile
        assert!(!is_osaka_precompile(&Address::with_last_byte(0xA0)));
    }

    #[test]
    fn precompile_names_match_count() {
        assert_eq!(PRECOMPILE_NAMES.len(), STANDARD_PRECOMPILE_ADDRESSES.len());
    }

    #[test]
    fn osaka_precompile_count() {
        let osaka = Precompiles::new(PrecompileSpecId::OSAKA);
        let count = osaka.addresses().count();
        // 9 standard + 1 KZG + 7 BLS12-381 + 1 P256VERIFY = 18
        assert_eq!(
            count, 18,
            "OSAKA should have 18 precompiles (9 standard + 1 KZG + 7 BLS12-381 + 1 P256VERIFY), got {}",
            count
        );
    }
}
