//! Integration tests for `execute_block()` / `execute_block_sequential()` API.
//!
//! Proves the S05 core claims:
//! - **PARA-06**: Receipts have correct `cumulative_gas_used` (monotonically increasing)
//! - **PARA-07**: `execute_block()` returns correct `BlockResult` for various scenarios
//! - **PARA-08**: Parallel and sequential paths produce identical state roots
//!
//! Test addresses use 0xE0+ range to avoid precompile collisions (0x01-0x13, 0x100).
//! Gas limits ≤ 16,777,216 (EIP-7825 OSAKA cap).
//! Gas price = 1 gwei for predictable fee calculations.

use std::sync::Arc;

use alloy_primitives::{address, keccak256, Address, Bytes, B256, U256};

use monad_evm::{execute_block, execute_block_sequential};
use monad_scheduler::execute_block_parallel;
use monad_state::InMemoryState;
use monad_types::{AccountInfo, BlockEnv, BlockResult, Transaction};

// ── Address constants (all ≥ 0xE0) ─────────────────────────────────────

fn sender_a() -> Address {
    address!("0x00000000000000000000000000000000000000E1")
}
fn sender_b() -> Address {
    address!("0x00000000000000000000000000000000000000E2")
}
fn sender_c() -> Address {
    address!("0x00000000000000000000000000000000000000E3")
}
fn sender_d() -> Address {
    address!("0x00000000000000000000000000000000000000E4")
}

fn receiver_a() -> Address {
    address!("0x00000000000000000000000000000000000000F1")
}
fn receiver_b() -> Address {
    address!("0x00000000000000000000000000000000000000F2")
}
fn receiver_c() -> Address {
    address!("0x00000000000000000000000000000000000000F3")
}
fn receiver_d() -> Address {
    address!("0x00000000000000000000000000000000000000F4")
}

fn coinbase_addr() -> Address {
    address!("0x00000000000000000000000000000000000000C0")
}

/// Address with deployed REVERT bytecode for revert tests.
fn revert_contract() -> Address {
    address!("0x00000000000000000000000000000000000000D0")
}

// ── Helpers ─────────────────────────────────────────────────────────────

fn make_block_env() -> BlockEnv {
    BlockEnv {
        number: 1,
        coinbase: coinbase_addr(),
        timestamp: 1_700_000_000,
        gas_limit: 16_777_216, // EIP-7825 OSAKA cap
        base_fee: U256::ZERO,
        difficulty: U256::ZERO,
    }
}

fn make_transfer(from: Address, to: Address, value: U256, nonce: u64) -> Transaction {
    Transaction {
        sender: from,
        to: Some(to),
        value,
        data: Bytes::new(),
        gas_limit: 100_000,
        nonce,
        gas_price: U256::from(1_000_000_000u64), // 1 gwei
    }
}

/// Create a base state with funded senders and a zero-balance coinbase.
fn make_base_state() -> InMemoryState {
    let hundred_eth = U256::from(100) * U256::from(1_000_000_000_000_000_000u128);
    InMemoryState::new()
        .with_account(sender_a(), AccountInfo::new(hundred_eth, 0))
        .with_account(sender_b(), AccountInfo::new(hundred_eth, 0))
        .with_account(sender_c(), AccountInfo::new(hundred_eth, 0))
        .with_account(sender_d(), AccountInfo::new(hundred_eth, 0))
        .with_account(coinbase_addr(), AccountInfo::new(U256::ZERO, 0))
}

/// Run transactions through the parallel path (execute_block_parallel → execute_block),
/// composing the full pipeline in test scope where monad-scheduler is a dev-dependency.
fn run_parallel_block(
    transactions: &[Transaction],
    base_state: &InMemoryState,
    block_env: &BlockEnv,
) -> BlockResult {
    let state_provider: Arc<dyn monad_state::StateProvider> = Arc::new(base_state.clone());
    let par_result = execute_block_parallel(transactions, state_provider, block_env, 4);

    let total_fees = par_result.beneficiary_tracker.total_fees();
    execute_block(base_state, &par_result.tx_results, total_fees, block_env)
        .expect("execute_block should succeed")
}

/// Run transactions through the sequential path.
fn run_sequential_block(
    transactions: &[Transaction],
    base_state: &InMemoryState,
    block_env: &BlockEnv,
) -> BlockResult {
    execute_block_sequential(transactions, base_state, block_env)
        .expect("execute_block_sequential should succeed")
}

/// Assert that two BlockResults have identical state roots.
fn assert_state_roots_match(label: &str, parallel: &BlockResult, sequential: &BlockResult) {
    assert_eq!(
        parallel.state_root, sequential.state_root,
        "[{}] State root mismatch!\n  parallel:   {:?}\n  sequential: {:?}",
        label, parallel.state_root, sequential.state_root
    );
}

/// Assert that receipts have monotonically increasing cumulative gas.
fn assert_cumulative_gas_monotonic(label: &str, result: &BlockResult) {
    let mut prev_gas = 0u64;
    for (i, receipt) in result.receipts.iter().enumerate() {
        assert!(
            receipt.cumulative_gas_used >= prev_gas,
            "[{}] Cumulative gas is not monotonically increasing at receipt {}: {} < {}",
            label,
            i,
            receipt.cumulative_gas_used,
            prev_gas
        );
        if receipt.success || receipt.cumulative_gas_used > 0 {
            // For successful txs, cumulative gas must strictly increase.
            // For failed txs, gas may still be consumed.
            assert!(
                receipt.cumulative_gas_used > prev_gas || i == 0,
                "[{}] Cumulative gas did not increase at receipt {}: {} == {}",
                label,
                i,
                receipt.cumulative_gas_used,
                prev_gas
            );
        }
        prev_gas = receipt.cumulative_gas_used;
    }
}

// ── Test Cases: State Root Differential ─────────────────────────────────

/// Empty block: 0 transactions.
/// Both paths should return B256::ZERO state root and empty receipts.
#[test]
fn test_block_empty() {
    let base_state = make_base_state();
    let block_env = make_block_env();
    let transactions: Vec<Transaction> = vec![];

    let par_result = run_parallel_block(&transactions, &base_state, &block_env);
    let seq_result = run_sequential_block(&transactions, &base_state, &block_env);

    assert_state_roots_match("empty_block", &par_result, &seq_result);
    assert_eq!(par_result.state_root, B256::ZERO, "empty block → zero state root");
    assert!(par_result.receipts.is_empty(), "empty block → no receipts");
    assert_eq!(par_result.gas_used, 0, "empty block → zero gas");
    assert!(par_result.logs.is_empty(), "empty block → no logs");
    assert_eq!(seq_result.state_root, B256::ZERO, "sequential empty → zero state root");
}

/// Single transfer: 1 transaction.
/// Both paths should produce identical non-zero state root.
/// 1 receipt with `success: true` and `cumulative_gas_used == gas_used`.
#[test]
fn test_block_single_transfer() {
    let base_state = make_base_state();
    let block_env = make_block_env();
    let one_eth = U256::from(1_000_000_000_000_000_000u128);

    let transactions = vec![make_transfer(sender_a(), receiver_a(), one_eth, 0)];

    let par_result = run_parallel_block(&transactions, &base_state, &block_env);
    let seq_result = run_sequential_block(&transactions, &base_state, &block_env);

    assert_state_roots_match("single_transfer", &par_result, &seq_result);
    assert_ne!(par_result.state_root, B256::ZERO, "non-empty block → non-zero state root");

    // Receipt checks.
    assert_eq!(par_result.receipts.len(), 1, "single tx → 1 receipt");
    assert!(par_result.receipts[0].success, "transfer should succeed");
    assert_eq!(
        par_result.receipts[0].cumulative_gas_used, par_result.gas_used,
        "single tx: cumulative == total gas"
    );
    assert!(par_result.gas_used > 0, "should consume gas");

    // Sequential path should match.
    assert_eq!(seq_result.receipts.len(), 1);
    assert!(seq_result.receipts[0].success);
    assert_eq!(par_result.gas_used, seq_result.gas_used, "gas_used should match");
}

/// Independent transfers: 4 txs from different senders to different receivers.
/// Zero conflicts — parallel should produce identical state root without re-execution.
/// 4 receipts with correct cumulative gas (PARA-06).
#[test]
fn test_block_independent_transfers() {
    let base_state = make_base_state();
    let block_env = make_block_env();
    let one_eth = U256::from(1_000_000_000_000_000_000u128);

    let transactions = vec![
        make_transfer(sender_a(), receiver_a(), one_eth, 0),
        make_transfer(sender_b(), receiver_b(), one_eth, 0),
        make_transfer(sender_c(), receiver_c(), one_eth, 0),
        make_transfer(sender_d(), receiver_d(), one_eth, 0),
    ];

    let par_result = run_parallel_block(&transactions, &base_state, &block_env);
    let seq_result = run_sequential_block(&transactions, &base_state, &block_env);

    assert_state_roots_match("independent_transfers", &par_result, &seq_result);

    // Receipt count and success.
    assert_eq!(par_result.receipts.len(), 4, "4 txs → 4 receipts");
    for (i, receipt) in par_result.receipts.iter().enumerate() {
        assert!(receipt.success, "transfer {} should succeed", i);
    }

    // PARA-06: cumulative gas is monotonically increasing.
    assert_cumulative_gas_monotonic("independent_transfers_par", &par_result);
    assert_cumulative_gas_monotonic("independent_transfers_seq", &seq_result);

    // Total gas should match.
    assert_eq!(par_result.gas_used, seq_result.gas_used, "total gas must match");
}

/// Serial dependency chain: 3 txs from same sender (nonce 0, 1, 2).
/// Forces OCC conflicts on sender balance/nonce — parallel must re-execute
/// to produce correct results. Both paths must produce identical state root.
#[test]
fn test_block_serial_dependency() {
    let base_state = make_base_state();
    let block_env = make_block_env();
    let one_eth = U256::from(1_000_000_000_000_000_000u128);

    let transactions = vec![
        make_transfer(sender_a(), receiver_a(), one_eth, 0),
        make_transfer(sender_a(), receiver_b(), one_eth, 1),
        make_transfer(sender_a(), receiver_c(), one_eth, 2),
    ];

    let par_result = run_parallel_block(&transactions, &base_state, &block_env);
    let seq_result = run_sequential_block(&transactions, &base_state, &block_env);

    assert_state_roots_match("serial_dependency", &par_result, &seq_result);

    // All 3 should succeed.
    assert_eq!(par_result.receipts.len(), 3, "3 txs → 3 receipts");
    for (i, receipt) in par_result.receipts.iter().enumerate() {
        assert!(receipt.success, "serial tx {} should succeed", i);
    }

    // Cumulative gas monotonically increasing (PARA-06).
    assert_cumulative_gas_monotonic("serial_dependency_par", &par_result);
    assert_cumulative_gas_monotonic("serial_dependency_seq", &seq_result);

    // Gas totals match.
    assert_eq!(par_result.gas_used, seq_result.gas_used);
}

/// Mixed block: 2 conflicting pairs.
/// tx0: sender_a → receiver_a (nonce 0) — conflicts with tx1 on sender_a
/// tx1: sender_a → receiver_b (nonce 1) — conflicts with tx0 on sender_a
/// tx2: sender_b → receiver_c (nonce 0) — independent
/// tx3: sender_b → receiver_d (nonce 1) — conflicts with tx2 on sender_b
///
/// Exercises both conflict patterns simultaneously.
#[test]
fn test_block_mixed() {
    let base_state = make_base_state();
    let block_env = make_block_env();
    let one_eth = U256::from(1_000_000_000_000_000_000u128);

    let transactions = vec![
        make_transfer(sender_a(), receiver_a(), one_eth, 0),
        make_transfer(sender_a(), receiver_b(), one_eth, 1),
        make_transfer(sender_b(), receiver_c(), one_eth, 0),
        make_transfer(sender_b(), receiver_d(), one_eth, 1),
    ];

    let par_result = run_parallel_block(&transactions, &base_state, &block_env);
    let seq_result = run_sequential_block(&transactions, &base_state, &block_env);

    assert_state_roots_match("mixed_block", &par_result, &seq_result);

    // All should succeed.
    assert_eq!(par_result.receipts.len(), 4, "4 txs → 4 receipts");
    for (i, receipt) in par_result.receipts.iter().enumerate() {
        assert!(receipt.success, "mixed tx {} should succeed", i);
    }

    // Cumulative gas checks.
    assert_cumulative_gas_monotonic("mixed_par", &par_result);
    assert_cumulative_gas_monotonic("mixed_seq", &seq_result);

    assert_eq!(par_result.gas_used, seq_result.gas_used);
}

// ── Test Cases: Edge Cases ──────────────────────────────────────────────

/// All-revert block: every transaction calls a contract that REVERTs.
///
/// Sets up contract bytecode at `revert_contract()` address: `0x5F5FFD`
/// (PUSH0 PUSH0 REVERT — pushes offset=0, size=0 and then REVERTs).
/// All receipts should have `success: false`. State root should match
/// between parallel and sequential paths.
#[test]
fn test_block_all_revert() {
    let block_env = make_block_env();

    // PUSH0 PUSH0 REVERT = 0x5F 0x5F 0xFD
    let revert_bytecode = Bytes::from(vec![0x5F, 0x5F, 0xFD]);
    let code_hash = keccak256(&revert_bytecode);

    let hundred_eth = U256::from(100) * U256::from(1_000_000_000_000_000_000u128);

    // Set up the revert contract with code in the state.
    let revert_acct = AccountInfo {
        balance: U256::ZERO,
        nonce: 0,
        code_hash,
        code: Some(revert_bytecode.clone()),
    };

    let base_state = InMemoryState::new()
        .with_account(sender_a(), AccountInfo::new(hundred_eth, 0))
        .with_account(sender_b(), AccountInfo::new(hundred_eth, 0))
        .with_account(revert_contract(), revert_acct)
        .with_code(code_hash, revert_bytecode)
        .with_account(coinbase_addr(), AccountInfo::new(U256::ZERO, 0));

    // Transactions that CALL the revert contract — they will all revert.
    let transactions = vec![
        Transaction {
            sender: sender_a(),
            to: Some(revert_contract()),
            value: U256::ZERO, // No value transfer, just a CALL
            data: Bytes::new(),
            gas_limit: 100_000,
            nonce: 0,
            gas_price: U256::from(1_000_000_000u64),
        },
        Transaction {
            sender: sender_b(),
            to: Some(revert_contract()),
            value: U256::ZERO,
            data: Bytes::new(),
            gas_limit: 100_000,
            nonce: 0,
            gas_price: U256::from(1_000_000_000u64),
        },
    ];

    let par_result = run_parallel_block(&transactions, &base_state, &block_env);
    let seq_result = run_sequential_block(&transactions, &base_state, &block_env);

    assert_state_roots_match("all_revert", &par_result, &seq_result);

    // All receipts should show failure.
    assert_eq!(par_result.receipts.len(), 2, "2 txs → 2 receipts");
    for (i, receipt) in par_result.receipts.iter().enumerate() {
        assert!(
            !receipt.success,
            "revert tx {} should have success=false, got success=true",
            i
        );
    }

    // Cumulative gas should still be monotonically increasing (gas is consumed
    // even on revert).
    assert_cumulative_gas_monotonic("all_revert_par", &par_result);
    assert_cumulative_gas_monotonic("all_revert_seq", &seq_result);

    // Sequential should also show all failures.
    for (i, receipt) in seq_result.receipts.iter().enumerate() {
        assert!(!receipt.success, "sequential revert tx {} should fail", i);
    }

    // Gas totals match.
    assert_eq!(par_result.gas_used, seq_result.gas_used);
}

/// Determinism: Running execute_block() twice with identical inputs must produce
/// bitwise-identical state roots. This catches HashMap non-determinism.
#[test]
fn test_block_state_root_determinism() {
    let base_state = make_base_state();
    let block_env = make_block_env();
    let one_eth = U256::from(1_000_000_000_000_000_000u128);

    let transactions = vec![
        make_transfer(sender_a(), receiver_a(), one_eth, 0),
        make_transfer(sender_b(), receiver_b(), one_eth, 0),
        make_transfer(sender_c(), receiver_c(), one_eth, 0),
    ];

    // Run sequential path twice (deterministic single-threaded execution).
    let result1 = run_sequential_block(&transactions, &base_state, &block_env);
    let result2 = run_sequential_block(&transactions, &base_state, &block_env);

    assert_eq!(
        result1.state_root, result2.state_root,
        "State root must be bitwise identical across runs.\n  run1: {:?}\n  run2: {:?}",
        result1.state_root, result2.state_root
    );
    assert_eq!(result1.gas_used, result2.gas_used);
    assert_eq!(result1.receipts.len(), result2.receipts.len());

    // Also run parallel path twice.
    let par_result1 = run_parallel_block(&transactions, &base_state, &block_env);
    let par_result2 = run_parallel_block(&transactions, &base_state, &block_env);

    assert_eq!(
        par_result1.state_root, par_result2.state_root,
        "Parallel state root must be deterministic.\n  run1: {:?}\n  run2: {:?}",
        par_result1.state_root, par_result2.state_root
    );
}

/// Large block: many independent txs to stress-test cumulative gas accounting.
/// Verifies PARA-06 at scale.
#[test]
fn test_block_cumulative_gas_accounting() {
    let block_env = make_block_env();
    let one_eth = U256::from(1_000_000_000_000_000_000u128);
    let hundred_eth = U256::from(100) * one_eth;

    // Create 8 unique senders and receivers.
    let senders: Vec<Address> = (0xE1u8..=0xE8)
        .map(|b| Address::with_last_byte(b))
        .collect();
    let receivers: Vec<Address> = (0xF1u8..=0xF8)
        .map(|b| Address::with_last_byte(b))
        .collect();

    let mut base_state = InMemoryState::new()
        .with_account(coinbase_addr(), AccountInfo::new(U256::ZERO, 0));
    for &sender in &senders {
        base_state = base_state.with_account(sender, AccountInfo::new(hundred_eth, 0));
    }

    let transactions: Vec<Transaction> = senders
        .iter()
        .zip(receivers.iter())
        .map(|(&from, &to)| make_transfer(from, to, one_eth, 0))
        .collect();

    let par_result = run_parallel_block(&transactions, &base_state, &block_env);
    let seq_result = run_sequential_block(&transactions, &base_state, &block_env);

    assert_state_roots_match("cumulative_gas_8tx", &par_result, &seq_result);

    // Verify cumulative gas accounting (PARA-06).
    assert_eq!(par_result.receipts.len(), 8, "8 txs → 8 receipts");
    assert_cumulative_gas_monotonic("cumulative_gas_par", &par_result);
    assert_cumulative_gas_monotonic("cumulative_gas_seq", &seq_result);

    // All transfers are identical (21_000 gas each for simple ETH transfer).
    // Cumulative gas at receipt i should be (i+1) * single_tx_gas.
    let single_tx_gas = par_result.receipts[0].cumulative_gas_used;
    for (i, receipt) in par_result.receipts.iter().enumerate() {
        let expected_cumulative = single_tx_gas * (i as u64 + 1);
        assert_eq!(
            receipt.cumulative_gas_used, expected_cumulative,
            "Receipt {} cumulative gas: expected {}, got {}",
            i, expected_cumulative, receipt.cumulative_gas_used
        );
    }

    // Total gas should be 8 * single_tx_gas.
    assert_eq!(par_result.gas_used, single_tx_gas * 8);
    assert_eq!(par_result.gas_used, seq_result.gas_used);
}
