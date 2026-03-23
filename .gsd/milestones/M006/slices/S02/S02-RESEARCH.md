# S02 — Research: NestJS — Storage Layout 디코딩 + Actionable Suggestion 생성

**Date:** 2026-03-24
**Depth:** Targeted — known technology (NestJS/TypeScript/solc), moderately complex domain logic (storage layout decoding), integration with well-documented S01 output

## Summary

S02 extends the Vibe-Room-Backend NestJS service to (a) extract `storageLayout` from solc compilation, (b) decode runtime storage slot addresses from CLI `conflict_details` into human-readable variable/mapping names, (c) map transaction indices to function names, and (d) generate actionable suggestions and a conflict matrix for S03's frontend heatmap.

The work is straightforward in terms of NestJS plumbing — extending `CompileService`, `EngineService` interface, and `VibeScoreService`. The main complexity is the **slot→variable decoding logic**: simple variables map directly by slot number, but mappings and dynamic arrays use `keccak256(key ++ baseSlot)` at runtime, making reverse mapping heuristic-based. Since solc `storageLayout` gives us base slots and type information, we can identify which variable a runtime slot *likely* belongs to by checking if the slot is the base slot itself or lies in the keccak256 address space of a mapping/array base slot. For the typical contract patterns this project targets (simple variables + mappings), this heuristic is accurate.

The approach: don't reverse keccak256 — instead, classify each runtime slot by checking (1) exact match to a known slot, (2) if the variable type at that base slot is `mapping` or `dynamic_array` and the runtime slot is a large number (>> max declared slot), attribute it to the mapping/array with the nearest declared base slot. This works because mapping/array runtime slots are keccak256-derived and thus astronomically larger than static variable slot numbers.

## Recommendation

**Build in 3 tasks:**

1. **CompileService extension** — Add `storageLayout` to solc `outputSelection` and `CompileResultDto`. This is a 1-line config change + interface extension. Build first because everything downstream depends on it.

2. **Storage layout decoding + suggestion generation module** — New service/utility `StorageLayoutDecoder` that takes `storageLayout` + CLI `conflict_details` and produces decoded conflict analysis + actionable suggestions. This is the core logic and should be built as a pure function (no NestJS deps) for easy unit testing.

3. **Pipeline wiring** — Extend `EngineService.CliOutput` with `conflict_details`, wire `VibeScoreService` to pass storage layout through the decode→suggest pipeline, extend `VibeScoreResultDto` with `conflictAnalysis` field.

## Implementation Landscape

### Key Files

- `Vibe-Room-Backend/src/contracts/compile.service.ts` — Modify solc `outputSelection` to include `'storageLayout'`. Add `storageLayout` to the return object. Currently returns `{ contractName, abi, bytecode }` — extend to `{ contractName, abi, bytecode, storageLayout? }`.

- `Vibe-Room-Backend/src/contracts/dto/compile-result.dto.ts` — Add `storageLayout?: StorageLayout` field. Define `StorageLayout`, `StorageEntry`, `StorageTypeInfo` TypeScript interfaces matching solc output format.

- `Vibe-Room-Backend/src/engine/engine.service.ts` — Extend `CliOutput` interface with `conflict_details?: ConflictDetails`. Must match S01's JSON schema exactly: `{ per_tx: TxAccessSummary[], conflicts: ConflictPair[] }` where `LocationInfo = { location_type: string, address: string, slot?: string }` (slot is **optional**, only present for Storage type — uses `skip_serializing_if`). `conflict_type` is `"write-write"` | `"read-write"`.

- `Vibe-Room-Backend/src/vibe-score/storage-layout-decoder.ts` — **NEW FILE.** Pure utility module with:
  - `decodeSlotToVariable(slot: string, storageLayout: StorageLayout): DecodedVariable | null` — matches a hex slot to a variable name. Logic: (1) exact slot match for simple variables, (2) for mappings/dynamic arrays whose base slots are known, check if runtime slot is a large number (> max declared slot) and heuristically attribute it to the mapping with the closest matching keccak256 pattern.
  - `buildConflictAnalysis(conflictDetails: ConflictDetails, storageLayout: StorageLayout, abi: any[], txFunctionMap: Map<number, string>, coinbaseAddress: string): ConflictAnalysis` — Main orchestrator. Filters coinbase conflicts, decodes slots, groups by variable, maps tx indices to function names, generates suggestions, builds the matrix.
  - `generateSuggestion(variable: DecodedConflict): string` — Produces specific actionable text based on variable type and conflicting functions.
  - `buildMatrix(decodedConflicts: DecodedConflict[]): ConflictMatrix` — Builds function×variable intensity matrix for S03 heatmap.

- `Vibe-Room-Backend/src/vibe-score/vibe-score.service.ts` — Modify `analyzeContract()` pipeline:
  - Phase 1 (compile) now also gets `storageLayout`
  - Phase 3 (block construction) must track which tx index maps to which function name (already has this data in the loop — just needs a `Map<number, string>`)
  - Phase 5 (score calculation) now also calls `buildConflictAnalysis()` with conflict_details from engine output + storageLayout from compilation + tx→function map
  - Must filter coinbase address conflicts per KNOWLEDGE.md rule (coinbase = `blockEnv.coinbase`)

- `Vibe-Room-Backend/src/vibe-score/dto/vibe-score-result.dto.ts` — Extend with `conflictAnalysis?: ConflictAnalysis` field. Define:
  ```typescript
  interface ConflictAnalysis {
    conflicts: DecodedConflict[];
    matrix: ConflictMatrix;
  }
  interface DecodedConflict {
    variableName: string;
    variableType: string;
    slot: string;
    functions: string[];
    conflictType: string;
    suggestion: string;
  }
  interface ConflictMatrix {
    rows: string[];  // function names
    cols: string[];  // variable names
    cells: number[][]; // intensity values (conflict count per cell)
  }
  ```

### Build Order

1. **CompileService storageLayout extraction** (lowest risk, unblocks everything) — 1-line change to `outputSelection`, interface extension. Verify with a manual `compile()` call that `storageLayout` appears in output.

2. **CliOutput interface extension + type definitions** — Add `conflict_details` to `EngineService.CliOutput`. Add all TypeScript interfaces for decoded conflicts, matrix, etc. No logic yet — just types.

3. **Storage layout decoder module** — Build and unit test `storage-layout-decoder.ts` with known storageLayout fixtures. This is the core complexity:
   - Simple variable decoding (slot "0" → `balances`, slot "1" → `totalSupply`)
   - Mapping slot heuristic (runtime slot 0x290decd9... → mapping `balances` at base slot 0)
   - Coinbase filtering
   - Suggestion generation per variable type
   - Matrix building
   Test with a ParallelConflict-like storageLayout fixture + mock conflict_details.

4. **Pipeline wiring in VibeScoreService** — Wire it all together: compilation gets layout, tx→function map built during block construction, conflict_details parsed from engine output, decoder called, result included in response. Maintain backward compat: if `conflict_details` is absent from CLI output, `conflictAnalysis` is omitted from response.

### Verification Approach

1. **Unit tests for storage-layout-decoder.ts:**
   - `decodeSlotToVariable()` with exact slot match (simple variable)
   - `decodeSlotToVariable()` with mapping heuristic (large runtime slot → mapping base)
   - `buildConflictAnalysis()` with mock data: should produce decoded conflicts with variable names
   - Coinbase address filtering: conflicts involving coinbase should be excluded
   - Suggestion text contains specific variable/function names
   - Matrix dimensions match unique functions × unique variables

2. **Unit tests for CompileService:**
   - Existing tests continue to pass
   - New test: compile a contract and verify `storageLayout` field is present and has `storage` array

3. **Unit tests for VibeScoreService:**
   - Existing tests continue to pass (backward compat — mock engine output without conflict_details)
   - New test: mock engine output WITH conflict_details → verify `conflictAnalysis` present in result

4. **Commands:**
   - `cd Vibe-Room-Backend && npx jest test/compile.service.spec.ts`
   - `cd Vibe-Room-Backend && npx jest test/storage-layout-decoder.spec.ts` (new)
   - `cd Vibe-Room-Backend && npx jest test/vibe-score.service.spec.ts`

## Constraints

- **solc storageLayout format:** `{ storage: [{ label, slot, offset, type }], types: { [typeId]: { encoding, label, numberOfBytes, key?, value?, base?, members? } } }`. The `slot` field is a **decimal string** (e.g., `"0"`, `"1"`), NOT hex. Runtime slots from CLI are **hex with 0x prefix**. Conversion between decimal base slot and hex runtime slot is required.

- **S01 Forward Intelligence binding:** `LocationInfo.slot` is `Optional<String>` — present only for `location_type === "Storage"`. It is lowercase hex with `0x` prefix. `conflict_type` is exactly `"write-write"` or `"read-write"`. Addresses are lowercase hex with `0x` prefix.

- **Coinbase filtering (KNOWLEDGE.md):** Coinbase address (`blockEnv.coinbase`, currently `0x00...C0`) appears in nearly all tx conflicts. Must filter before analysis. Compare `conflict.location.address` against `blockEnv.coinbase` (case-insensitive).

- **ethers.js already in deps** (v6.16.0) — `ethers.keccak256` and `ethers.AbiCoder` are available for mapping slot computation if needed.

- **Backend test pattern:** Jest with NestJS `Test.createTestingModule()`, mocked providers. New pure utility module can be tested without NestJS DI.

- **CliOutput backward compat:** The `conflict_details` field is always present in the new CLI output (not optional on the Rust side), but the TypeScript interface should mark it as `conflict_details?: ConflictDetails` for graceful handling of older CLI binaries.

## Common Pitfalls

- **Decimal vs Hex slot confusion** — solc storageLayout uses decimal strings for slot (`"0"`, `"1"`, `"7"`). CLI conflict_details uses hex strings (`"0x0"`, `"0x1"`, `"0x7"`). The decoder must normalize both to BigInt for comparison. Missing this causes zero matches.

- **Mapping runtime slot attribution** — A mapping at base slot 0 produces runtime slots like `keccak256(abi.encode(key, 0))` which are huge 256-bit numbers. Don't try to reverse keccak256. Instead: any Storage conflict slot that doesn't match a declared simple variable slot AND is a large number (> total declared slots) gets attributed to the mapping/array with the best heuristic match. For single-mapping contracts this is unambiguous. For multiple mappings, exact attribution requires knowing the key — fall back to "one of: balances, allowances" when ambiguous.

- **Deploy tx (index 0) is not a function call** — Transaction index 0 is the deploy transaction (constructor). Don't map it to a function name. The tx→function map should handle index 0 as `"constructor"` or skip it.

- **Empty conflict_details graceful handling** — When CLI returns 0 conflicts or when engine returns null, the API response should omit `conflictAnalysis` entirely (not return empty arrays). This keeps backward compat clean for S03.

## Open Risks

- **Multiple mappings at different base slots** — When two mappings both have conflict slots that are large numbers, the decoder can't distinguish which mapping a given runtime slot belongs to without the key. For M006 scope (ParallelConflict-like contracts with 1-2 mappings + a few simple vars), the heuristic of "large slot → mapping, small slot → simple var" is sufficient. Contracts with 10+ mappings may get ambiguous attributions. Acceptable for now — noted in suggestions as "possibly mapping X or Y".

- **`storageLayout` may be undefined** — solc may not return storageLayout for contracts with errors, empty contracts, or very old pragma versions. The pipeline must handle `storageLayout === undefined` gracefully by skipping the decode step and returning only raw conflict data.

## Sources

- Solidity storage layout documentation: each entry in `storage[]` has `{ astId, contract, label, offset, slot, type }` where `slot` is a decimal string and `type` references into the `types` map (source: [Solidity docs — Layout in Storage](https://docs.soliditylang.org/en/v0.8.14/internals/layout_in_storage.html))
- Mapping slot computation: `keccak256(abi.encode(key, baseSlot))` for `mapping(keyType => valueType)` at base slot `baseSlot` — from Solidity storage layout spec
