---
id: T02
parent: S02
milestone: M006
provides:
  - decodeSlotToVariable() — hex runtime slot → variable name decoder with exact match and mapping heuristic
  - buildConflictAnalysis() — full conflict analysis orchestrator with coinbase filtering and Storage-type filtering
  - generateSuggestion() — actionable English suggestion generator per variable type
  - buildMatrix() — function × variable conflict count matrix builder
  - 17 passing unit tests for the decoder module
key_files:
  - Vibe-Room-Backend/src/vibe-score/storage-layout-decoder.ts
  - Vibe-Room-Backend/test/storage-layout-decoder.spec.ts
key_decisions:
  - Pure function module with no NestJS DI dependency — all functions exported for direct testing and downstream wiring
  - BigInt normalization for hex/decimal slot comparison — handles both solc decimal strings and CLI hex strings
  - Mapping heuristic uses "slot > max declared slot" threshold — single mapping gets exact attribution, multiple mappings get "unknown (possibly X or Y)"
  - Suggestion text in English per D019 decision (UX text language unified to English)
patterns_established:
  - Hex slot → BigInt → exact match against decimal storageLayout entries
  - Large runtime slot heuristic for mapping/dynamic_array attribution
  - Variable grouping by name for conflict deduplication across multiple ConflictPair entries
observability_surfaces:
  - decodeSlotToVariable() returns "unknown_slot_0xNNN" on decode failure — grep API responses for "unknown_slot" to detect decode misses
  - buildConflictAnalysis() returns empty { conflicts: [], matrix: {...} } when storageLayout is undefined — no errors thrown
  - Run `npx jest test/storage-layout-decoder.spec.ts --verbose` to inspect individual test outcomes
duration: 12m
verification_result: passed
completed_at: 2026-03-24T05:38:00+09:00
blocker_discovered: false
---

# T02: storage-layout-decoder 모듈 구현 + 단위 테스트

**Implemented pure-function storage-layout-decoder module with hex→decimal slot decoding, mapping heuristic, coinbase filtering, suggestion generation, and matrix builder — all 17 unit tests pass**

## What Happened

Built the core domain logic module `storage-layout-decoder.ts` as a pure TypeScript module with no NestJS DI dependency. Implemented all four exported functions:

1. **decodeSlotToVariable()**: Converts hex runtime slots (`0x0`) to variable names by comparing against solc storageLayout's decimal slot strings (`"0"`) via BigInt normalization. Falls back to mapping heuristic for large runtime slots (keccak256-derived), attributing to a single mapping or reporting "unknown (possibly X or Y)" for multiple mappings. Returns `unknown_slot_0xNNN` as final fallback.

2. **buildConflictAnalysis()**: Main orchestrator that filters coinbase address conflicts (case-insensitive), keeps only Storage-type conflicts, decodes each slot, maps tx indices to function names via txFunctionMap, groups conflicts by variable, attaches suggestions, and builds the matrix. Returns empty result when storageLayout is undefined (graceful degradation).

3. **generateSuggestion()**: Generates English actionable suggestions based on variable type — mapping-specific (key range separation), simple variable (per-function splitting), dynamic array (mapping-based structure), and unknown slot (verify storage layout).

4. **buildMatrix()**: Builds a function × variable 2D matrix with conflict counts at each intersection.

Created 17 comprehensive unit tests covering exact slot match, mapping heuristic (single and multiple), unknown fallback, coinbase filtering, non-Storage conflict skipping, storageLayout undefined, suggestion content for all types, matrix dimensions/counts, full ParallelConflict-like integration, and tx index resolution.

## Verification

- `npx jest test/storage-layout-decoder.spec.ts` — 17/17 tests pass
- ParallelConflict fixture: slot `"0x0"` → `"counter"` decoded, suggestion contains "counter", "increment", "incrementBy"
- `npx jest test/storage-layout-decoder.spec.ts -- --testNamePattern="undefined|unknown"` — failure-path tests pass (storageLayout undefined → empty result, unknown slot → fallback name)
- Existing slice tests unaffected: compile.service.spec.ts (10 pass), vibe-score.service.spec.ts (13 pass)

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cd Vibe-Room-Backend && npx jest test/storage-layout-decoder.spec.ts --verbose` | 0 | ✅ pass (17/17) | 3.6s |
| 2 | `cd Vibe-Room-Backend && npx jest test/storage-layout-decoder.spec.ts -- --testNamePattern="undefined\|unknown"` | 0 | ✅ pass (17/17) | 2.9s |
| 3 | `cd Vibe-Room-Backend && npx jest test/compile.service.spec.ts` | 0 | ✅ pass (10/10) | 5.6s |
| 4 | `cd Vibe-Room-Backend && npx jest test/vibe-score.service.spec.ts` | 0 | ✅ pass (13/13) | 6.1s |

### Slice-level verification (partial — T02 is intermediate)

| # | Slice Check | Status |
|---|------------|--------|
| 1 | `npx jest test/compile.service.spec.ts` — storageLayout 추출 테스트 | ✅ pass (10/10) |
| 2 | `npx jest test/storage-layout-decoder.spec.ts` — decoder 모듈 8+ 테스트 | ✅ pass (17/17) |
| 3 | `npx jest test/vibe-score.service.spec.ts` — conflictAnalysis 통합 테스트 | ⏳ not yet wired (T03) |
| 4 | failure-path verification (undefined/unknown) | ✅ pass |

## Diagnostics

- Run `npx jest test/storage-layout-decoder.spec.ts --verbose` to see all 17 test names and results.
- All functions are pure exports — can be imported and called directly in a REPL or downstream test for debugging.
- To verify slot decoding: `decodeSlotToVariable('0x0', layout)` should return `{ variableName: 'counter', ... }` for ParallelConflict layout.
- To detect decode failures in API responses: grep for `unknown_slot` in `conflictAnalysis.conflicts[].variableName`.

## Deviations

- Added 17 tests instead of the planned 9 — additional coverage for edge cases (exact match with larger hex, multi-mapping heuristic, array suggestion, matrix intersection counting, empty matrix, tx index resolution fallback) at zero cost.
- Added Observability Impact section to T02-PLAN.md as required by pre-flight gap check.

## Known Issues

None.

## Files Created/Modified

- `Vibe-Room-Backend/src/vibe-score/storage-layout-decoder.ts` — New: pure-function decoder module with 4 exported functions (decodeSlotToVariable, buildConflictAnalysis, generateSuggestion, buildMatrix)
- `Vibe-Room-Backend/test/storage-layout-decoder.spec.ts` — New: 17 unit tests covering all decoder functions, edge cases, and failure paths
