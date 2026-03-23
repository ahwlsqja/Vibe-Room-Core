---
estimated_steps: 4
estimated_files: 3
---

# T01: Fix E2E selectors, unit test, and layout.tsx token for English UX copy

**Slice:** S04 — Regression — E2E 테스트 호환 + 최종 검증
**Milestone:** M004

## Description

S03 replaced all ~35 Korean user-facing strings with English across the Vibe-Loom UI. This broke 8 Korean text selectors in the Playwright E2E test file and 1 unit test assertion. Additionally, `layout.tsx` has one remaining `text-gray-100` (a structural gray reference) that should be `text-text-primary` for full design token compliance.

This task fixes all three files so the test suite matches the current English UI text.

**Important:** All file edits are in the Vibe-Loom repo at `/home/ahwlsqja/Vibe-Loom/`, NOT the monad-core worktree.

**Relevant skills:** None needed — this is mechanical text replacement.

## Steps

1. **Update E2E selectors in `e2e/full-stack.spec.ts`** — Fix 8 lines with Korean text:
   - Line 173: `getByText('컴파일 중...')` → `getByText('Compiling...')`
   - Line 242: `getByText('분석 중...')` → `getByText('Analyzing...')`
   - Line 250: Remove `제안|` from the regex pattern (English alternatives `suggestion|Suggestion` already present)
   - Line 271: `getByText('배포 중...')` → `getByText('Deploying...')`
   - Line 276: Remove `|배포.*실패` from regex pattern (keep `deploy.*fail|error`)
   - Line 448: Remove `|에러|실패` from regex pattern (keep `error|fail`)
   - Line 478: `getByText('배포 중...')` → `getByText('Deploying...')`
   - Line 483: Remove `|배포.*실패` from regex pattern (keep `deploy.*fail|error`)
   
   Use `sed -i` for Korean string replacement (the `edit` tool has inconsistent UTF-8 handling with Korean characters — see KNOWLEDGE.md).

2. **Update unit test in `src/__tests__/VibeScoreDashboard.test.tsx`** — Fix 2 lines:
   - Line 59: Change test description from `'shows loading skeleton with "분석 중..." text'` to `'shows loading skeleton with "Analyzing..." text'`
   - Line 61: Change `screen.getByText('분석 중...')` to `screen.getByText('Analyzing...')`

3. **Fix layout.tsx token** — In `src/app/layout.tsx` line 32:
   - Change `text-gray-100` to `text-text-primary` in the body className

4. **Verify** — Run `npm run build` and `npm test` from `/home/ahwlsqja/Vibe-Loom/`

## Must-Haves

- [ ] All 8 Korean text selectors in `e2e/full-stack.spec.ts` replaced with English equivalents
- [ ] Korean alternatives removed from regex patterns in lines 250, 276, 448, 483
- [ ] Unit test description and assertion updated from `분석 중...` to `Analyzing...`
- [ ] `text-gray-100` replaced with `text-text-primary` in layout.tsx
- [ ] `npm run build` exits 0
- [ ] `npm test` passes (including VibeScoreDashboard loading test)
- [ ] Zero `gray-[0-9]` references in `src/**/*.tsx`

## Verification

- `cd /home/ahwlsqja/Vibe-Loom && npm run build` — exit 0
- `cd /home/ahwlsqja/Vibe-Loom && npm test` — all tests pass
- `grep -rn 'gray-[0-9]' /home/ahwlsqja/Vibe-Loom/src/ --include="*.tsx" | grep -v node_modules` — zero results
- `LC_ALL=C grep -Pn '[\xea-\xed][\x80-\xbf][\x80-\xbf]' /home/ahwlsqja/Vibe-Loom/e2e/full-stack.spec.ts | grep -v '//'` — zero results (no Korean in functional selectors, only in comments)

## Observability Impact

- **Signals changed**: Unit test assertion text changes from Korean `분석 중...` to English `Analyzing...`; E2E selectors now match English UI text. No runtime logging or metrics change.
- **How to inspect**: Run `npm test -- --verbose` to see individual test case names and pass/fail. Run `npx playwright test --reporter=list` for E2E results. Use `grep -rn 'gray-[0-9]' src/ --include="*.tsx"` to confirm zero design-token violations.
- **Failure visibility**: Build failure exits non-zero with TypeScript errors. Unit test failure shows expected vs actual text mismatch. E2E failure shows selector-not-found timeout.

## Inputs

- `/home/ahwlsqja/Vibe-Loom/e2e/full-stack.spec.ts` — E2E test file with 8 Korean text selectors that no longer match the English UI
- `/home/ahwlsqja/Vibe-Loom/src/__tests__/VibeScoreDashboard.test.tsx` — Unit test asserting Korean `분석 중...` which is now `Analyzing...`
- `/home/ahwlsqja/Vibe-Loom/src/app/layout.tsx` — Layout file with remaining `text-gray-100` structural gray reference

## Expected Output

- `/home/ahwlsqja/Vibe-Loom/e2e/full-stack.spec.ts` — All 8 Korean selectors replaced with English; Korean regex alternatives removed
- `/home/ahwlsqja/Vibe-Loom/src/__tests__/VibeScoreDashboard.test.tsx` — Test description and assertion updated to `Analyzing...`
- `/home/ahwlsqja/Vibe-Loom/src/app/layout.tsx` — `text-gray-100` replaced with `text-text-primary`
