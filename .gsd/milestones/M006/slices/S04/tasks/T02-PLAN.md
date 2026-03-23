---
estimated_steps: 4
estimated_files: 1
skills_used:
  - test
---

# T02: Playwright E2E — 히트맵 렌더링 + 하위 호환 검증

**Slice:** S04 — E2E 검증 — 전체 파이프라인 통합 테스트
**Milestone:** M006

## Description

Vibe-Loom Playwright E2E 테스트 파일(`full-stack.spec.ts`)에 conflict analysis 전용 `test.describe('Conflict Analysis E2E')` 블록을 추가한다. 라이브 서비스(vibe-loom.xyz + Railway 백엔드)를 대상으로 ParallelConflict 컨트랙트의 히트맵/suggestion 카드 렌더링과 FixedContract의 하위 호환을 검증한다.

**핵심 제약:** 라이브 백엔드에 M006 Rust 바이너리가 배포되지 않았을 수 있으므로, 모든 테스트는 `Promise.race` 패턴 + graceful skip 가드를 사용해야 한다. 테스트가 라이브 서비스 상태에 따라 hard-fail하면 안 된다.

## Steps

1. **`test.describe('Conflict Analysis E2E')` 블록 추가.** 기존 `full-stack.spec.ts` 파일 끝에 새 describe 블록 추가. 기존 헬퍼(`waitForMonaco`, `setEditorContent`, `getEditorContent`)를 재사용.

2. **테스트 1: ParallelConflict 히트맵 + suggestion 카드 렌더링.**
   ```
   1. page.goto('/') → waitForMonaco
   2. 'ParallelConflict' 버튼 클릭 → waitForTimeout(2000) → 소스 로드 확인
   3. 'Vibe Score' 버튼 클릭
   4. Promise.race로 대기:
      - [data-testid="conflict-matrix"] visible → 'heatmap-visible'
      - SVG circle visible (기존 gauge) → 'gauge-only' (conflict analysis 없이 기존 결과만)
      - timeout 60s → 'timeout'
   5. 결과가 'heatmap-visible'이면:
      - [data-testid="conflict-card"] 존재 확인
      - 'counter' 텍스트 히트맵 또는 카드 내 존재 확인
      - 스크린샷: e2e/screenshots/conflict-heatmap.png
   6. 결과가 'gauge-only'이면:
      - console.log으로 기록 (backend가 아직 conflict_details를 반환하지 않음)
      - test는 pass로 처리 (라이브 서비스 상태 의존)
   7. 결과가 'timeout'이면: test.skip()
   ```

3. **테스트 2: FixedContract 하위 호환.**
   ```
   1. page.goto('/') → waitForMonaco
   2. 'FixedContract' 버튼 클릭 → waitForTimeout(2000)
   3. 'Vibe Score' 버튼 클릭
   4. SVG circle (gauge) visible 대기 (timeout 60s)
   5. [data-testid="conflict-matrix"] NOT visible 확인 (FixedContract는 충돌 없음)
   6. 기존 suggestion 텍스트 존재 확인 (suggestion|parallel|well-suited 패턴)
   7. 스크린샷: e2e/screenshots/fixedcontract-compat.png
   ```

4. **기존 테스트 통과 확인.** `npx playwright test e2e/full-stack.spec.ts` 전체 실행 — 기존 테스트와 새 테스트 모두 pass (또는 서비스 상태에 따라 graceful skip).

## Must-Haves

- [ ] ParallelConflict 테스트: `[data-testid="conflict-matrix"]` visible 또는 서비스 상태에 따라 graceful 처리
- [ ] FixedContract 테스트: `[data-testid="conflict-matrix"]` NOT present + SVG gauge visible
- [ ] 기존 Playwright E2E 테스트 전부 pass 유지
- [ ] 모든 테스트에 `Promise.race` 타임아웃 패턴 적용 — hard-fail 없음
- [ ] 각 주요 단계에서 스크린샷 증거 캡처

## Verification

- `cd /home/ahwlsqja/Vibe-Loom && npx playwright test e2e/full-stack.spec.ts` — 전체 pass 또는 graceful skip
- 스크린샷 파일 존재: `Vibe-Loom/e2e/screenshots/conflict-heatmap.png` (테스트 실행 시)

## Inputs

- `/home/ahwlsqja/Vibe-Loom/e2e/full-stack.spec.ts` — 기존 Playwright E2E 파일 (확장 대상)
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/VibeScoreDashboard.tsx` — data-testid 위치 참조 (`conflict-matrix`, `conflict-card`)
- `/home/ahwlsqja/Vibe-Loom/src/lib/api-client.ts` — ConflictAnalysis 타입 참조

## Expected Output

- `/home/ahwlsqja/Vibe-Loom/e2e/full-stack.spec.ts` — 새 `test.describe('Conflict Analysis E2E')` 블록 추가 (ParallelConflict + FixedContract 테스트)
