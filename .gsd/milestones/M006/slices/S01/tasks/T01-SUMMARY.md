---
id: T01
parent: S01
milestone: M006
provides:
  - Scheduler::return_read_set() method to preserve ReadSets after successful validation
  - collect_results() returns Vec<(ExecutionResult, WriteSet, ReadSet)> 3-tuple
  - ParallelExecutionResult::tx_results updated to 3-tuple
  - test_read_set_preserved_after_validation test confirming ReadSet contains Balance + Nonce reads
key_files:
  - crates/scheduler/src/coordinator.rs
  - crates/scheduler/src/parallel_executor.rs
  - crates/cli/src/main.rs
key_decisions:
  - ReadSet is returned to TxState only on validation success; on failure the tx re-executes and gets a new ReadSet
  - collect_results() uses unwrap_or_default() for missing ReadSets, making data loss explicitly observable as empty ReadSets
patterns_established:
  - return_read_set() mirrors take_read_set() using the same mutex pattern for thread-safe TxState access
observability_surfaces:
  - ReadSet emptiness in collect_results() output is explicit — empty ReadSet means data was not preserved
  - test_read_set_preserved_after_validation verifies Balance and Nonce LocationKey presence
duration: 15m
verification_result: passed
completed_at: 2026-03-24
blocker_discovered: false
---

# T01: Preserve ReadSets in scheduler after validation and extend collect_results

**Added return_read_set() to Scheduler and extended collect_results() to 3-tuple (ExecutionResult, WriteSet, ReadSet) for downstream conflict analysis**

## What Happened

Implemented the five planned steps to preserve ReadSets through the Block-STM validation pipeline:

1. Added `Scheduler::return_read_set(tx_index, read_set)` method in coordinator.rs — the inverse of `take_read_set()`, using the same mutex-based TxState access pattern.

2. Modified `handle_validate()` in parallel_executor.rs to call `return_read_set()` when validation succeeds, restoring the ReadSet to TxState. On validation failure, the ReadSet is intentionally dropped since the tx will re-execute and produce a new one.

3. Changed `collect_results()` return type from `Vec<(ExecutionResult, WriteSet)>` to `Vec<(ExecutionResult, WriteSet, ReadSet)>`, adding `state.read_set.take().unwrap_or_default()` to collect ReadSets alongside existing results.

4. Updated `ParallelExecutionResult::tx_results` type to the 3-tuple and adjusted the module-level observability documentation.

5. Updated the `test_parallel_independent_transfers` destructure pattern from 2-tuple to 3-tuple (`_read_set`), and added `test_read_set_preserved_after_validation` which verifies that ReadSets contain Balance and Nonce LocationKeys after value transfer execution.

6. Updated CLI main.rs destructure from `(exec_result, _write_set)` to `(exec_result, _write_set, _read_set)` for compile compatibility.

## Verification

- `cargo test -p monad-scheduler`: 25 tests passed (24 existing + 1 new `test_read_set_preserved_after_validation`)
- `cargo build -p monad-cli`: Clean build with 3-tuple compatibility
- Empty block diagnostic check: Passed (CLI returns empty results and stats correctly)

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p monad-scheduler` | 0 | ✅ pass | 17.3s |
| 2 | `cargo build -p monad-cli` | 0 | ✅ pass | 13.1s |
| 3 | `echo '...' \| cargo run -p monad-cli 2>/dev/null \| python3 -c "...EMPTY_OK..."` | 0 | ✅ pass | <5s |
| 4 | `cargo test -p monad-cli` (slice-level, T02 scope) | — | ⏳ deferred to T02 | — |
| 5 | Integration check (slice-level, T02 scope) | — | ⏳ deferred to T02 | — |

## Diagnostics

- Run `cargo test -p monad-scheduler test_read_set_preserved_after_validation` to verify ReadSet preservation is working.
- If ReadSets appear empty in downstream conflict analysis, check that `handle_validate()` calls `return_read_set()` on the success path.
- `collect_results()` uses `unwrap_or_default()` — empty ReadSet in output means the ReadSet was never stored or was cleared by a validation failure.

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `crates/scheduler/src/coordinator.rs` — Added `return_read_set()` method, changed `collect_results()` to return 3-tuple with ReadSet
- `crates/scheduler/src/parallel_executor.rs` — Updated `ParallelExecutionResult::tx_results` to 3-tuple, modified `handle_validate()` to preserve ReadSet on success, updated test patterns, added `test_read_set_preserved_after_validation`
- `crates/cli/src/main.rs` — Updated destructure pattern from 2-tuple to 3-tuple for compile compatibility
