# S04: Regression — E2E 테스트 호환 + 최종 검증 — Research

**Date:** 2026-03-23
**Depth:** Light research — straightforward test fixup + checklist verification with known patterns.

## Summary

S04 is a regression validation slice. Its job is to (1) update the existing 22 Playwright E2E tests to match the Korean→English text changes made in S03, (2) fix one broken unit test, (3) run the full test suite against the live site to prove 22/22 pass, and (4) verify the impeccable anti-pattern checklist is clean.

The work is well-bounded: S03 replaced ~35 Korean user-facing strings with English. The E2E test file (`e2e/full-stack.spec.ts`) has **8 lines** with Korean text in `getByText()` selectors that will no longer match. Additionally, one unit test (`src/__tests__/VibeScoreDashboard.test.tsx` line 61) asserts `'분석 중...'` which is now `'Analyzing...'`. There are no DOM structural changes that would break other selectors — S01-S03 changed CSS classes but preserved the DOM element hierarchy, button text (Compile/Deploy/Vibe Score), and role attributes.

One minor leftover from S02: `src/app/layout.tsx` line 32 still has `text-gray-100` which should be `text-text-primary` for full design token compliance.

## Recommendation

Three tasks:

1. **Fix E2E test selectors** — Update 8 Korean text references in `e2e/full-stack.spec.ts` to English equivalents. Fix one unit test in `src/__tests__/VibeScoreDashboard.test.tsx`.
2. **Run full test suite** — Execute `npx playwright test` against live site (vibe-loom.xyz), verify 22/22 pass (or 21 pass + 1 skip for deploy-dependent test). Also run `npm test` for unit tests.
3. **Final verification** — Anti-pattern checklist, design token audit, desktop+mobile screenshot evidence, remaining Korean audit.

## Implementation Landscape

### Key Files

- `/home/ahwlsqja/Vibe-Loom/e2e/full-stack.spec.ts` (505 lines) — The single E2E test file with 22 tests across 11 describe blocks. 8 lines contain Korean text in selectors that must be updated to English.
- `/home/ahwlsqja/Vibe-Loom/src/__tests__/VibeScoreDashboard.test.tsx` (87 lines) — Unit test with one assertion for Korean `'분석 중...'` → should be `'Analyzing...'`.
- `/home/ahwlsqja/Vibe-Loom/src/app/layout.tsx` — One remaining `text-gray-100` → `text-text-primary` for full token compliance.
- `/home/ahwlsqja/Vibe-Loom/playwright.config.ts` — Config targeting `https://vibe-loom.xyz`, 120s timeout, 1 retry. No changes needed.

### Exact E2E Selector Changes Required

| Line | Current (Korean) | New (English) | Test | Risk |
|------|-------------------|---------------|------|------|
| 173 | `getByText('컴파일 중...')` | `getByText('Compiling...')` | Compile Flow / successful compile | Low — `.catch(() => {})` makes it soft |
| 242 | `getByText('분석 중...')` | `getByText('Analyzing...')` | Vibe-Score Flow | Low — `.catch(() => {})` makes it soft |
| 250 | `제안` in regex | Remove `제안` from regex (English alternatives already present) | Vibe-Score Flow | None — regex has `suggestion\|Suggestion` |
| 271 | `getByText('배포 중...')` | `getByText('Deploying...')` | Deploy Flow | Low — `.catch(() => {})` makes it soft |
| 276 | `배포.*실패` in regex | Remove Korean from regex, keep `deploy.*fail\|error` | Deploy Flow | None — English alternatives present |
| 448 | `에러\|실패` in regex | Remove Korean from regex, keep `error\|fail` | AI Error Analysis | None — English alternatives present |
| 478 | `getByText('배포 중...')` | `getByText('Deploying...')` | Contract Interaction | Low — `.catch(() => {})` makes it soft |
| 483 | `배포.*실패` in regex | Remove Korean from regex | Contract Interaction | None — English alternatives present |

### Unit Test Change

| File | Line | Current | New |
|------|------|---------|-----|
| `src/__tests__/VibeScoreDashboard.test.tsx` | 59 | `'shows loading skeleton with "분석 중..." text'` | `'shows loading skeleton with "Analyzing..." text'` |
| `src/__tests__/VibeScoreDashboard.test.tsx` | 61 | `screen.getByText('분석 중...')` | `screen.getByText('Analyzing...')` |

### Layout.tsx Token Fix

| File | Line | Current | New |
|------|------|---------|-----|
| `src/app/layout.tsx` | 32 | `text-gray-100` | `text-text-primary` |

### Build Order

1. **T01: Fix all test selectors** — Update 8 E2E lines + 2 unit test lines + 1 layout.tsx token fix. This is purely mechanical text replacement. Run `npm run build` and `npm test` to verify unit tests pass.
2. **T02: Run E2E suite + final verification** — Execute `npx playwright test` against live site. Run impeccable anti-pattern checklist. Capture desktop (1440×900) + mobile (375×812) screenshot evidence. Audit remaining Korean strings. Document results.

### Verification Approach

**E2E test pass:**
```bash
cd /home/ahwlsqja/Vibe-Loom && npx playwright test --reporter=list
```
Expect: 22 tests, ≥20 passed, ≤1 skipped (deploy-dependent), 0 failed.

**Unit tests pass:**
```bash
cd /home/ahwlsqja/Vibe-Loom && npm test
```
Expect: All tests pass including VibeScoreDashboard loading test.

**Build passes:**
```bash
cd /home/ahwlsqja/Vibe-Loom && npm run build
```
Expect: exit 0.

**Anti-pattern checklist (all should return 0 matches):**
```bash
# Pure black/white backgrounds (excluding backdrop opacity)
grep -rn 'bg-black\b' src/ --include="*.tsx" | grep -v '/[0-9]'
grep -rn 'bg-white\b' src/ --include="*.tsx"
# Gradient text
grep -rn 'bg-clip-text\|text-transparent.*gradient' src/ --include="*.tsx"
# Inter/system-ui font declarations
grep -rn 'font-family.*Inter\b\|font-family.*system-ui' src/ --include="*.tsx" --include="*.css"
# Cards in cards
# (manual visual check)
# Structural gray references
grep -rn 'gray-[0-9]' src/ --include="*.tsx" | grep -v node_modules
```
After T01 fix, expect: zero structural gray references.

**Korean audit (should return only JSDoc):**
```bash
LC_ALL=C grep -rPn '[\xea-\xed][\x80-\xbf][\x80-\xbf]' src/ --include="*.tsx" | grep -v 'node_modules\|.next\|__tests__\|//'
```
Expect: only VibeStatus.tsx JSDoc lines 8-10.

**Design token coverage:**
```bash
grep -c 'bg-surface-\|text-text-\|border-border-\|font-display\|font-body' src/app/page.tsx src/components/ide/*.tsx src/app/layout.tsx
```
Expect: >0 per file.

## Constraints

- E2E tests run against **live** `https://vibe-loom.xyz` — deploy-dependent tests may skip/flaky due to Monad testnet latency (30-90s). This is pre-existing behavior (D008).
- The `opacity-0 pointer-events-none` mobile tab pattern (S03) means elements are in DOM but visually hidden. Playwright's `toBeVisible()` on content inside inactive tabs could be affected — but the existing mobile tests only click tab buttons (which remain fully visible), so no breakage expected.
- The E2E test file lives in the **Vibe-Loom repo** (`/home/ahwlsqja/Vibe-Loom/e2e/`), not the monad-core worktree. Changes must be applied there.

## Common Pitfalls

- **Korean in regex alternatives** — Lines 276, 448, 483 use regex patterns like `/deploy.*fail|배포.*실패|error/i`. The Korean alternatives won't match anymore but are harmless (they're in `|` branches with English equivalents). Still, they should be cleaned up for maintainability.
- **Soft assertions with .catch** — Lines 173, 242, 271, 478 have `.catch(() => {})` which means the Korean text failure is silently swallowed. The tests would "pass" even without fixing these lines — but the assertions would be no-ops. Must fix for the assertions to actually verify behavior.
