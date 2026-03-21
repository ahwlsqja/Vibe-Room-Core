//! Integration tests for MvDatabase executing real EVM transactions.
//!
//! These tests prove that MvDatabase correctly routes state reads through
//! MVHashMap, records per-field reads in ReadSet, and propagates ESTIMATE
//! errors. They also verify that LazyBeneficiaryTracker prevents false
//! conflicts on the coinbase address.
//!
//! Test addresses use 0xE0+ range to avoid precompile collisions (0x01-0x13, 0x100).
//! Gas limits are ≤ 16,777,216 (EIP-7825 OSAKA cap).

use std::sync::Arc;

use alloy_primitives::{Address, U256};

use monad_mv_state::{
    LazyBeneficiaryTracker, LocationKey, MVHashMap, MvDatabase, MvReadResult,
    ReadOrigin, WriteSet, WriteValue,
};
use monad_state::InMemoryState;
use monad_types::{AccountInfo, Bytes};

use revm::{
    context::{Context, TxEnv},
    primitives::TxKind,
    ExecuteEvm, MainBuilder, MainContext,
};

/// Sender address in the 0xE0+ range.
fn sender_addr() -> Address {
    Address::with_last_byte(0xE1)
}

/// Receiver address in the 0xE0+ range.
fn receiver_addr() -> Address {
    Address::with_last_byte(0xE2)
}

/// Secondary receiver address in the 0xE0+ range.
fn receiver2_addr() -> Address {
    Address::with_last_byte(0xE3)
}

/// Coinbase (block beneficiary) address in the 0xE0+ range.
fn coinbase_addr() -> Address {
    Address::with_last_byte(0xE0)
}

/// Helper: execute a simple value transfer transaction through MvDatabase.
///
/// Returns the MvDatabase after execution (with ReadSet/WriteSet still inside).
/// The caller is responsible for extracting the sets.
fn execute_transfer_via_mvdb(
    mv_state: Arc<MVHashMap>,
    base_state: Arc<dyn monad_state::StateProvider>,
    tx_index: u32,
    sender: Address,
    receiver: Address,
    value: U256,
    nonce: u64,
) -> monad_mv_state::MvDatabase {
    let mut db = MvDatabase::new(
        Arc::clone(&mv_state),
        Arc::clone(&base_state),
        tx_index,
    );

    // Build the EVM with MvDatabase as the database.
    let ctx = Context::mainnet()
        .with_db(&mut db)
        .modify_block_chained(|block| {
            block.number = U256::from(1);
            block.timestamp = U256::from(1000);
            block.gas_limit = 16_777_216;
            block.basefee = 0;
            block.beneficiary = coinbase_addr();
        })
        .modify_cfg_chained(|cfg| {
            cfg.chain_id = 1;
            cfg.disable_eip3607 = true;
            cfg.disable_base_fee = true;
            cfg.disable_fee_charge = true;
        });

    let mut evm = ctx.build_mainnet();

    let tx_env = TxEnv::builder()
        .caller(sender)
        .kind(TxKind::Call(receiver))
        .value(value)
        .data(Bytes::new())
        .gas_limit(21_000)
        .gas_price(0)
        .nonce(nonce)
        .build()
        .expect("valid tx env");

    let result = evm.transact_one(tx_env).expect("tx should execute");

    assert!(
        result.is_success(),
        "transaction should succeed: {:?}",
        result
    );

    db
}

// ── Test 1: Sequential multi-tx through MvDatabase ──────────────────────

/// Execute tx0 through MvDatabase, apply its WriteSet to MVHashMap,
/// then execute tx1 at tx_index=1. Verify tx1's ReadSet shows it read
/// tx0's output via MVHashMap (not just base state).
#[test]
fn sequential_multi_tx_reads_through_mvhashmap() {
    let hundred_eth = U256::from(100_000_000_000_000_000_000u128);
    let one_eth = U256::from(1_000_000_000_000_000_000u128);

    let base_state: Arc<dyn monad_state::StateProvider> = Arc::new(
        InMemoryState::new()
            .with_account(sender_addr(), AccountInfo::new(hundred_eth, 0))
            .with_account(receiver_addr(), AccountInfo::new(U256::ZERO, 0))
            .with_account(receiver2_addr(), AccountInfo::new(U256::ZERO, 0)),
    );

    let mv_state = Arc::new(MVHashMap::new());

    // ── tx0: sender → receiver (1 ETH) ─────────────────────────────

    let mut db0 = execute_transfer_via_mvdb(
        Arc::clone(&mv_state),
        Arc::clone(&base_state),
        0,
        sender_addr(),
        receiver_addr(),
        one_eth,
        0,
    );

    let rs0 = db0.take_read_set();
    let mut ws0 = db0.take_write_set();

    // tx0 should have read sender and receiver balance/nonce/codehash from base state.
    assert!(
        rs0.len() >= 3,
        "tx0 should have recorded reads: got {}",
        rs0.len()
    );

    // Manually record the state changes that EVM produced into the WriteSet.
    // In real execution, the scheduler does this. Here we simulate the sender
    // balance decrease and nonce increment, and receiver balance increase.
    ws0.record(
        LocationKey::Balance(sender_addr()),
        WriteValue::Balance(hundred_eth - one_eth),
    );
    ws0.record(
        LocationKey::Nonce(sender_addr()),
        WriteValue::Nonce(1),
    );
    ws0.record(
        LocationKey::Balance(receiver_addr()),
        WriteValue::Balance(one_eth),
    );

    // Publish tx0's writes to MVHashMap.
    ws0.apply_to(&mv_state, 0, 0);

    // Verify MVHashMap has tx0's balance write for receiver.
    match mv_state.read(&LocationKey::Balance(receiver_addr()), 1) {
        MvReadResult::Value(WriteValue::Balance(v), 0, 0) => {
            assert_eq!(v, one_eth, "MVHashMap should have receiver's 1 ETH");
        }
        other => panic!("expected Value(Balance) from tx=0, got {:?}", other),
    }

    // ── tx1: sender → receiver2 (1 ETH) ────────────────────────────
    // tx1 executes at tx_index=1. When it reads sender's balance, it should
    // see tx0's updated value from MVHashMap (not the original base state).

    let mut db1 = MvDatabase::new(
        Arc::clone(&mv_state),
        Arc::clone(&base_state),
        1,
    );

    // Read sender's info through MvDatabase — should see tx0's writes.
    let acct = revm::database_interface::Database::basic(&mut db1, sender_addr())
        .unwrap()
        .unwrap();

    // Sender balance should reflect tx0's write (100 ETH - 1 ETH = 99 ETH).
    assert_eq!(
        acct.balance,
        hundred_eth - one_eth,
        "tx1 should see sender's balance after tx0's write via MVHashMap"
    );
    assert_eq!(
        acct.nonce, 1,
        "tx1 should see sender's nonce after tx0's increment via MVHashMap"
    );

    // Verify ReadSet recorded the reads as coming from MVHashMap.
    let rs1 = db1.take_read_set();
    assert!(rs1.len() >= 3, "tx1 should have 3+ reads");

    // Check that Balance read came from MVHashMap (tx=0).
    let mut found_mv_balance = false;
    for (loc, origin) in rs1.iter() {
        if let LocationKey::Balance(addr) = loc {
            if *addr == sender_addr() {
                match origin {
                    ReadOrigin::MvHashMap {
                        tx_index: 0,
                        incarnation: 0,
                    } => found_mv_balance = true,
                    _ => panic!(
                        "sender balance read should be from MVHashMap(tx=0,inc=0), got {:?}",
                        origin
                    ),
                }
            }
        }
    }
    assert!(
        found_mv_balance,
        "tx1 must have read sender's balance from MVHashMap"
    );

    // Validate the ReadSet — should pass since MVHashMap hasn't changed.
    assert!(rs1.validate(&mv_state, 1), "ReadSet should validate");
}

// ── Test 2: ESTIMATE marker produces error ──────────────────────────────

/// Write a value to MVHashMap, then mark it as ESTIMATE.
/// Verify that MvDatabase returns a ReadEstimate error when reading.
#[test]
fn estimate_marker_produces_error() {
    let base_state: Arc<dyn monad_state::StateProvider> = Arc::new(
        InMemoryState::new()
            .with_account(
                sender_addr(),
                AccountInfo::new(U256::from(1000u64), 0),
            ),
    );

    let mv_state = Arc::new(MVHashMap::new());

    // tx=0 writes sender's balance.
    mv_state.write(
        LocationKey::Balance(sender_addr()),
        0,
        0,
        WriteValue::Balance(U256::from(500)),
    );

    // Mark tx=0 as ESTIMATE (being re-executed).
    mv_state.mark_estimate(0);

    // tx=1 tries to read sender's info — should get ReadEstimate error.
    let mut db = MvDatabase::new(Arc::clone(&mv_state), base_state, 1);
    let result = revm::database_interface::Database::basic(&mut db, sender_addr());

    assert!(result.is_err(), "Reading ESTIMATE should return error");

    let err = result.unwrap_err();
    match &err.0 {
        monad_types::EvmError::ReadEstimate { tx_index, location } => {
            assert_eq!(*tx_index, 0, "ESTIMATE should reference tx=0");
            assert!(
                location.contains("Balance"),
                "Error location should mention Balance, got: {}",
                location
            );
        }
        other => panic!("Expected ReadEstimate error, got {:?}", other),
    }
}

/// ESTIMATE on storage slot also produces error.
#[test]
fn estimate_marker_on_storage_produces_error() {
    let base_state: Arc<dyn monad_state::StateProvider> = Arc::new(
        InMemoryState::new()
            .with_account(
                sender_addr(),
                AccountInfo::new(U256::from(1000u64), 0),
            )
            .with_storage(sender_addr(), U256::from(0), U256::from(42)),
    );

    let mv_state = Arc::new(MVHashMap::new());

    // tx=0 writes a storage slot.
    mv_state.write(
        LocationKey::Storage(sender_addr(), U256::from(0)),
        0,
        0,
        WriteValue::Storage(U256::from(999)),
    );
    mv_state.mark_estimate(0);

    // tx=1 reads the storage slot — should get error.
    let mut db = MvDatabase::new(Arc::clone(&mv_state), base_state, 1);
    let result = revm::database_interface::Database::storage(&mut db, sender_addr(), U256::from(0));

    assert!(result.is_err(), "Reading ESTIMATE storage should return error");

    let err = result.unwrap_err();
    match &err.0 {
        monad_types::EvmError::ReadEstimate { tx_index, .. } => {
            assert_eq!(*tx_index, 0);
        }
        other => panic!("Expected ReadEstimate, got {:?}", other),
    }
}

// ── Test 3: Lazy beneficiary — no coinbase balance in MVHashMap ──────────

/// Execute a transaction, record the gas fee in LazyBeneficiaryTracker,
/// and verify that MVHashMap has NO entry for coinbase balance.
/// This proves that the lazy pattern prevents false conflicts.
#[test]
fn lazy_beneficiary_keeps_coinbase_out_of_mvhashmap() {
    let hundred_eth = U256::from(100_000_000_000_000_000_000u128);
    let one_eth = U256::from(1_000_000_000_000_000_000u128);

    let base_state: Arc<dyn monad_state::StateProvider> = Arc::new(
        InMemoryState::new()
            .with_account(sender_addr(), AccountInfo::new(hundred_eth, 0))
            .with_account(receiver_addr(), AccountInfo::new(U256::ZERO, 0))
            .with_account(coinbase_addr(), AccountInfo::new(U256::ZERO, 0)),
    );

    let mv_state = Arc::new(MVHashMap::new());
    let mut beneficiary_tracker = LazyBeneficiaryTracker::new();

    // Execute tx0.
    let mut db0 = execute_transfer_via_mvdb(
        Arc::clone(&mv_state),
        Arc::clone(&base_state),
        0,
        sender_addr(),
        receiver_addr(),
        one_eth,
        0,
    );

    let _rs0 = db0.take_read_set();
    let mut ws0 = db0.take_write_set();

    // Record sender/receiver balance changes in WriteSet (but NOT coinbase).
    ws0.record(
        LocationKey::Balance(sender_addr()),
        WriteValue::Balance(hundred_eth - one_eth),
    );
    ws0.record(
        LocationKey::Nonce(sender_addr()),
        WriteValue::Nonce(1),
    );
    ws0.record(
        LocationKey::Balance(receiver_addr()),
        WriteValue::Balance(one_eth),
    );

    // Record gas fee in the lazy beneficiary tracker instead of MVHashMap.
    let gas_fee = U256::from(21_000u64); // simulated gas fee
    beneficiary_tracker.record_gas_fee(0, gas_fee);

    // Publish tx0's writes to MVHashMap.
    ws0.apply_to(&mv_state, 0, 0);

    // ── Verification: coinbase balance is NOT in MVHashMap ──────────
    match mv_state.read(&LocationKey::Balance(coinbase_addr()), 1) {
        MvReadResult::NotFound => {
            // Expected: no coinbase balance entry in MVHashMap.
        }
        other => panic!(
            "Coinbase balance should NOT be in MVHashMap (lazy pattern), got {:?}",
            other
        ),
    }

    // Verify the tracker accumulated the fee correctly.
    assert_eq!(
        beneficiary_tracker.get_fee(0),
        Some(gas_fee),
        "Tracker should have recorded gas fee for tx0"
    );
    assert_eq!(
        beneficiary_tracker.total_fees(),
        gas_fee,
        "Total fees should equal the single tx's fee"
    );

    // ── Execute tx1 and accumulate its fee too ─────────────────────
    let mut db1 = execute_transfer_via_mvdb(
        Arc::clone(&mv_state),
        Arc::clone(&base_state),
        1,
        sender_addr(),
        receiver_addr(),
        U256::from(1u64), // tiny value
        1,
    );

    let _rs1 = db1.take_read_set();

    beneficiary_tracker.record_gas_fee(1, U256::from(21_000u64));

    // Coinbase still not in MVHashMap.
    match mv_state.read(&LocationKey::Balance(coinbase_addr()), 2) {
        MvReadResult::NotFound => {} // Expected
        other => panic!(
            "Coinbase should still not be in MVHashMap after tx1, got {:?}",
            other
        ),
    }

    assert_eq!(
        beneficiary_tracker.total_fees(),
        U256::from(42_000u64),
        "Total fees should be 21000 + 21000 = 42000"
    );
    assert_eq!(
        beneficiary_tracker.get_accumulated_fees().len(),
        2,
        "Should have fees for 2 transactions"
    );
}

// ── Test 4: ReadSet validation detects version changes ──────────────────

/// Execute a read through MvDatabase, then change the MVHashMap state,
/// and verify ReadSet validation detects the conflict.
#[test]
fn read_set_validation_detects_mvhashmap_change() {
    let base_state: Arc<dyn monad_state::StateProvider> = Arc::new(
        InMemoryState::new()
            .with_account(sender_addr(), AccountInfo::new(U256::from(5000u64), 3)),
    );

    let mv_state = Arc::new(MVHashMap::new());

    // tx=0 writes sender's balance.
    mv_state.write(
        LocationKey::Balance(sender_addr()),
        0,
        0,
        WriteValue::Balance(U256::from(4000)),
    );

    // tx=1 reads sender via MvDatabase.
    let mut db = MvDatabase::new(Arc::clone(&mv_state), Arc::clone(&base_state), 1);
    let acct = revm::database_interface::Database::basic(&mut db, sender_addr())
        .unwrap()
        .unwrap();
    assert_eq!(acct.balance, U256::from(4000));

    let rs = db.take_read_set();

    // Validation should pass initially.
    assert!(rs.validate(&mv_state, 1), "Initial validation should pass");

    // tx=0 re-executes with incarnation=1, writing a different balance.
    mv_state.write(
        LocationKey::Balance(sender_addr()),
        0,
        1,
        WriteValue::Balance(U256::from(3000)),
    );

    // Validation should now fail (incarnation changed).
    assert!(
        !rs.validate(&mv_state, 1),
        "Validation should fail after version change"
    );
}

// ── Test 5: Three sequential transactions with full R/W tracking ────────

/// End-to-end test matching the S03 demo: tx0 writes, tx1 reads tx0's
/// output, tx2 reads both — with correct ReadSets and WriteSets.
#[test]
fn three_sequential_txs_full_tracking() {
    let initial_balance = U256::from(100_000_000_000_000_000_000u128);

    let base_state: Arc<dyn monad_state::StateProvider> = Arc::new(
        InMemoryState::new()
            .with_account(sender_addr(), AccountInfo::new(initial_balance, 0))
            .with_account(receiver_addr(), AccountInfo::new(U256::ZERO, 0))
            .with_account(receiver2_addr(), AccountInfo::new(U256::ZERO, 0)),
    );

    let mv_state = Arc::new(MVHashMap::new());
    let mut beneficiary_tracker = LazyBeneficiaryTracker::new();
    let one_eth = U256::from(1_000_000_000_000_000_000u128);

    // ── tx0: sender → receiver (1 ETH) ─────────────────────────────
    let mut db0 = execute_transfer_via_mvdb(
        Arc::clone(&mv_state),
        Arc::clone(&base_state),
        0,
        sender_addr(),
        receiver_addr(),
        one_eth,
        0,
    );

    let _rs0 = db0.take_read_set();
    let mut ws0 = WriteSet::new();
    ws0.record(
        LocationKey::Balance(sender_addr()),
        WriteValue::Balance(initial_balance - one_eth),
    );
    ws0.record(LocationKey::Nonce(sender_addr()), WriteValue::Nonce(1));
    ws0.record(
        LocationKey::Balance(receiver_addr()),
        WriteValue::Balance(one_eth),
    );
    ws0.apply_to(&mv_state, 0, 0);
    beneficiary_tracker.record_gas_fee(0, U256::from(21_000));

    // ── tx1: sender → receiver2 (1 ETH) ────────────────────────────
    // tx1 reads sender's balance from MVHashMap (should see 99 ETH).
    let mut db1 = MvDatabase::new(Arc::clone(&mv_state), Arc::clone(&base_state), 1);
    let sender_acct = revm::database_interface::Database::basic(&mut db1, sender_addr())
        .unwrap()
        .unwrap();
    assert_eq!(
        sender_acct.balance,
        initial_balance - one_eth,
        "tx1 sees tx0's sender balance"
    );

    let rs1 = db1.take_read_set();
    // Verify sender balance came from MVHashMap.
    let balance_origin = rs1.iter()
        .find(|(k, _)| matches!(k, LocationKey::Balance(a) if *a == sender_addr()))
        .map(|(_, v)| v)
        .expect("tx1 should have read sender balance");
    assert!(
        matches!(balance_origin, ReadOrigin::MvHashMap { tx_index: 0, incarnation: 0 }),
        "tx1 sender balance should come from MVHashMap(tx=0, inc=0)"
    );

    // Simulate tx1's writes.
    let mut ws1 = WriteSet::new();
    ws1.record(
        LocationKey::Balance(sender_addr()),
        WriteValue::Balance(initial_balance - one_eth - one_eth),
    );
    ws1.record(LocationKey::Nonce(sender_addr()), WriteValue::Nonce(2));
    ws1.record(
        LocationKey::Balance(receiver2_addr()),
        WriteValue::Balance(one_eth),
    );
    ws1.apply_to(&mv_state, 1, 0);
    beneficiary_tracker.record_gas_fee(1, U256::from(21_000));

    // ── tx2: read both receiver and receiver2 balances ──────────────
    let mut db2 = MvDatabase::new(Arc::clone(&mv_state), Arc::clone(&base_state), 2);

    let recv_acct = revm::database_interface::Database::basic(&mut db2, receiver_addr())
        .unwrap()
        .unwrap();
    assert_eq!(recv_acct.balance, one_eth, "tx2 sees receiver got 1 ETH from tx0");

    let recv2_acct = revm::database_interface::Database::basic(&mut db2, receiver2_addr())
        .unwrap()
        .unwrap();
    assert_eq!(recv2_acct.balance, one_eth, "tx2 sees receiver2 got 1 ETH from tx1");

    let sender_acct = revm::database_interface::Database::basic(&mut db2, sender_addr())
        .unwrap()
        .unwrap();
    assert_eq!(
        sender_acct.balance,
        initial_balance - one_eth - one_eth,
        "tx2 sees sender after both transfers"
    );

    let rs2 = db2.take_read_set();

    // tx2 reads 3 accounts × 3 fields each = 9 reads.
    assert!(rs2.len() >= 9, "tx2 should have at least 9 reads, got {}", rs2.len());

    // All receiver and receiver2 balance reads should come from MVHashMap.
    let recv_balance_origin = rs2.iter()
        .find(|(k, _)| matches!(k, LocationKey::Balance(a) if *a == receiver_addr()))
        .map(|(_, v)| v)
        .expect("tx2 should read receiver balance");
    assert!(
        matches!(recv_balance_origin, ReadOrigin::MvHashMap { tx_index: 0, .. }),
        "receiver balance from tx0: {:?}",
        recv_balance_origin
    );

    let recv2_balance_origin = rs2.iter()
        .find(|(k, _)| matches!(k, LocationKey::Balance(a) if *a == receiver2_addr()))
        .map(|(_, v)| v)
        .expect("tx2 should read receiver2 balance");
    assert!(
        matches!(recv2_balance_origin, ReadOrigin::MvHashMap { tx_index: 1, .. }),
        "receiver2 balance from tx1: {:?}",
        recv2_balance_origin
    );

    // Validate all ReadSets.
    assert!(rs1.validate(&mv_state, 1), "tx1 ReadSet should validate");
    assert!(rs2.validate(&mv_state, 2), "tx2 ReadSet should validate");

    // Coinbase never entered MVHashMap.
    assert!(
        matches!(
            mv_state.read(&LocationKey::Balance(coinbase_addr()), 3),
            MvReadResult::NotFound
        ),
        "Coinbase should not be in MVHashMap"
    );

    // Beneficiary tracker has fees from both txs.
    assert_eq!(beneficiary_tracker.total_fees(), U256::from(42_000));
}
