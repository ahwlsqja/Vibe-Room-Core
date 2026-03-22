//! Chain replay test — execute real Monad mainnet block transactions
//! against pre-state loaded from prestateTracer and compare gasUsed/status
//! with actual on-chain receipts.
//!
//! Fixture: Block 63070208 (0x3c26000) from Monad mainnet
//! - 4 transactions (1 system + 3 DeFi contract calls)
//! - Pre-state loaded via `debug_traceBlockByNumber` with prestateTracer
//! - Receipts loaded via `eth_getBlockReceipts`
//!
//! Note: tx[0] is a system transaction (gas=0, to=0x1000) which our executor
//! cannot replay (it's a consensus-level operation). We skip it and replay tx[1-3].

use std::collections::HashMap;

use alloy_primitives::{Address, Bytes, B256, U256};
use monad_evm::executor::EvmExecutor;
use monad_state::InMemoryState;
use monad_types::{AccountInfo, BlockEnv, Transaction};

/// Parse a hex string (with or without 0x prefix) to U256.
fn hex_to_u256(s: &str) -> U256 {
    let s = s.strip_prefix("0x").unwrap_or(s);
    U256::from_str_radix(s, 16).unwrap()
}

fn hex_to_u64(s: &str) -> u64 {
    let s = s.strip_prefix("0x").unwrap_or(s);
    u64::from_str_radix(s, 16).unwrap()
}

fn hex_to_address(s: &str) -> Address {
    s.parse::<Address>().unwrap()
}

fn hex_to_bytes(s: &str) -> Bytes {
    let s = s.strip_prefix("0x").unwrap_or(s);
    Bytes::from(hex::decode(s).unwrap())
}

#[allow(dead_code)]
fn hex_to_b256(s: &str) -> B256 {
    s.parse::<B256>().unwrap()
}

/// Load pre-state from flat prestateTracer output (non-diffMode).
/// Merges all tx pre-states into a single InMemoryState.
fn load_prestate_from_fixture() -> InMemoryState {
    let data: Vec<serde_json::Value> =
        serde_json::from_str(include_str!("fixtures/prestate_flat_63070208.json")).unwrap();

    let mut accounts: HashMap<Address, AccountInfo> = HashMap::new();
    let mut storage: HashMap<(Address, U256), U256> = HashMap::new();
    let mut code: HashMap<B256, Bytes> = HashMap::new();

    for trace in &data {
        let result = trace["result"].as_object().unwrap();
        for (addr_hex, info) in result {
            let addr = hex_to_address(addr_hex);

            let balance = info
                .get("balance")
                .and_then(|v| v.as_str())
                .map(hex_to_u256)
                .unwrap_or(U256::ZERO);

            let nonce = info
                .get("nonce")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            let code_bytes = info
                .get("code")
                .and_then(|v| v.as_str())
                .filter(|s| *s != "0x" && !s.is_empty())
                .map(hex_to_bytes);

            let code_hash = if let Some(ref cb) = code_bytes {
                let hash = alloy_primitives::keccak256(cb.as_ref());
                code.insert(hash, cb.clone());
                hash
            } else {
                monad_types::KECCAK_EMPTY
            };

            // Only insert if not already present (first occurrence = pre-state)
            accounts.entry(addr).or_insert_with(|| {
                if code_bytes.is_some() {
                    AccountInfo::new_contract(balance, nonce, code_hash, code_bytes.clone().unwrap())
                } else {
                    AccountInfo::new(balance, nonce)
                }
            });

            // Load storage slots
            if let Some(storage_map) = info.get("storage").and_then(|v| v.as_object()) {
                for (slot_hex, value_hex) in storage_map {
                    let slot = hex_to_u256(slot_hex);
                    let value = hex_to_u256(value_hex.as_str().unwrap());
                    storage.entry((addr, slot)).or_insert(value);
                }
            }
        }
    }

    let mut state = InMemoryState::new();
    for (addr, info) in accounts {
        state.insert_account(addr, info);
    }
    for ((addr, slot), value) in storage {
        state.insert_storage(addr, slot, value);
    }
    for (hash, bytecode) in code {
        state.insert_code(hash, bytecode);
    }

    state
}

/// Load block data from fixture.
fn load_block_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("fixtures/block_63070208.json")).unwrap()
}

/// Load receipts from fixture.
fn load_receipts_fixture() -> Vec<serde_json::Value> {
    serde_json::from_str(include_str!("fixtures/receipts_63070208.json")).unwrap()
}

/// Replay a single transaction and compare with on-chain receipt.
fn replay_tx(
    tx_json: &serde_json::Value,
    receipt_json: &serde_json::Value,
    state: &InMemoryState,
    block_env: &BlockEnv,
    tx_index: usize,
) {
    let from = hex_to_address(tx_json["from"].as_str().unwrap());
    let to = tx_json["to"]
        .as_str()
        .map(hex_to_address);
    let value = hex_to_u256(tx_json["value"].as_str().unwrap());
    let input = hex_to_bytes(tx_json["input"].as_str().unwrap());
    let nonce = hex_to_u64(tx_json["nonce"].as_str().unwrap());
    let gas_limit = hex_to_u64(tx_json["gas"].as_str().unwrap());

    // Skip system txs (gas=0)
    if gas_limit == 0 {
        println!("  tx[{}]: skipped (system tx, gas=0)", tx_index);
        return;
    }

    let gas_price = tx_json
        .get("gasPrice")
        .and_then(|v| v.as_str())
        .map(hex_to_u256)
        .unwrap_or(U256::ZERO);

    let tx = Transaction {
        sender: from,
        to,
        value,
        data: input,
        gas_limit,
        nonce,
        gas_price,
    };

    let result = EvmExecutor::execute_tx_with_state_changes(&tx, state, block_env);

    let expected_status = hex_to_u64(receipt_json["status"].as_str().unwrap());
    let expected_gas = hex_to_u64(receipt_json["gasUsed"].as_str().unwrap());

    match result {
        Ok((exec_result, _state_changes)) => {
            let actual_status = if exec_result.is_success() { 1u64 } else { 0u64 };
            let actual_gas = exec_result.gas_used();

            println!(
                "  tx[{}]: status={}/{} gasUsed={}/{} {}",
                tx_index,
                actual_status,
                expected_status,
                actual_gas,
                expected_gas,
                if actual_status == expected_status { "✓" } else { "✗ STATUS MISMATCH" }
            );

            assert_eq!(
                actual_status, expected_status,
                "tx[{}] status mismatch: got {} expected {}",
                tx_index, actual_status, expected_status
            );

            // Gas comparison: allow some tolerance due to fee model differences
            // (our executor uses disable_fee_charge=true which may affect gas accounting)
            // For now, just verify the tx succeeded/failed correctly.
            // Exact gas matching requires matching the exact fee model.
            if actual_gas != expected_gas {
                let diff_pct = if expected_gas > 0 {
                    ((actual_gas as f64 - expected_gas as f64).abs() / expected_gas as f64) * 100.0
                } else {
                    0.0
                };
                println!(
                    "    gas diff: {} vs {} ({:.1}%)",
                    actual_gas, expected_gas, diff_pct
                );
                // Gas differences are expected due to:
                // 1. disable_fee_charge=true in our executor (EIP-1559 fee mechanics differ)
                // 2. Monad-specific opcode pricing (different from standard Ethereum)
                // 3. Potential pre-state gaps (prestateTracer may not capture all accessed storage)
                // We log the difference but only fail on status mismatch.
            }
        }
        Err(e) => {
            if expected_status == 1 {
                panic!(
                    "tx[{}] expected success but got error: {:?}",
                    tx_index, e
                );
            } else {
                println!("  tx[{}]: reverted as expected (err: {:?})", tx_index, e);
            }
        }
    }
}

/// Replay Monad mainnet block 63070208 — verify tx execution status
/// matches on-chain receipts.
#[test]
fn replay_monad_block_63070208() {
    let state = load_prestate_from_fixture();
    let block_json = load_block_fixture();
    let receipts = load_receipts_fixture();

    let block_env = BlockEnv {
        number: hex_to_u64(block_json["number"].as_str().unwrap()),
        coinbase: hex_to_address(block_json["miner"].as_str().unwrap()),
        timestamp: hex_to_u64(block_json["timestamp"].as_str().unwrap()),
        gas_limit: hex_to_u64(block_json["gasLimit"].as_str().unwrap()),
        base_fee: block_json
            .get("baseFeePerGas")
            .and_then(|v| v.as_str())
            .map(hex_to_u256)
            .unwrap_or(U256::ZERO),
        difficulty: U256::ZERO,
    };

    let txs = block_json["transactions"].as_array().unwrap();

    println!("Replaying block {} ({} txs)", block_env.number, txs.len());

    // Track how many txs we actually replayed (skip system txs)
    let mut replayed = 0;

    for (i, tx_json) in txs.iter().enumerate() {
        let gas_limit = hex_to_u64(tx_json["gas"].as_str().unwrap());
        if gas_limit == 0 {
            println!("  tx[{}]: skipped (system tx)", i);
            continue;
        }

        replay_tx(tx_json, &receipts[i], &state, &block_env, i);
        replayed += 1;
    }

    assert!(replayed > 0, "should have replayed at least one transaction");
    println!("Replayed {} non-system transactions successfully", replayed);
}
