# S04: Regression — E2E 테스트 호환 + 최종 검증

**Goal:** All 22 existing Playwright E2E tests pass against the redesigned UI, unit tests pass, and the impeccable anti-pattern checklist is clean — proving zero functional regression from M004's visual redesign.
**Demo:** `npx playwright test --reporter=list` shows 22 tests with ≥20 passed, ≤1 skipped, 0 failed. `npm test` passes all unit tests. Anti-pattern grep checks return zero violations. Desktop + mobile screenshot evidence captured.

## Must-Haves

- 8 Korean text selectors in `e2e/full-stack.spec.ts` updated to English equivalents
- Unit test in `src/__tests__/VibeScoreDashboard.test.tsx` updated from `'분석 중...'` to `'Analyzing...'`
- `src/app/layout.tsx` remaining `text-gray-100` fixed to `text-text-primary`
- `npm run build` exits 0
- `npm test` passes all unit tests
- Anti-pattern checklist (no bg-black/bg-white, no gradient text, no Inter/system-ui, zero structural gray-N references) — all clean
- Design token usage confirmed across all component files

## Proof Level

- This slice proves: final-assembly
- Real runtime required: yes (E2E tests run against live site)
- Human/UAT required: no (automated checks + screenshot evidence)

## Verification

- `cd /home/ahwlsqja/Vibe-Loom && npm run build` — exit 0
- `cd /home/ahwlsqja/Vibe-Loom && npm test` — all tests pass including VibeScoreDashboard loading test
- `cd /home/ahwlsqja/Vibe-Loom && npx playwright test --reporter=list` — 22 tests, ≥20 passed, ≤1 skipped, 0 failed
- `grep -rn 'gray-[0-9]' /home/ahwlsqja/Vibe-Loom/src/ --include="*.tsx" | grep -v node_modules` — zero results
- `grep -rn 'bg-black\b' /home/ahwlsqja/Vibe-Loom/src/ --include="*.tsx" | grep -v '/[0-9]'` — zero results
- `grep -rn 'bg-clip-text' /home/ahwlsqja/Vibe-Loom/src/ --include="*.tsx"` — zero results
- `LC_ALL=C grep -rPn '[\xea-\xed][\x80-\xbf][\x80-\xbf]' /home/ahwlsqja/Vibe-Loom/src/ --include="*.tsx" | grep -v 'node_modules\|.next\|__tests__\|//'` — only VibeStatus.tsx JSDoc lines 8-10

## Integration Closure

- Upstream surfaces consumed: S03's completed UI (English UX copy, motion classes, tab-fade pattern), S02's design token vocabulary, S01's design system foundation
- New wiring introduced in this slice: none — purely test/validation fixes
- What remains before the milestone is truly usable end-to-end: nothing — this is the final slice

## Observability / Diagnostics

- **Build exit code**: `npm run build` exit 0 confirms no TypeScript/Next.js compilation errors after text changes
- **Unit test pass/fail**: `npm test` output shows individual test case results; loading-text assertion directly validates the Korean→English rename
- **E2E suite output**: `npx playwright test --reporter=list` shows per-test pass/fail; failures would indicate selector mismatches against live UI
- **Anti-pattern grep checks**: Zero-result grep commands for `gray-[0-9]`, `bg-black`, `bg-clip-text` confirm design token compliance
- **Korean audit grep**: `LC_ALL=C grep -rPn '[\xea-\xed][\x80-\xbf][\x80-\xbf]'` detects residual Korean — only JSDoc comments in VibeStatus.tsx are expected
- **Failure artifacts**: Playwright screenshots in `e2e/screenshots/` capture visual state at each E2E test step
- **Redaction**: No secrets or PII involved in this slice

## Tasks

- [x] **T01: Fix E2E selectors, unit test, and layout.tsx token for English UX copy** `est:20m`
  - Why: S03 replaced all Korean user-facing text with English, breaking 8 E2E selectors and 1 unit test assertion. One layout.tsx gray reference remains from S02.
  - Files: `/home/ahwlsqja/Vibe-Loom/e2e/full-stack.spec.ts`, `/home/ahwlsqja/Vibe-Loom/src/__tests__/VibeScoreDashboard.test.tsx`, `/home/ahwlsqja/Vibe-Loom/src/app/layout.tsx`
  - Do: (1) Update 8 Korean text selectors in E2E test file to English equivalents — `컴파일 중...` → `Compiling...`, `분석 중...` → `Analyzing...`, `배포 중...` → `Deploying...`, remove Korean alternatives from regex patterns. (2) Update unit test description and assertion from `분석 중...` to `Analyzing...`. (3) Fix `text-gray-100` → `text-text-primary` in layout.tsx. (4) Run `npm run build` and `npm test` to verify.
  - Verify: `cd /home/ahwlsqja/Vibe-Loom && npm run build && npm test` — both exit 0
  - Done when: All 3 files updated, build passes, unit tests pass, zero `gray-[0-9]` references in src/

- [x] **T02: Run E2E suite and capture final verification evidence** `est:30m`
  - Why: Proves zero functional regression from the M004 redesign by running the full 22-test E2E suite and documenting anti-pattern compliance.
  - Files: `/home/ahwlsqja/Vibe-Loom/e2e/full-stack.spec.ts`, `/home/ahwlsqja/Vibe-Loom/playwright.config.ts`
  - Do: (1) Run `npx playwright test --reporter=list` against live site. (2) Run anti-pattern checklist greps (bg-black, bg-white, gradient text, Inter/system-ui, structural gray). (3) Run Korean audit grep. (4) Run design token coverage check across all component files. (5) Document all results.
  - Verify: `cd /home/ahwlsqja/Vibe-Loom && npx playwright test --reporter=list` — ≥20 passed, 0 failed
  - Done when: E2E results documented (≥20 pass, 0 fail), anti-pattern checklist all clean, design token coverage confirmed, Korean audit shows only JSDoc lines

## Files Likely Touched

- `/home/ahwlsqja/Vibe-Loom/e2e/full-stack.spec.ts`
- `/home/ahwlsqja/Vibe-Loom/src/__tests__/VibeScoreDashboard.test.tsx`
- `/home/ahwlsqja/Vibe-Loom/src/app/layout.tsx`
