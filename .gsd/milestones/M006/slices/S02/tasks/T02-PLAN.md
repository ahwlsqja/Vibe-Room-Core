---
estimated_steps: 5
estimated_files: 2
---

# T02: storage-layout-decoder 모듈 구현 + 단위 테스트

**Slice:** S02 — NestJS — Storage Layout 디코딩 + Actionable Suggestion 생성
**Milestone:** M006

## Description

S02의 핵심 도메인 로직을 순수 함수 모듈로 구현한다. `storage-layout-decoder.ts`는 NestJS DI에 의존하지 않는 순수 TypeScript 모듈로, S01의 CLI `conflict_details`와 solc `storageLayout`을 입력받아 디코딩된 충돌 분석 + actionable suggestion + 매트릭스를 출력한다.

**Key domain complexities:**
- solc storageLayout의 slot은 **decimal string** (e.g., `"0"`, `"1"`). CLI conflict_details의 slot은 **hex with 0x prefix** (e.g., `"0x0"`, `"0x7"`). BigInt 변환으로 정규화 필수.
- mapping/dynamic array의 runtime slot은 `keccak256(key ++ baseSlot)`로 생성되어 매우 큰 256-bit 숫자. 역변환 불가 — heuristic: max declared slot보다 큰 runtime slot은 mapping/array에 귀속.
- coinbase 주소 (e.g., `0x00...C0`)의 Balance/Nonce/CodeHash 충돌은 모든 tx에서 발생하는 EVM 내재적 동작 → 필터링 필수.

**Relevant skills:** `test` (Jest test generation)

## Steps

1. **decodeSlotToVariable() 함수 구현** — `Vibe-Room-Backend/src/vibe-score/storage-layout-decoder.ts`:
   - 입력: `slot: string` (hex with 0x prefix), `storageLayout: StorageLayout`
   - hex slot → `BigInt(slot)` 변환
   - storageLayout.storage 순회: 각 entry의 decimal slot → `BigInt(entry.slot)` 변환 → exact match 확인
   - exact match 시: `{ variableName: entry.label, variableType: storageLayout.types[entry.type]?.label, slot: slot }`
   - no match 시: max declared slot 계산. runtime slot > max declared slot → storageLayout에서 encoding==="mapping" 또는 "dynamic_array"인 변수 찾기 → 해당 변수에 귀속 (단일 mapping이면 확정, 복수면 "unknown (possibly X or Y)")
   - 여전히 no match: `{ variableName: "unknown_slot_" + slot, variableType: "unknown", slot }`

2. **buildConflictAnalysis() 메인 오케스트레이터 구현:**
   - 입력: `conflictDetails: ConflictDetails`, `storageLayout: StorageLayout | undefined`, `abi: any[]`, `txFunctionMap: Map<number, string>`, `coinbaseAddress: string`
   - Step A: conflicts 필터링 — `conflict.location.address.toLowerCase() === coinbaseAddress.toLowerCase()` 제거
   - Step B: Storage 타입 conflicts만 디코딩 대상 (`location.location_type === "Storage"` && `location.slot` 존재)
   - Step C: 각 conflict의 slot을 `decodeSlotToVariable()` 로 디코딩
   - Step D: tx_a/tx_b를 `txFunctionMap`으로 함수명 변환
   - Step E: 동일 variable로 그룹핑 → DecodedConflict[] 생성
   - Step F: `generateSuggestion()` 호출로 각 conflict에 suggestion 첨부
   - Step G: `buildMatrix()` 호출로 function×variable matrix 생성
   - storageLayout이 undefined면 빈 결과 반환

3. **generateSuggestion() 함수 구현:**
   - variable type에 따른 구체적 제안:
     - `mapping` 관련: `"mapping '{name}'에서 {func1}과 {func2}가 충돌 — 키 범위 분리 또는 별도 mapping 사용 권장"`
     - `uint256` 등 단순 변수: `"변수 '{name}'에서 {func1}과 {func2}가 충돌 — 함수별 별도 변수 분리 또는 accumulation 패턴 사용 권장"`
     - `dynamic_array`: `"배열 '{name}'에서 충돌 — push 대신 mapping 기반 구조로 전환 권장"`
     - unknown: `"slot {slot}에서 충돌 — storage layout을 확인하세요"`
   - **영어로 출력** (D019 결정: UX 텍스트 언어 통일 English)

4. **buildMatrix() 함수 구현:**
   - 입력: `decodedConflicts: DecodedConflict[]`
   - unique function names → rows, unique variable names → cols
   - 2D cells 초기화 (rows.length × cols.length, all 0)
   - 각 conflict의 functions × variableName 교차점에 +1
   - 반환: `{ rows, cols, cells }`

5. **포괄적 단위 테스트** — `Vibe-Room-Backend/test/storage-layout-decoder.spec.ts`:
   - `decodeSlotToVariable()` — exact slot match (slot "0x0" matches entry slot "0" → "counter")
   - `decodeSlotToVariable()` — mapping heuristic (큰 runtime slot → mapping base에 귀속)
   - `decodeSlotToVariable()` — no match → "unknown_slot_0x..."
   - `buildConflictAnalysis()` — coinbase 주소 충돌 필터링 확인
   - `buildConflictAnalysis()` — non-Storage conflicts (Balance/Nonce) 스킵 확인
   - `buildConflictAnalysis()` — storageLayout undefined → 빈 결과
   - `generateSuggestion()` — suggestion에 변수명/함수명 포함 확인
   - `buildMatrix()` — matrix dimensions 정확성 (rows=unique funcs, cols=unique vars)
   - `buildConflictAnalysis()` — 전체 ParallelConflict-like fixture: counter 변수, increment/incrementBy 함수 → decoded conflict + matrix + suggestion

   **Test fixture — ParallelConflict storageLayout mock:**
   ```typescript
   const parallelConflictLayout: StorageLayout = {
     storage: [{ astId: 1, contract: 'ParallelConflict', label: 'counter', offset: 0, slot: '0', type: 't_uint256' }],
     types: { 't_uint256': { encoding: 'inplace', label: 'uint256', numberOfBytes: '32' } }
   };
   ```

   **Test fixture — conflict_details mock:**
   ```typescript
   const mockConflictDetails: ConflictDetails = {
     per_tx: [...],
     conflicts: [
       { location: { location_type: 'Storage', address: '0x...deployed', slot: '0x0' }, tx_a: 1, tx_b: 2, conflict_type: 'write-write' },
       // coinbase conflicts to filter:
       { location: { location_type: 'Balance', address: '0x00...C0' }, tx_a: 0, tx_b: 1, conflict_type: 'read-write' },
     ]
   };
   ```

## Must-Haves

- [ ] decodeSlotToVariable()가 hex→decimal 변환으로 exact slot match 수행
- [ ] mapping/dynamic_array heuristic: large runtime slot → mapping base variable 귀속
- [ ] coinbase 주소 충돌 필터링 (대소문자 무시)
- [ ] non-Storage 타입 conflict 스킵 (Balance, Nonce, CodeHash는 변수명 디코딩 불가)
- [ ] suggestion 텍스트에 구체적 variableName + functionName 포함 (영어)
- [ ] matrix rows=unique function names, cols=unique variable names, cells=conflict counts
- [ ] storageLayout undefined → 빈 결과 (에러 아님)
- [ ] 8개 이상 단위 테스트 통과

## Verification

- `cd Vibe-Room-Backend && npx jest test/storage-layout-decoder.spec.ts` — 8+ 테스트 전부 통과
- ParallelConflict fixture로 slot "0x0" → "counter" 디코딩 확인

## Inputs

- `Vibe-Room-Backend/src/contracts/dto/compile-result.dto.ts` — StorageLayout, StorageEntry, StorageTypeInfo 인터페이스 (T01에서 정의)
- `Vibe-Room-Backend/src/engine/engine.service.ts` — ConflictDetails, ConflictPair, LocationInfo, TxAccessSummary 인터페이스 (T01에서 정의)
- `Vibe-Room-Backend/src/vibe-score/dto/vibe-score-result.dto.ts` — DecodedConflict, ConflictMatrix, ConflictAnalysis 인터페이스 (T01에서 정의)

## Observability Impact

- **New inspectable functions:** `decodeSlotToVariable()`, `buildConflictAnalysis()`, `generateSuggestion()`, `buildMatrix()` are all pure exported functions — testable and inspectable without DI.
- **Failure visibility:** `decodeSlotToVariable()` returns `unknown_slot_0xNNN` on decode failure instead of throwing — agents can grep for "unknown_slot" in API responses to detect decode misses. `buildConflictAnalysis()` returns empty `{ conflicts: [], matrix: { rows: [], cols: [], cells: [] } }` when storageLayout is undefined — no error thrown.
- **Diagnostic signals:** Test suite validates all edge cases — run `npx jest test/storage-layout-decoder.spec.ts --verbose` to inspect individual test outcomes.

## Expected Output

- `Vibe-Room-Backend/src/vibe-score/storage-layout-decoder.ts` — decodeSlotToVariable(), buildConflictAnalysis(), generateSuggestion(), buildMatrix() 함수 exports
- `Vibe-Room-Backend/test/storage-layout-decoder.spec.ts` — 8+ 단위 테스트
