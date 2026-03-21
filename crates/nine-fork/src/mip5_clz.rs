//! MIP-5: CLZ (Count Leading Zeros) opcode verification.
//!
//! CLZ is already implemented in revm v36's OSAKA spec at opcode `0x1E`.
//! The implementation lives in `revm-interpreter-34.0.0/src/instructions/bitwise.rs`
//! and uses `U256::leading_zeros()`, which maps to hardware `lzcnt` on x86-64.
//!
//! The opcode is gated behind `check!(OSAKA)` — since `Context::mainnet()` defaults
//! to `SpecId::OSAKA`, CLZ is automatically available in our executor without any
//! configuration changes.
//!
//! ## Gas Cost
//!
//! CLZ uses the standard VERYLOW gas cost (3 gas) as a simple bitwise operation.
//!
//! ## Specification
//!
//! - Opcode: `0x1E`
//! - Stack input: 1 value (the U256 to count leading zeros of)
//! - Stack output: 1 value (number of leading zero bits, 0..=256)
//! - Edge cases: input `0` → `256`, input `U256::MAX` → `0`, input `1` → `255`

#[cfg(test)]
mod tests {
    use monad_evm::EvmExecutor;
    use monad_state::InMemoryState;
    use monad_types::{
        AccountInfo, Address, BlockEnv, Bytes, ExecutionResult, Transaction, U256,
    };

    /// Sender address for test transactions.
    fn sender() -> Address {
        Address::with_last_byte(0xA0)
    }

    /// Helper to build a contract that:
    /// 1. Computes CLZ of a hardcoded PUSH32 value in its constructor
    /// 2. Stores the result at storage slot 0 via SSTORE
    /// 3. Returns empty runtime code (minimal contract)
    ///
    /// This avoids needing a separate CALL after deployment — we read
    /// the result from state changes (storage slot 0 of the deployed contract).
    fn build_clz_deploy_bytecode(value: U256) -> Vec<u8> {
        let mut bytecode = Vec::new();

        // PUSH32 <value> — push the test value onto the stack
        bytecode.push(0x7F); // PUSH32
        let value_bytes: [u8; 32] = value.to_be_bytes::<32>();
        bytecode.extend_from_slice(&value_bytes);

        // CLZ (0x1E) — count leading zeros, replaces top of stack
        bytecode.push(0x1E);

        // PUSH1 0x00 — storage slot 0
        bytecode.push(0x60);
        bytecode.push(0x00);

        // SSTORE — store CLZ result at slot 0
        bytecode.push(0x55);

        // Return empty runtime code (contract with no code body)
        // PUSH1 0x00, PUSH1 0x00, RETURN
        bytecode.push(0x60);
        bytecode.push(0x00);
        bytecode.push(0x60);
        bytecode.push(0x00);
        bytecode.push(0xF3);

        bytecode
    }

    /// Builds a contract with CLZ in its runtime code so we can CALL it
    /// and read the return value. The runtime code:
    ///   PUSH32 <value> → CLZ → PUSH1 0x00 → MSTORE → PUSH1 0x20 → PUSH1 0x00 → RETURN
    ///
    /// Init code deploys this runtime code using:
    ///   PUSH <runtime_len> → DUP1 → PUSH <offset> → PUSH1 0 → CODECOPY → PUSH1 0 → RETURN
    fn build_clz_callable_contract(value: U256) -> Vec<u8> {
        // Runtime code
        let mut runtime = Vec::new();
        runtime.push(0x7F); // PUSH32
        runtime.extend_from_slice(&value.to_be_bytes::<32>());
        runtime.push(0x1E); // CLZ
        runtime.push(0x60); // PUSH1 0x00
        runtime.push(0x00);
        runtime.push(0x52); // MSTORE
        runtime.push(0x60); // PUSH1 0x20
        runtime.push(0x20);
        runtime.push(0x60); // PUSH1 0x00
        runtime.push(0x00);
        runtime.push(0xF3); // RETURN

        let runtime_len = runtime.len();

        // Init code: copy runtime to memory and return it
        let mut init = Vec::new();
        // PUSH1 <runtime_len>
        init.push(0x60);
        init.push(runtime_len as u8);
        // DUP1 (for RETURN size)
        init.push(0x80);
        // PUSH1 <init_code_len + runtime stuff offset> — we'll calculate after
        // The offset of runtime code = length of init code
        // Init code so far is 7 bytes, plus 2 more for the CODECOPY args = we need to know total
        // Let's build it differently:

        // PUSH1 runtime_len
        init.clear();
        init.push(0x60);
        init.push(runtime_len as u8);
        // DUP1
        init.push(0x80);
        // PUSH1 <offset> — will be init.len() + 4 (remaining init bytes)
        let offset_pos = init.len();
        init.push(0x60);
        init.push(0x00); // placeholder
        // PUSH1 0x00 (destOffset in memory)
        init.push(0x60);
        init.push(0x00);
        // CODECOPY
        init.push(0x39);
        // PUSH1 0x00 (memory offset for RETURN)
        init.push(0x60);
        init.push(0x00);
        // RETURN
        init.push(0xF3);

        // Fix offset: runtime code starts right after init code
        let init_len = init.len();
        init[offset_pos + 1] = init_len as u8;

        // Concatenate init + runtime
        let mut full = init;
        full.extend_from_slice(&runtime);
        full
    }

    /// Executes a CLZ test by deploying a contract that stores CLZ(value) in
    /// storage slot 0, then checks the stored result matches expected.
    fn verify_clz_via_sstore(value: U256, expected_clz: u64) {
        let bytecode = build_clz_deploy_bytecode(value);
        let state = InMemoryState::new()
            .with_account(sender(), AccountInfo::new(U256::from(10_000_000_000u64), 0));

        let tx = Transaction {
            sender: sender(),
            to: None, // CREATE
            value: U256::ZERO,
            data: Bytes::from(bytecode),
            gas_limit: 1_000_000,
            nonce: 0,
            gas_price: U256::ZERO,
        };

        let (result, state_changes) =
            EvmExecutor::execute_tx_with_state_changes(&tx, &state, &BlockEnv::default())
                .expect("CLZ deploy transaction should succeed");

        assert!(
            result.is_success(),
            "CLZ deploy should succeed for value {:#x}, got {:?}",
            value, result
        );

        // Find the deployed contract's state changes (not the sender)
        let contract_entry = state_changes
            .iter()
            .find(|(addr, (_, storage))| {
                **addr != sender() && !storage.is_empty()
            })
            .unwrap_or_else(|| {
                panic!(
                    "Expected contract with storage changes for CLZ({:#x}). State changes: {:?}",
                    value,
                    state_changes.keys().collect::<Vec<_>>()
                )
            });

        let (_contract_addr, (_info, storage)) = contract_entry;
        let clz_result = storage
            .get(&U256::ZERO)
            .unwrap_or_else(|| panic!("Storage slot 0 should contain CLZ result"));

        assert_eq!(
            *clz_result,
            U256::from(expected_clz),
            "CLZ({:#x}) should be {}, got {}",
            value, expected_clz, clz_result
        );
    }

    /// Executes a CLZ test by deploying a callable contract, then calling it
    /// and checking the return value.
    fn verify_clz_via_call(value: U256, expected_clz: u64) {
        let deploy_bytecode = build_clz_callable_contract(value);
        let state = InMemoryState::new()
            .with_account(sender(), AccountInfo::new(U256::from(10_000_000_000u64), 0));

        // Step 1: Deploy
        let deploy_tx = Transaction {
            sender: sender(),
            to: None,
            value: U256::ZERO,
            data: Bytes::from(deploy_bytecode),
            gas_limit: 1_000_000,
            nonce: 0,
            gas_price: U256::ZERO,
        };

        let (result, deploy_changes) =
            EvmExecutor::execute_tx_with_state_changes(&deploy_tx, &state, &BlockEnv::default())
                .expect("CLZ contract deploy should succeed");

        assert!(result.is_success(), "deploy should succeed: {:?}", result);

        // Find the deployed contract address
        let (contract_addr, (contract_info, _)) = deploy_changes
            .iter()
            .find(|(addr, (info, _))| {
                **addr != sender() && info.code.as_ref().is_some_and(|c| !c.is_empty())
            })
            .expect("should find deployed contract");

        // Step 2: Call the contract
        let mut call_state = InMemoryState::new()
            .with_account(sender(), AccountInfo::new(U256::from(10_000_000_000u64), 1));

        // Re-create the contract in state for the call
        let code = contract_info.code.clone().unwrap();
        let code_hash = alloy_primitives::keccak256(&code);
        call_state.insert_account(
            *contract_addr,
            AccountInfo::new_contract(U256::ZERO, 1, code_hash.into(), code),
        );

        let call_tx = Transaction {
            sender: sender(),
            to: Some(*contract_addr),
            value: U256::ZERO,
            data: Bytes::new(),
            gas_limit: 1_000_000,
            nonce: 1,
            gas_price: U256::ZERO,
        };

        let result = EvmExecutor::execute_tx(&call_tx, &call_state, &BlockEnv::default())
            .expect("CLZ contract call should succeed");

        match &result {
            ExecutionResult::Success { output, .. } => {
                let result_value = U256::from_be_slice(output.as_ref());
                assert_eq!(
                    result_value,
                    U256::from(expected_clz),
                    "CLZ({:#x}) via CALL should be {}, got {}",
                    value, expected_clz, result_value
                );
            }
            other => panic!("Expected Success, got {:?}", other),
        }
    }

    // ─── CLZ edge case tests via SSTORE pattern ────────────────────────

    #[test]
    fn clz_of_zero_returns_256() {
        verify_clz_via_sstore(U256::ZERO, 256);
    }

    #[test]
    fn clz_of_one_returns_255() {
        verify_clz_via_sstore(U256::from(1u64), 255);
    }

    #[test]
    fn clz_of_max_returns_0() {
        verify_clz_via_sstore(U256::MAX, 0);
    }

    #[test]
    fn clz_of_high_bit_set_returns_0() {
        // 1 << 255 — highest possible bit set
        let value = U256::from(1u64) << 255;
        verify_clz_via_sstore(value, 0);
    }

    #[test]
    fn clz_of_two_returns_254() {
        verify_clz_via_sstore(U256::from(2u64), 254);
    }

    #[test]
    fn clz_of_0xff_returns_248() {
        verify_clz_via_sstore(U256::from(0xFFu64), 248);
    }

    #[test]
    fn clz_of_0x100_returns_247() {
        verify_clz_via_sstore(U256::from(0x100u64), 247);
    }

    #[test]
    fn clz_of_arbitrary_value() {
        // 0x00000000_00000000_00000001_00000000... = bit 128 set → 127 leading zeros
        let value = U256::from(1u128) << 128;
        verify_clz_via_sstore(value, 127);
    }

    #[test]
    fn clz_of_second_highest_bit() {
        // 1 << 254 → 1 leading zero
        let value = U256::from(1u64) << 254;
        verify_clz_via_sstore(value, 1);
    }

    // ─── CLZ via CALL pattern (proves runtime code execution) ──────────

    #[test]
    fn clz_via_call_zero() {
        verify_clz_via_call(U256::ZERO, 256);
    }

    #[test]
    fn clz_via_call_max() {
        verify_clz_via_call(U256::MAX, 0);
    }

    #[test]
    fn clz_via_call_one() {
        verify_clz_via_call(U256::from(1u64), 255);
    }

    // ─── Gas accounting ────────────────────────────────────────────────

    #[test]
    fn clz_gas_accounting() {
        // Deploy and verify that gas used is reasonable (includes CLZ cost).
        // CLZ uses VERYLOW gas (3) — we verify the overall transaction completes
        // within expected gas bounds.
        let bytecode = build_clz_deploy_bytecode(U256::from(42u64));
        let state = InMemoryState::new()
            .with_account(sender(), AccountInfo::new(U256::from(10_000_000_000u64), 0));

        let tx = Transaction {
            sender: sender(),
            to: None,
            value: U256::ZERO,
            data: Bytes::from(bytecode),
            gas_limit: 1_000_000,
            nonce: 0,
            gas_price: U256::ZERO,
        };

        let result = EvmExecutor::execute_tx(&tx, &state, &BlockEnv::default())
            .expect("should succeed");

        assert!(result.is_success());
        let gas = result.gas_used();
        // Gas should include:
        // - Intrinsic (21000 + calldata cost)
        // - PUSH32 (3) + CLZ (3) + PUSH1 (3) + SSTORE (20000+) + PUSH1 (3) + PUSH1 (3) + RETURN (0)
        // Total should be > 21000 and reasonably bounded
        assert!(
            gas > 21_000,
            "CLZ transaction should use more than intrinsic gas, got {}",
            gas
        );
        assert!(
            gas < 100_000,
            "CLZ transaction should use less than 100k gas, got {}",
            gas
        );
    }
}
