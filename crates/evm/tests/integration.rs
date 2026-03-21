//! End-to-end integration tests for the full S01 EVM pipeline.
//!
//! These tests prove the complete S01 demo: single transactions produce
//! correct results against in-memory state, with all three execution modes
//! (value transfer, contract deploy, precompile call) exercised in sequence.

use monad_evm::EvmExecutor;
use monad_state::InMemoryState;
use monad_types::{
    AccountInfo, Address, BlockEnv, Bytes, ExecutionResult, Transaction, U256,
};

/// Non-precompile address for the funded EOA sender.
fn sender_addr() -> Address {
    Address::with_last_byte(0x10)
}

/// Non-precompile address for the ETH receiver.
fn receiver_addr() -> Address {
    Address::with_last_byte(0x20)
}

/// Helper to create an AccountInfo with code for pre-loading into state.
fn account_with_code(balance: U256, nonce: u64, code: Vec<u8>) -> AccountInfo {
    let code_bytes = Bytes::from(code);
    let code_hash = alloy_primitives::keccak256(&code_bytes);
    AccountInfo::new_contract(balance, nonce, code_hash.into(), code_bytes)
}

// ─── Full pipeline integration test ────────────────────────────────────────

/// End-to-end test that exercises the complete S01 execution pipeline:
/// 1. Value transfer: send ETH between accounts, verify balance changes
/// 2. Contract deploy: deploy a contract, verify code storage
/// 3. Precompile call: invoke SHA-256 precompile via deployed contract, verify output
///
/// Each step uses its own properly-configured state. The test proves that
/// single transactions produce correct results against in-memory state,
/// and all three capabilities (transfer, deploy, precompile) work through
/// the execution pipeline.
#[test]
fn integration_full_pipeline() {
    let block_env = BlockEnv::default();
    let sender = sender_addr();
    let receiver = receiver_addr();
    let hundred_eth = U256::from(100_000_000_000_000_000_000u128);
    let one_eth = U256::from(1_000_000_000_000_000_000u128);

    // ── Step 1: Value transfer ──────────────────────────────────────────

    let state = InMemoryState::new()
        .with_account(sender, AccountInfo::new(hundred_eth, 0))
        .with_account(receiver, AccountInfo::new(U256::ZERO, 0));

    let transfer_tx = Transaction {
        sender,
        to: Some(receiver),
        value: one_eth,
        data: Bytes::new(),
        gas_limit: 21_000,
        nonce: 0,
        gas_price: U256::ZERO,
    };

    let (result, changes) =
        EvmExecutor::execute_tx_with_state_changes(&transfer_tx, &state, &block_env)
            .expect("value transfer should succeed");

    assert!(result.is_success(), "transfer should succeed: {:?}", result);
    assert_eq!(result.gas_used(), 21_000, "intrinsic gas for simple transfer");

    let (sender_info, _) = changes.get(&sender).expect("sender in changes");
    let (receiver_info, _) = changes.get(&receiver).expect("receiver in changes");
    assert_eq!(sender_info.balance, hundred_eth - one_eth, "sender loses 1 ETH");
    assert_eq!(receiver_info.balance, one_eth, "receiver gains 1 ETH");
    assert_eq!(sender_info.nonce, 1, "sender nonce incremented");

    // ── Step 2: Contract deployment ─────────────────────────────────────
    // Deploy minimal contract: returns single byte 0xFF as runtime code.
    // Init code: PUSH1 0xFF, PUSH1 0, MSTORE, PUSH1 1, PUSH1 31, RETURN
    let init_code = Bytes::from(vec![
        0x60, 0xFF, 0x60, 0x00, 0x52, 0x60, 0x01, 0x60, 0x1F, 0xF3,
    ]);

    let deploy_state = InMemoryState::new()
        .with_account(sender, AccountInfo::new(hundred_eth, 0));

    let deploy_tx = Transaction {
        sender,
        to: None, // CREATE
        value: U256::ZERO,
        data: init_code,
        gas_limit: 100_000,
        nonce: 0,
        gas_price: U256::ZERO,
    };

    let (result, changes) =
        EvmExecutor::execute_tx_with_state_changes(&deploy_tx, &deploy_state, &block_env)
            .expect("contract deploy should succeed");

    assert!(result.is_success(), "deploy should succeed: {:?}", result);
    assert!(result.gas_used() > 21_000, "deploy costs more than intrinsic gas");

    // Find the deployed contract address (not the sender)
    let contract_entry = changes
        .iter()
        .find(|(addr, _)| **addr != sender)
        .expect("deployed contract in state changes");
    let (_contract_addr, (contract_info, _)) = contract_entry;

    assert!(contract_info.code.is_some(), "contract has code stored");
    assert_eq!(
        contract_info.code.as_ref().unwrap().as_ref(),
        &[0xFF],
        "deployed runtime code is 0xFF"
    );
    assert_eq!(contract_info.nonce, 1, "contract nonce is 1");

    // ── Step 3: Precompile call (SHA-256 via STATICCALL) ────────────────
    // EVM bytecode that calls SHA-256 precompile (0x02) with input "hello":
    // 1. PUSH5 "hello" → PUSH1 0 → MSTORE (stores at mem[27..32])
    // 2. STATICCALL(gas, 0x02, argOffset=27, argSize=5, retOffset=32, retSize=32)
    // 3. POP → PUSH1 32 → PUSH1 32 → RETURN
    let sha256_caller = vec![
        0x64, 0x68, 0x65, 0x6c, 0x6c, 0x6f, // PUSH5 "hello"
        0x60, 0x00,                           // PUSH1 0
        0x52,                                 // MSTORE
        0x60, 0x20,                           // PUSH1 32 (retSize)
        0x60, 0x20,                           // PUSH1 32 (retOffset)
        0x60, 0x05,                           // PUSH1 5 (argSize)
        0x60, 0x1B,                           // PUSH1 27 (argOffset)
        0x60, 0x02,                           // PUSH1 0x02 (SHA-256)
        0x5A,                                 // GAS
        0xFA,                                 // STATICCALL
        0x50,                                 // POP success flag
        0x60, 0x20,                           // PUSH1 32
        0x60, 0x20,                           // PUSH1 32
        0xF3,                                 // RETURN
    ];

    let caller_addr = Address::with_last_byte(0x30);
    let precompile_state = InMemoryState::new()
        .with_account(sender, AccountInfo::new(hundred_eth, 0))
        .with_account(caller_addr, account_with_code(U256::ZERO, 1, sha256_caller));

    let call_tx = Transaction {
        sender,
        to: Some(caller_addr),
        value: U256::ZERO,
        data: Bytes::new(),
        gas_limit: 100_000,
        nonce: 0,
        gas_price: U256::ZERO,
    };

    let result = EvmExecutor::execute_tx(&call_tx, &precompile_state, &block_env)
        .expect("precompile call should succeed");

    assert!(result.is_success(), "precompile call should succeed: {:?}", result);

    // SHA-256("hello") = 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
    let expected_hash = hex::decode(
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    ).unwrap();

    match &result {
        ExecutionResult::Success { output, gas_used, .. } => {
            assert_eq!(
                output.as_ref(),
                expected_hash.as_slice(),
                "output should be SHA-256(\"hello\")"
            );
            assert!(*gas_used > 21_000, "precompile call costs more than intrinsic");
        }
        other => panic!("expected Success, got {:?}", other),
    }
}

// ─── Individual precompile-through-EVM test ────────────────────────────────

/// Tests that a STATICCALL to the identity precompile (0x04) through the
/// full EVM execution pipeline returns input data unchanged.
#[test]
fn integration_identity_precompile_through_evm() {
    let sender = sender_addr();

    // Bytecode: stores "test" in memory, STATICCALL to identity (0x04), returns result
    let runtime_code = vec![
        0x63, 0x74, 0x65, 0x73, 0x74,       // PUSH4 "test"
        0x60, 0x00,                           // PUSH1 0
        0x52,                                 // MSTORE
        0x60, 0x04,                           // PUSH1 4 (retSize)
        0x60, 0x20,                           // PUSH1 32 (retOffset)
        0x60, 0x04,                           // PUSH1 4 (argSize)
        0x60, 0x1C,                           // PUSH1 28 (argOffset)
        0x60, 0x04,                           // PUSH1 0x04 (identity)
        0x5A,                                 // GAS
        0xFA,                                 // STATICCALL
        0x50,                                 // POP
        0x60, 0x04,                           // PUSH1 4
        0x60, 0x20,                           // PUSH1 32
        0xF3,                                 // RETURN
    ];

    let contract_addr = Address::with_last_byte(0x30);
    let state = InMemoryState::new()
        .with_account(sender, AccountInfo::new(U256::from(10_000_000_000_000_000_000u128), 0))
        .with_account(contract_addr, account_with_code(U256::ZERO, 1, runtime_code));

    let tx = Transaction {
        sender,
        to: Some(contract_addr),
        value: U256::ZERO,
        data: Bytes::new(),
        gas_limit: 100_000,
        nonce: 0,
        gas_price: U256::ZERO,
    };

    let result = EvmExecutor::execute_tx(&tx, &state, &BlockEnv::default())
        .expect("identity precompile call should succeed");

    match &result {
        ExecutionResult::Success { output, .. } => {
            assert_eq!(output.as_ref(), b"test", "identity returns input unchanged");
        }
        other => panic!("expected Success, got {:?}", other),
    }
}
