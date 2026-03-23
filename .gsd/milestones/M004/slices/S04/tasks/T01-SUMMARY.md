---
id: T01
parent: S04
milestone: M004
provides:
  - English-aligned E2E selectors for all 22 Playwright tests
  - English-aligned unit test assertion for VibeScoreDashboard loading state
  - Full design token compliance in layout.tsx (text-text-primary)
  - Jest config fix to exclude e2e/ directory from unit test runner
key_files:
  - /home/ahwlsqja/Vibe-Loom/e2e/full-stack.spec.ts
  - /home/ahwlsqja/Vibe-Loom/src/__tests__/VibeScoreDashboard.test.tsx
  - /home/ahwlsqja/Vibe-Loom/src/app/layout.tsx
  - /home/ahwlsqja/Vibe-Loom/jest.config.js
key_decisions:
  - Added testPathIgnorePatterns for e2e/ in jest.config.js to prevent Jest from loading Playwright test files
patterns_established:
  - Use sed -i for Korean string replacement (edit tool has inconsistent UTF-8 handling with Korean)
observability_surfaces:
  - npm test output shows per-test pass/fail for the renamed loading-text assertion
  - grep checks confirm zero gray-[0-9] and zero functional Korean in test selectors
duration: 10m
verification_result: passed
completed_at: 2026-03-23
blocker_discovered: false
---

# T01: Fix E2E selectors, unit test, and layout.tsx token for English UX copy

**Updated 8 Korean E2E selectors to English, fixed unit test loading-text assertion, replaced text-gray-100 with text-text-primary in layout.tsx, and added e2e/ exclusion to jest.config.js**

## What Happened

S03 replaced all Korean user-facing strings with English across the Vibe-Loom UI, breaking 8 Playwright E2E selectors and 1 unit test assertion. This task performed mechanical text replacement across three files:

1. **E2E test file** (`e2e/full-stack.spec.ts`): Replaced `컴파일 중...` → `Compiling...`, `분석 중...` → `Analyzing...`, `배포 중...` → `Deploying...` in `getByText()` calls, and removed Korean alternatives (`제안|`, `|배포.*실패`, `|에러|실패`) from regex patterns that already had English equivalents.

2. **Unit test** (`src/__tests__/VibeScoreDashboard.test.tsx`): Updated test description and `screen.getByText()` assertion from `분석 중...` to `Analyzing...`.

3. **Layout** (`src/app/layout.tsx`): Replaced `text-gray-100` with `text-text-primary` for full design token compliance.

4. **Jest config** (`jest.config.js`): Added `testPathIgnorePatterns: ['<rootDir>/e2e/']` — Jest was incorrectly picking up the Playwright spec file and failing on the `@playwright/test` import. This was a pre-existing issue unmasked by running `npm test`.

## Verification

- `npm run build` — exit 0, successful production build
- `npm test` — 5 suites, 57 tests all passed (including VibeScoreDashboard "Analyzing..." loading test)
- `grep -rn 'gray-[0-9]' src/ --include="*.tsx"` — zero results
- `grep -rn 'bg-black\b' src/ --include="*.tsx"` — zero results
- `grep -rn 'bg-clip-text' src/ --include="*.tsx"` — zero results
- Korean audit on E2E file — zero functional Korean (only comments)
- Korean audit on src/*.tsx — only VibeStatus.tsx JSDoc lines 8-10 (expected)

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cd /home/ahwlsqja/Vibe-Loom && npm run build` | 0 | ✅ pass | 32s |
| 2 | `cd /home/ahwlsqja/Vibe-Loom && npm test` | 0 | ✅ pass | 3s |
| 3 | `grep -rn 'gray-[0-9]' src/ --include="*.tsx" \| grep -v node_modules` | 1 (no match) | ✅ pass | <1s |
| 4 | `grep -rn 'bg-black\b' src/ --include="*.tsx" \| grep -v '/[0-9]'` | 1 (no match) | ✅ pass | <1s |
| 5 | `grep -rn 'bg-clip-text' src/ --include="*.tsx"` | 1 (no match) | ✅ pass | <1s |
| 6 | `LC_ALL=C grep -Pn Korean e2e/full-stack.spec.ts \| grep -v '//'` | 1 (no match) | ✅ pass | <1s |
| 7 | `LC_ALL=C grep -rPn Korean src/ --include="*.tsx" \| grep -v 'node_modules\|__tests__\|//'` | 0 | ✅ pass (only VibeStatus.tsx:8-10 JSDoc) | <1s |

## Diagnostics

- Run `npm test -- --verbose` to see individual test names including the renamed "shows loading skeleton with Analyzing... text"
- Run `npx playwright test --reporter=list` (T02 task) to validate E2E selectors against the live UI
- Check `grep -rn 'gray-[0-9]' src/ --include="*.tsx"` for design token violations

## Deviations

- **Added jest.config.js change** (not in original plan): Jest was loading the Playwright E2E spec file and failing on the `@playwright/test` import. Added `testPathIgnorePatterns: ['<rootDir>/e2e/']` to fix this pre-existing config issue. Without this, `npm test` would always report 1 failed suite.

## Known Issues

None.

## Files Created/Modified

- `/home/ahwlsqja/Vibe-Loom/e2e/full-stack.spec.ts` — 8 Korean text selectors replaced with English equivalents; Korean alternatives removed from 4 regex patterns
- `/home/ahwlsqja/Vibe-Loom/src/__tests__/VibeScoreDashboard.test.tsx` — Test description and assertion updated from `분석 중...` to `Analyzing...`
- `/home/ahwlsqja/Vibe-Loom/src/app/layout.tsx` — `text-gray-100` replaced with `text-text-primary`
- `/home/ahwlsqja/Vibe-Loom/jest.config.js` — Added `testPathIgnorePatterns` to exclude `e2e/` from Jest runner
