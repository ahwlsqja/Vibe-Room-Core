use alloy_primitives::{Bytes, U256, B256};
use serde::{Deserialize, Serialize};

/// Account information stored in the state trie.
///
/// Represents the state of a single Ethereum account, including balance,
/// nonce, code hash, and optionally the bytecode itself. The `code` field
/// is `Some` when the full bytecode is cached alongside the account info
/// (e.g., after reading from state), and `None` when only the hash is known.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountInfo {
    /// Account balance in wei.
    pub balance: U256,
    /// Transaction count / sequence number.
    pub nonce: u64,
    /// Keccak-256 hash of the account's bytecode. For externally owned
    /// accounts (EOAs), this is the keccak-256 of empty bytes.
    pub code_hash: B256,
    /// Bytecode of the contract. `None` for EOAs or when not loaded.
    pub code: Option<Bytes>,
}

/// Keccak-256 hash of the empty byte string — the code hash for EOAs.
pub const KECCAK_EMPTY: B256 = B256::new([
    0xc5, 0xd2, 0x46, 0x01, 0x86, 0xf7, 0x23, 0x3c,
    0x92, 0x7e, 0x7d, 0xb2, 0xdc, 0xc7, 0x03, 0xc0,
    0xe5, 0x00, 0xb6, 0x53, 0xca, 0x82, 0x27, 0x3b,
    0x7b, 0xfa, 0xd8, 0x04, 0x5d, 0x85, 0xa4, 0x70,
]);

impl AccountInfo {
    /// Creates a new EOA (externally owned account) with the given balance and nonce.
    pub fn new(balance: U256, nonce: u64) -> Self {
        Self {
            balance,
            nonce,
            code_hash: KECCAK_EMPTY,
            code: None,
        }
    }

    /// Creates a contract account with the given balance, nonce, code hash, and bytecode.
    pub fn new_contract(balance: U256, nonce: u64, code_hash: B256, code: Bytes) -> Self {
        Self {
            balance,
            nonce,
            code_hash,
            code: Some(code),
        }
    }

    /// Returns `true` if this account has no code (i.e., is an EOA).
    pub fn is_empty_code(&self) -> bool {
        self.code_hash == KECCAK_EMPTY
    }

    /// Returns `true` if the account has zero balance, zero nonce, and no code.
    pub fn is_empty(&self) -> bool {
        self.balance.is_zero() && self.nonce == 0 && self.is_empty_code()
    }

    /// Creates an empty account (zero balance, zero nonce, no code).
    pub fn default_account() -> Self {
        Self::new(U256::ZERO, 0)
    }
}

impl Default for AccountInfo {
    fn default() -> Self {
        Self::default_account()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eoa_account() {
        let acct = AccountInfo::new(U256::from(1_000_000u64), 5);
        assert_eq!(acct.balance, U256::from(1_000_000u64));
        assert_eq!(acct.nonce, 5);
        assert!(acct.is_empty_code());
        assert!(acct.code.is_none());
    }

    #[test]
    fn test_contract_account() {
        let code = Bytes::from(vec![0x60, 0x00, 0x60, 0x00, 0xf3]);
        let code_hash = B256::with_last_byte(0xAB);
        let acct = AccountInfo::new_contract(
            U256::ZERO,
            1,
            code_hash,
            code.clone(),
        );
        assert!(!acct.is_empty_code());
        assert_eq!(acct.code, Some(code));
        assert_eq!(acct.code_hash, code_hash);
    }

    #[test]
    fn test_empty_account() {
        let acct = AccountInfo::default();
        assert!(acct.is_empty());
        assert!(acct.is_empty_code());
        assert_eq!(acct.balance, U256::ZERO);
        assert_eq!(acct.nonce, 0);
    }

    #[test]
    fn test_non_empty_account_with_balance() {
        let acct = AccountInfo::new(U256::from(1u64), 0);
        assert!(!acct.is_empty());
    }

    #[test]
    fn test_non_empty_account_with_nonce() {
        let acct = AccountInfo::new(U256::ZERO, 1);
        assert!(!acct.is_empty());
    }

    #[test]
    fn test_keccak_empty_constant() {
        // Well-known keccak-256 of empty bytes
        assert_eq!(
            format!("{:x}", KECCAK_EMPTY),
            "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
        );
    }

    #[test]
    fn test_account_info_clone_eq() {
        let acct = AccountInfo::new(U256::from(42u64), 3);
        let acct2 = acct.clone();
        assert_eq!(acct, acct2);
    }
}
