# S01: Rust CLI — R/W Set 충돌 데이터 JSON 출력 — Research

**Date:** 2026-03-24
**Depth:** Deep (core scheduler modification + new serialization layer)

## Summary

S01 extends the monad-core CLI to output `conflict_details` alongside the existing `results`, `incarnations`, and `stats` fields. The primary risk — ReadSet preservation after validation — is tractable. The current flow in `parallel_executor.rs::handle_validate()` calls `scheduler.take_read_set(tx_idx)`, which moves the ReadSet out of `TxState` for validation. After validation, the ReadSet is dropped (goes out of scope). The fix is surgical: return the ReadSet to `TxState` after validation succeeds, then collect it in `collect_results()`.

The second half of the work is purely additive: building conflict detection logic and serializable output types in the CLI. No external libraries needed — this is iteration over `BTreeMap<LocationKey, _>` to find overlapping keys between tx pairs.

**Requirement coverage:** R005 (CLI binary + JSON interface — extends existing), R006 (Vibe Score — produces conflict data that S02 will decode into variable names), R017 (parallel execution optimization — this is the data source).

## Recommendation

**Approach: Modify scheduler to preserve ReadSets, build conflict analysis in CLI.**

1. **Scheduler layer** — Add `return_read_set()` to `Scheduler`, call it from `handle_validate()` after validation passes. Extend `collect_results()` to return `Vec<(ExecutionResult, WriteSet, ReadSet)>`. Update `ParallelExecutionResult` to include `read_sets: Vec<ReadSet>`.

2. **CLI layer** — Define serializable `ConflictDetails` types (separate from internal `LocationKey`/`WriteValue`). After execution, iterate over all ReadSets/WriteSets to detect (a) write-write conflicts and (b) read-write conflicts. Append to `CliOutput` as `conflict_details`.

This approach is preferred over alternatives:
- **Alternative: Clone ReadSet before take** — wasteful; ReadSets can be large. Returning after use avoids the clone.
- **Alternative: Add Serialize to LocationKey/WriteValue** — couples internal types to serialization format. CLI-specific types give freedom to evolve the JSON schema independently.
- **Alternative: Detect conflicts during execution** — the coordinator already knows about conflicts (incarnation > 0), but it doesn't record *which* locations conflicted. Post-execution analysis from ReadSets/WriteSets is simpler and doesn't add overhead to the hot path.

## Implementation Landscape

### Key Files

- `crates/scheduler/src/parallel_executor.rs` (240 lines) — `ParallelExecutionResult` struct needs `read_sets: Vec<ReadSet>` field. `handle_validate()` function (line ~190) needs to return ReadSet to TxState after successful validation. `execute_block_parallel()` needs to collect ReadSets in `collect_results()` call.

- `crates/scheduler/src/coordinator.rs` (290 lines) — `Scheduler` needs new `return_read_set(tx_index, read_set)` method. `collect_results()` (line ~175) currently returns `Vec<(ExecutionResult, WriteSet)>` — change to `Vec<(ExecutionResult, WriteSet, ReadSet)>`.

- `crates/scheduler/src/types.rs` (75 lines) — No structural change needed. `TxState.read_set: Option<ReadSet>` already exists. The return_read_set method just puts it back.

- `crates/mv-state/src/types.rs` (95 lines) — `LocationKey` enum (`Storage(Address, U256)`, `Balance(Address)`, `Nonce(Address)`, `CodeHash(Address)`). No modification needed — CLI will pattern-match on these to produce serializable output.

- `crates/mv-state/src/read_write_sets.rs` (443 lines) — `ReadSet` (BTreeMap<LocationKey, ReadOrigin>) and `WriteSet` (BTreeMap<LocationKey, WriteValue>) with `.iter()` methods. No modification needed.

- `crates/cli/src/main.rs` (245 lines) — `CliOutput` struct (line ~60) gains `conflict_details` field. New conflict analysis module added. The existing result-mapping loop (line ~165) needs to destructure 3-tuples instead of 2-tuples.

- `crates/scheduler/src/worker.rs` (275 lines) — No change needed. `execute_transaction()` already returns `ReadSet` in `ExecutionOutcome::Success`. `validate_transaction()` already takes `&ReadSet` (borrow, not move).

### Critical Data Flow (current → modified)

**Current flow:**
```
execute_transaction() → ExecutionOutcome::Success { read_set, ... }
  → handle_execute() → scheduler.finish_execution(read_set, write_set, result)
    → TxState.read_set = Some(read_set)
  → handle_validate() → scheduler.take_read_set() → ReadSet moved out
    → validate_transaction(&read_set) → bool
    → read_set DROPPED (out of scope)  ← THIS IS THE PROBLEM
  → collect_results() → Vec<(ExecutionResult, WriteSet)>  ← NO ReadSet
```

**Modified flow:**
```
  → handle_validate() → scheduler.take_read_set() → ReadSet moved out
    → validate_transaction(&read_set) → bool
    → if valid: scheduler.return_read_set(tx_idx, read_set)  ← NEW
    → if invalid: read_set dropped (tx will re-execute anyway)
  → collect_results() → Vec<(ExecutionResult, WriteSet, ReadSet)>  ← INCLUDES ReadSet
```

### Conflict Detection Algorithm (CLI-side, post-execution)

```
For each pair (tx_a, tx_b) where tx_a < tx_b:
  1. write-write: intersection of write_set_a.keys() ∩ write_set_b.keys()
  2. read-write:  intersection of read_set_a.keys() ∩ write_set_b.keys()
  3. write-read:  intersection of write_set_a.keys() ∩ read_set_b.keys()
```

Complexity: O(n² × m) where n = tx count, m = avg set size. For typical blocks (5-50 txs, ~10-50 entries per set), this is negligible (< 1ms).

### Output JSON Schema (from ROADMAP)

```json
{
  "results": [...],
  "incarnations": [...],
  "stats": {...},
  "conflict_details": {
    "per_tx": [
      {
        "tx_index": 0,
        "reads": [
          { "location_type": "Storage", "address": "0x...", "slot": "0x..." },
          { "location_type": "Balance", "address": "0x..." }
        ],
        "writes": [
          { "location_type": "Storage", "address": "0x...", "slot": "0x...", "value_type": "Storage" },
          { "location_type": "Nonce", "address": "0x...", "value_type": "Nonce" }
        ]
      }
    ],
    "conflicts": [
      {
        "location": { "type": "Storage", "address": "0x...", "slot": "0x..." },
        "tx_a": 0,
        "tx_b": 1,
        "conflict_type": "write-write"
      }
    ]
  }
}
```

### Build Order

1. **Scheduler ReadSet preservation (FIRST — riskiest, unblocks everything)**
   - Add `return_read_set()` to `Scheduler` in `coordinator.rs`
   - Modify `handle_validate()` in `parallel_executor.rs` to call `return_read_set()` after successful validation
   - Change `collect_results()` return type to include `ReadSet`
   - Update `ParallelExecutionResult` to include `read_sets: Vec<ReadSet>`
   - **Verification:** `cargo test -p monad-scheduler` — all 19 existing tests must pass (no regression). New test: `test_read_set_preserved_after_validation` that verifies ReadSet is accessible in `collect_results()`.

2. **CLI conflict analysis types + detection logic (SECOND — additive, no risk to existing code)**
   - New serializable structs: `ConflictDetails`, `TxAccessSummary`, `LocationInfo`, `ConflictPair`
   - Conflict detection function: takes `&[(ExecutionResult, WriteSet, ReadSet)]` → `ConflictDetails`
   - Pattern-match `LocationKey` → `LocationInfo` serializable representation
   - **Verification:** Unit test with known ReadSet/WriteSet pairs → verify conflict detection correctness.

3. **CLI output integration (THIRD — wiring)**
   - Update `CliOutput` to include `conflict_details: ConflictDetails`
   - Destructure 3-tuples in result mapping loop
   - Call conflict detection after execution
   - **Verification:** `echo '...' | cargo run -p monad-cli` with a test payload → verify JSON output contains `conflict_details`.

### Verification Approach

1. **Unit tests (scheduler):**
   - `cargo test -p monad-scheduler` — existing 19 tests pass (regression guard)
   - New test: execute 2 conflicting value transfers (same sender), verify ReadSets are present in `ParallelExecutionResult`

2. **Unit tests (CLI conflict detection):**
   - Construct synthetic ReadSets/WriteSets with known overlaps
   - Verify `detect_conflicts()` produces correct `ConflictPair` entries
   - Verify serialization round-trips correctly

3. **Integration test (CLI end-to-end):**
   - Pipe a JSON block with conflicting transactions through `monad-cli`
   - Parse output JSON, assert `conflict_details.conflicts` is non-empty
   - Pipe a JSON block with independent transactions, assert `conflict_details.conflicts` is empty

4. **Regression:**
   - `cargo test --workspace` — full workspace must pass
   - Existing CLI output fields (`results`, `incarnations`, `stats`) unchanged

## Constraints

- **No serde on internal types.** `LocationKey`, `WriteValue`, `ReadOrigin` in `crates/mv-state/src/types.rs` should NOT get `Serialize` derives. The CLI defines its own serializable types and converts. This preserves the clean separation between internal execution types and external API.
- **mv-state Cargo.toml has no serde dependency** and adding one just for serialization would be wrong — the mv-state crate is the hot path for parallel execution.
- **ReadSet/WriteSet use BTreeMap** (deterministic ordering). The CLI's conflict detection output will also be deterministic.
- **`finish_validation(tx_index, false)` clears read_set to None.** The return_read_set call must happen ONLY for valid txs. For invalid txs, the tx re-executes and gets a new ReadSet.
- **Thread safety:** `return_read_set()` locks `tx_states[tx_index]` mutex (same pattern as existing methods). No new concurrency concerns.

## Common Pitfalls

- **Returning ReadSet for aborted txs** — If `handle_validate()` returns the ReadSet to TxState BEFORE calling `finish_validation(valid=false)`, the ReadSet gets cleared by finish_validation anyway. But if it happens AFTER, the read_set field would be None (already cleared). Solution: only call `return_read_set()` when `valid == true`, before `finish_validation()`.

- **collect_results() called before all txs validated** — The current code panics if a result is missing. Same applies to ReadSets. For error txs (`finish_execution_with_error()`), an empty ReadSet is stored. The collect path must handle `None` ReadSet gracefully (use `ReadSet::default()`).

- **3-tuple destructuring breaks existing test code** — `parallel_executor.rs` tests destructure `par_result.tx_results` as `(exec_result, write_set)`. After changing to 3-tuples, all test code that pattern-matches on `tx_results` must be updated. There are ~5 test functions in `parallel_executor.rs::tests`.

- **CLI output size** — Per-tx ReadSet/WriteSet data could be large for contract-heavy blocks. The roadmap notes this but defers optimization. Current scope (5-50 txs for Vibe Score analysis) is fine.

## Open Risks

- **`handle_validate` is in the hot loop** — Adding `return_read_set()` call adds one mutex lock per validated tx. This is the same cost as the existing `take_read_set()` call, so impact is negligible. But if profiling later shows contention, this could be revisited.

- **ReadSet for re-executed txs** — A tx that gets re-executed multiple times (incarnation > 1) has its ReadSet from the FINAL successful execution. Earlier incarnation ReadSets are lost. This is correct behavior (only the final execution's reads matter for conflict analysis), but worth documenting.
