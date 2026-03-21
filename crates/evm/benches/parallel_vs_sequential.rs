//! Criterion benchmark suite comparing parallel vs sequential block execution.
//!
//! Three benchmark groups measure throughput under different contention patterns:
//!
//! - **independent_transfers**: 8 unique senders → 8 unique receivers (zero conflicts).
//!   Parallel should outperform sequential due to full concurrency.
//!
//! - **serial_dependency**: 3 txs from the same sender (nonces 0/1/2), forcing
//!   OCC conflicts on balance/nonce. Parallel ≈ sequential (forced serialization).
//!
//! - **mixed_contention**: 8 txs with 2 conflicting pairs + 4 independent txs.
//!   Exercises partial parallelism; expect some speedup vs sequential.
//!
//! # Usage
//!
//! Full measurement run (writes HTML reports to `target/criterion/`):
//! ```bash
//! cargo bench --bench parallel_vs_sequential
//! ```
//!
//! Quick compile-and-run validation (no measurement, just verifies it works):
//! ```bash
//! cargo bench --bench parallel_vs_sequential -- --test
//! ```
//!
//! # Setup Pattern
//!
//! Each group constructs an `InMemoryState` with funded senders + zero-balance
//! coinbase, a `BlockEnv`, and a `Vec<Transaction>`. Addresses use the 0xE0+
//! range to avoid precompile collisions (0x01-0x13, 0x100). Gas limit is 100,000
//! per tx (well under EIP-7825 16M cap). Gas price is 1 gwei.
//!
//! The parallel path composes `execute_block_parallel()` + `execute_block()` —
//! the same full pipeline used by the integration tests in `block_execution.rs`.

use std::sync::Arc;

use alloy_primitives::{Address, Bytes, U256};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use monad_evm::{execute_block, execute_block_sequential};
use monad_scheduler::execute_block_parallel;
use monad_state::InMemoryState;
use monad_types::{AccountInfo, BlockEnv, Transaction};

// ── Constants ───────────────────────────────────────────────────────────

/// 1 ETH in wei.
const ONE_ETH: u128 = 1_000_000_000_000_000_000;
/// 100 ETH in wei — initial sender balance.
const HUNDRED_ETH: u128 = 100 * ONE_ETH;
/// 1 gwei — gas price for all benchmark transactions.
const GAS_PRICE: u64 = 1_000_000_000;
/// Gas limit per transaction (well under EIP-7825 16M cap).
const GAS_LIMIT: u64 = 100_000;
/// Number of parallel worker threads.
const NUM_WORKERS: usize = 4;

// ── Helpers ─────────────────────────────────────────────────────────────

fn coinbase_addr() -> Address {
    Address::with_last_byte(0xC0)
}

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
        gas_limit: GAS_LIMIT,
        nonce,
        gas_price: U256::from(GAS_PRICE),
    }
}

/// Run the full parallel pipeline: execute_block_parallel() → execute_block().
fn bench_parallel(transactions: &[Transaction], base_state: &InMemoryState, block_env: &BlockEnv) {
    let state_provider: Arc<dyn monad_state::StateProvider> = Arc::new(base_state.clone());
    let par_result = execute_block_parallel(transactions, state_provider, block_env, NUM_WORKERS);

    let total_fees = par_result.beneficiary_tracker.total_fees();
    let _block_result =
        execute_block(base_state, &par_result.tx_results, total_fees, block_env)
            .expect("execute_block should succeed");
}

/// Run the sequential pipeline: execute_block_sequential().
fn bench_sequential(
    transactions: &[Transaction],
    base_state: &InMemoryState,
    block_env: &BlockEnv,
) {
    let _block_result = execute_block_sequential(transactions, base_state, block_env)
        .expect("execute_block_sequential should succeed");
}

// ── Benchmark Groups ────────────────────────────────────────────────────

/// 8 unique senders → 8 unique receivers (zero conflicts).
/// Parallel should achieve maximum concurrency.
fn bench_independent_transfers(c: &mut Criterion) {
    let mut group = c.benchmark_group("independent_transfers");

    let one_eth = U256::from(ONE_ETH);
    let hundred_eth = U256::from(HUNDRED_ETH);

    // 8 senders: 0xE1..0xE8, 8 receivers: 0xF1..0xF8
    let senders: Vec<Address> = (0xE1u8..=0xE8).map(Address::with_last_byte).collect();
    let receivers: Vec<Address> = (0xF1u8..=0xF8).map(Address::with_last_byte).collect();

    let mut base_state =
        InMemoryState::new().with_account(coinbase_addr(), AccountInfo::new(U256::ZERO, 0));
    for &sender in &senders {
        base_state = base_state.with_account(sender, AccountInfo::new(hundred_eth, 0));
    }

    let block_env = make_block_env();
    let transactions: Vec<Transaction> = senders
        .iter()
        .zip(receivers.iter())
        .map(|(&from, &to)| make_transfer(from, to, one_eth, 0))
        .collect();

    group.bench_function("parallel", |b| {
        b.iter(|| bench_parallel(black_box(&transactions), black_box(&base_state), &block_env))
    });

    group.bench_function("sequential", |b| {
        b.iter(|| {
            bench_sequential(black_box(&transactions), black_box(&base_state), &block_env)
        })
    });

    group.finish();
}

/// 3 txs from the same sender with nonces 0/1/2.
/// Forces OCC conflicts on sender balance/nonce — parallel ≈ sequential.
fn bench_serial_dependency(c: &mut Criterion) {
    let mut group = c.benchmark_group("serial_dependency");

    let one_eth = U256::from(ONE_ETH);
    let hundred_eth = U256::from(HUNDRED_ETH);

    let sender = Address::with_last_byte(0xE1);
    let receivers: Vec<Address> = (0xF1u8..=0xF3).map(Address::with_last_byte).collect();

    let base_state = InMemoryState::new()
        .with_account(sender, AccountInfo::new(hundred_eth, 0))
        .with_account(coinbase_addr(), AccountInfo::new(U256::ZERO, 0));

    let block_env = make_block_env();
    let transactions: Vec<Transaction> = receivers
        .iter()
        .enumerate()
        .map(|(i, &to)| make_transfer(sender, to, one_eth, i as u64))
        .collect();

    group.bench_function("parallel", |b| {
        b.iter(|| bench_parallel(black_box(&transactions), black_box(&base_state), &block_env))
    });

    group.bench_function("sequential", |b| {
        b.iter(|| {
            bench_sequential(black_box(&transactions), black_box(&base_state), &block_env)
        })
    });

    group.finish();
}

/// 8 txs — 2 conflicting pairs (each pair shares a sender) + 4 independent txs.
///
/// Layout:
/// - Pair 1: sender 0xE1 → receiver 0xF1 (nonce 0), sender 0xE1 → receiver 0xF2 (nonce 1)
/// - Pair 2: sender 0xE2 → receiver 0xF3 (nonce 0), sender 0xE2 → receiver 0xF4 (nonce 1)
/// - Independent: sender 0xE3 → receiver 0xF5, sender 0xE4 → receiver 0xF6,
///                sender 0xE5 → receiver 0xF7, sender 0xE6 → receiver 0xF8
///
/// Exercises partial parallelism — expect some speedup vs fully sequential.
fn bench_mixed_contention(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed_contention");

    let one_eth = U256::from(ONE_ETH);
    let hundred_eth = U256::from(HUNDRED_ETH);

    // 6 senders: 0xE1..0xE6
    let senders: Vec<Address> = (0xE1u8..=0xE6).map(Address::with_last_byte).collect();

    let mut base_state =
        InMemoryState::new().with_account(coinbase_addr(), AccountInfo::new(U256::ZERO, 0));
    for &sender in &senders {
        base_state = base_state.with_account(sender, AccountInfo::new(hundred_eth, 0));
    }

    let block_env = make_block_env();

    let transactions = vec![
        // Pair 1: same sender, sequential nonces
        make_transfer(senders[0], Address::with_last_byte(0xF1), one_eth, 0),
        make_transfer(senders[0], Address::with_last_byte(0xF2), one_eth, 1),
        // Pair 2: same sender, sequential nonces
        make_transfer(senders[1], Address::with_last_byte(0xF3), one_eth, 0),
        make_transfer(senders[1], Address::with_last_byte(0xF4), one_eth, 1),
        // 4 independent txs
        make_transfer(senders[2], Address::with_last_byte(0xF5), one_eth, 0),
        make_transfer(senders[3], Address::with_last_byte(0xF6), one_eth, 0),
        make_transfer(senders[4], Address::with_last_byte(0xF7), one_eth, 0),
        make_transfer(senders[5], Address::with_last_byte(0xF8), one_eth, 0),
    ];

    group.bench_function("parallel", |b| {
        b.iter(|| bench_parallel(black_box(&transactions), black_box(&base_state), &block_env))
    });

    group.bench_function("sequential", |b| {
        b.iter(|| {
            bench_sequential(black_box(&transactions), black_box(&base_state), &block_env)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_independent_transfers,
    bench_serial_dependency,
    bench_mixed_contention
);
criterion_main!(benches);
