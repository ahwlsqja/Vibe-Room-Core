//! EIP-7823: modexp input size limit verification.
//!
//! EIP-7823 enforces a 1024-byte limit on `base_len`, `mod_len`, and `exp_len`
//! inputs to the modexp precompile (address `0x05`) when running under the OSAKA
//! spec. This prevents DoS attacks via excessively large modular exponentiation
//! inputs.
//!
//! The enforcement is already implemented in revm v36's OSAKA modexp variant
//! (`osaka_run` in `revm-precompile-32.1.0/src/modexp.rs`). Since
//! `Context::mainnet()` defaults to `SpecId::OSAKA`, this protection is
//! automatically active in our executor.
//!
//! ## Key constant
//!
//! `eip7823::INPUT_SIZE_LIMIT = 1024` — maximum allowed byte length for
//! base, exponent, and modulus inputs.
//!
//! ## Failure mode
//!
//! When any input exceeds 1024 bytes, the precompile returns
//! `PrecompileError::ModexpEip7823LimitSize`, which surfaces as
//! `ExecutionResult::Revert` through the EVM execution pipeline.

#[cfg(test)]
mod tests {
    use monad_evm::EvmExecutor;
    use monad_state::InMemoryState;
    use monad_types::{
        AccountInfo, Address, BlockEnv, Bytes, ExecutionResult, Transaction, U256,
    };

    /// Sender address for test transactions.
    fn sender() -> Address {
        Address::with_last_byte(0xB0)
    }

    /// Helper to create AccountInfo with code for a contract.
    fn account_with_code(code: Vec<u8>) -> AccountInfo {
        let code_bytes = Bytes::from(code);
        let code_hash = alloy_primitives::keccak256(&code_bytes);
        AccountInfo::new_contract(U256::ZERO, 1, code_hash.into(), code_bytes)
    }

    /// Builds modexp calldata: 3 x 32-byte length fields + base + exp + mod data.
    ///
    /// The modexp precompile expects:
    ///   <base_len: 32 bytes> <exp_len: 32 bytes> <mod_len: 32 bytes> <base> <exp> <mod>
    fn build_modexp_input(base_len: usize, exp_len: usize, mod_len: usize) -> Vec<u8> {
        let mut input = Vec::new();

        // base_len as 32-byte big-endian
        let mut bl = [0u8; 32];
        bl[28..32].copy_from_slice(&(base_len as u32).to_be_bytes());
        input.extend_from_slice(&bl);

        // exp_len as 32-byte big-endian
        let mut el = [0u8; 32];
        el[28..32].copy_from_slice(&(exp_len as u32).to_be_bytes());
        input.extend_from_slice(&el);

        // mod_len as 32-byte big-endian
        let mut ml = [0u8; 32];
        ml[28..32].copy_from_slice(&(mod_len as u32).to_be_bytes());
        input.extend_from_slice(&ml);

        // base data (fill with 0x01)
        input.extend(vec![0x01u8; base_len]);

        // exp data (fill with 0x01)
        input.extend(vec![0x01u8; exp_len]);

        // mod data (fill with 0x02 to avoid modulus 0/1 edge cases)
        input.extend(vec![0x02u8; mod_len]);

        input
    }

    /// Builds EVM bytecode for a contract that calls the modexp precompile
    /// via STATICCALL with the given calldata stored in memory.
    ///
    /// Pattern:
    /// 1. Store calldata in memory at offset 0 using CALLDATACOPY
    /// 2. STATICCALL(gas, 0x05, 0, calldatasize, retOffset, retSize)
    /// 3. Return the success flag + return data
    ///
    /// The contract accepts calldata as transaction input and forwards it to modexp.
    fn build_modexp_caller_bytecode() -> Vec<u8> {
        vec![
            // Copy all calldata to memory at offset 0
            0x36,       // CALLDATASIZE
            0x60, 0x00, // PUSH1 0 (srcOffset)
            0x60, 0x00, // PUSH1 0 (destOffset)
            0x37,       // CALLDATACOPY

            // STATICCALL(gas, addr=0x05, argOffset=0, argSize=CALLDATASIZE, retOffset, retSize)
            0x60, 0x20, // PUSH1 32 (retSize — modexp returns mod_len bytes, 32 is enough for test)
            0x36,       // CALLDATASIZE (use as retOffset — put return data after input)
            0x36,       // CALLDATASIZE (argSize)
            0x60, 0x00, // PUSH1 0 (argOffset)
            0x60, 0x05, // PUSH1 0x05 (modexp address)
            0x5A,       // GAS
            0xFA,       // STATICCALL

            // Store success flag (1 or 0) at memory[0]
            // The success flag is already on the stack from STATICCALL
            0x60, 0x00, // PUSH1 0
            0x52,       // MSTORE (stores success flag at mem[0])

            // Return 32 bytes from memory[0] (contains success flag as U256)
            0x60, 0x20, // PUSH1 32
            0x60, 0x00, // PUSH1 0
            0xF3,       // RETURN
        ]
    }

    /// Executes a modexp call through the EVM and returns whether the
    /// STATICCALL to the precompile succeeded (true) or reverted (false).
    fn call_modexp(base_len: usize, exp_len: usize, mod_len: usize) -> (bool, ExecutionResult) {
        let caller_code = build_modexp_caller_bytecode();
        let caller_addr = Address::with_last_byte(0xC0);

        let state = InMemoryState::new()
            .with_account(
                sender(),
                AccountInfo::new(U256::from(10_000_000_000_000_000_000u128), 0),
            )
            .with_account(caller_addr, account_with_code(caller_code));

        let modexp_input = build_modexp_input(base_len, exp_len, mod_len);

        let tx = Transaction {
            sender: sender(),
            to: Some(caller_addr),
            value: U256::ZERO,
            data: Bytes::from(modexp_input),
            // Gas limit must stay under EIP-7825's TX_GAS_LIMIT_CAP (16,777,216) for OSAKA
            gas_limit: 16_000_000,
            nonce: 0,
            gas_price: U256::ZERO,
        };

        let result = EvmExecutor::execute_tx(&tx, &state, &BlockEnv::default())
            .expect("transaction execution should not fail at the executor level");

        // Check if the STATICCALL to modexp succeeded
        let precompile_succeeded = match &result {
            ExecutionResult::Success { output, .. } => {
                // The output contains the success flag as a U256
                // STATICCALL returns 1 for success, 0 for failure
                if output.len() >= 32 {
                    let flag = U256::from_be_slice(&output[..32]);
                    flag == U256::from(1u64)
                } else {
                    false
                }
            }
            _ => false,
        };

        (precompile_succeeded, result)
    }

    // ─── EIP-7823: Inputs exceeding 1024 bytes should be REJECTED ──────

    #[test]
    fn modexp_rejects_base_len_exceeding_1024() {
        let (success, result) = call_modexp(1025, 1, 1);
        assert!(
            !success,
            "modexp should reject base_len=1025 (exceeds 1024 limit), result: {:?}",
            result
        );
    }

    #[test]
    fn modexp_rejects_exp_len_exceeding_1024() {
        let (success, result) = call_modexp(1, 1025, 1);
        assert!(
            !success,
            "modexp should reject exp_len=1025 (exceeds 1024 limit), result: {:?}",
            result
        );
    }

    #[test]
    fn modexp_rejects_mod_len_exceeding_1024() {
        let (success, result) = call_modexp(1, 1, 1025);
        assert!(
            !success,
            "modexp should reject mod_len=1025 (exceeds 1024 limit), result: {:?}",
            result
        );
    }

    #[test]
    fn modexp_rejects_all_inputs_exceeding_1024() {
        let (success, result) = call_modexp(2048, 2048, 2048);
        assert!(
            !success,
            "modexp should reject all inputs at 2048 bytes, result: {:?}",
            result
        );
    }

    // ─── EIP-7823: Inputs at exactly 1024 bytes should SUCCEED ─────────

    #[test]
    fn modexp_accepts_inputs_at_1024_bytes() {
        // Use base_len=1024, but small exp/mod to keep gas manageable
        // while still proving the 1024-byte limit is accepted
        let (success, result) = call_modexp(1024, 1, 1024);
        assert!(
            success,
            "modexp should accept base_len=1024, mod_len=1024 with small exp, result: {:?}",
            result
        );
    }

    #[test]
    fn modexp_accepts_base_len_at_1024() {
        let (success, result) = call_modexp(1024, 1, 1);
        assert!(
            success,
            "modexp should accept base_len=1024 with small exp/mod, result: {:?}",
            result
        );
    }

    // ─── Regression: Small valid inputs should always succeed ──────────

    #[test]
    fn modexp_small_valid_inputs() {
        // 2^3 mod 5 = 3
        let (success, result) = call_modexp(1, 1, 1);
        assert!(
            success,
            "modexp with small valid inputs (1,1,1) should succeed, result: {:?}",
            result
        );
    }

    #[test]
    fn modexp_32_byte_inputs() {
        let (success, result) = call_modexp(32, 32, 32);
        assert!(
            success,
            "modexp with 32-byte inputs should succeed, result: {:?}",
            result
        );
    }

    #[test]
    fn modexp_256_byte_inputs() {
        let (success, result) = call_modexp(256, 256, 256);
        assert!(
            success,
            "modexp with 256-byte inputs should succeed, result: {:?}",
            result
        );
    }

    // ─── Boundary tests ────────────────────────────────────────────────

    #[test]
    fn modexp_boundary_1023_bytes() {
        // Use 1023-byte base and mod with small exp to keep gas manageable
        let (success, result) = call_modexp(1023, 1, 1023);
        assert!(
            success,
            "modexp at 1023 bytes should succeed (just under limit), result: {:?}",
            result
        );
    }

    #[test]
    fn modexp_boundary_1024_vs_1025_base() {
        // 1024 should pass
        let (ok, _) = call_modexp(1024, 1, 1);
        assert!(ok, "base_len=1024 should pass");

        // 1025 should fail
        let (fail, _) = call_modexp(1025, 1, 1);
        assert!(!fail, "base_len=1025 should fail");
    }
}
