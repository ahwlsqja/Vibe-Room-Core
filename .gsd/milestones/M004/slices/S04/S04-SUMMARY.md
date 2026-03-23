---
id: S04
parent: M004
milestone: M004
provides:
  - Zero-regression proof — 21/22 E2E tests passed, 1 skipped (testnet timeout), 0 failed
  - Anti-pattern compliance proof — 5 design checklist checks all zero violations
  - Full design token adoption — all 13 component files have >0 token references
  - English UX copy alignment across E2E selectors and unit tests
  - Jest/Playwright coexistence config (testPathIgnorePatterns)
requires:
  - slice: S03
    provides: Completed UI with English UX copy, motion classes, tab-fade pattern
  - slice: S02
    provides: Design token vocabulary across all components
  - slice: S01
    provides: Design system foundation (tokens, Monaco theme, layout shell)
affects: []
key_files:
  - /home/ahwlsqja/Vibe-Loom/e2e/full-stack.spec.ts
  - /home/ahwlsqja/Vibe-Loom/src/__tests__/VibeScoreDashboard.test.tsx
  - /home/ahwlsqja/Vibe-Loom/src/app/layout.tsx
  - /home/ahwlsqja/Vibe-Loom/jest.config.js
key_decisions:
  - Added testPathIgnorePatterns for e2e/ in jest.config.js to prevent Jest from loading Playwright files
patterns_established:
  - E2E selector updates should always follow UX copy changes in the same milestone
  - Jest and Playwright coexist in the same repo via testPathIgnorePatterns
observability_surfaces:
  - npx playwright test --reporter=list — per-test pass/fail/skip status
  - npm test — 57 unit tests across 5 suites
  - Anti-pattern grep commands (bg-black, bg-white, bg-clip-text, Inter/system-ui, gray-N) — exit 1 means clean
  - e2e/screenshots/ — visual state at each E2E step
drill_down_paths:
  - .gsd/milestones/M004/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M004/slices/S04/tasks/T02-SUMMARY.md
duration: 36m
verification_result: passed
completed_at: 2026-03-23
---

# S04: Regression — E2E 테스트 호환 + 최종 검증

**Proved zero functional regression from M004's full visual redesign: 21/22 E2E passed, 57 unit tests passed, zero anti-pattern violations, full design token coverage across 13 component files**

## What Happened

S03 replaced all Korean user-facing text with English, which broke 8 Playwright E2E selectors and 1 unit test assertion that matched Korean strings. S04 fixed these mechanical mismatches and then ran the complete verification suite to prove the entire M004 redesign introduced no functional regressions.

**T01 — Fix selectors and assertions.** Updated 8 `getByText()` calls in `e2e/full-stack.spec.ts` from Korean (`컴파일 중...`, `분석 중...`, `배포 중...`) to English equivalents. Removed Korean alternatives from 4 regex patterns that already had English branches. Updated the VibeScoreDashboard unit test assertion from `분석 중...` to `Analyzing...`. Fixed the last `text-gray-100` in `layout.tsx` to `text-text-primary`. Also fixed a pre-existing issue: Jest was loading the Playwright spec file and failing on the `@playwright/test` import — added `testPathIgnorePatterns: ['<rootDir>/e2e/']` to `jest.config.js`.

**T02 — Full verification suite.** Ran the complete E2E Playwright suite against the live site (`vibe-loom.xyz`): 21 passed, 1 skipped (Contract Interaction test — Monad testnet deploy timed out, consistent with D008), 0 failed. Ran all 5 anti-pattern compliance checks (bg-black, bg-white, bg-clip-text, Inter/system-ui, gray-N) — all zero violations. Confirmed Korean text audit shows only VibeStatus.tsx JSDoc lines 8-10. Verified design token coverage across all 13 component files.

## Verification

| # | Check | Result | Detail |
|---|-------|--------|--------|
| 1 | `npm run build` | ✅ exit 0 | Production build, no TS errors |
| 2 | `npm test` | ✅ 57/57 passed | 5 suites including VibeScoreDashboard "Analyzing..." |
| 3 | Playwright E2E | ✅ 21 passed, 1 skipped, 0 failed | 22 total tests, 228s runtime |
| 4 | bg-black check | ✅ 0 violations | `grep -rn 'bg-black\b' src/ --include="*.tsx"` |
| 5 | bg-white check | ✅ 0 violations | `grep -rn 'bg-white\b' src/ --include="*.tsx"` |
| 6 | bg-clip-text check | ✅ 0 violations | `grep -rn 'bg-clip-text' src/ --include="*.tsx"` |
| 7 | Inter/system-ui check | ✅ 0 violations | Font-family grep on *.tsx + *.css |
| 8 | gray-N check | ✅ 0 violations | `grep -rn 'gray-[0-9]' src/ --include="*.tsx"` |
| 9 | Korean audit | ✅ expected only | VibeStatus.tsx lines 8-10 JSDoc |
| 10 | Design token coverage | ✅ all 13 files >0 | page.tsx(12), ContractInteraction(23), VibeScoreDashboard(24), etc. |

## New Requirements Surfaced

- none

## Deviations

- **jest.config.js testPathIgnorePatterns** — Not in the original plan. Jest was loading the Playwright E2E spec file (which imports `@playwright/test`) and failing. Added `testPathIgnorePatterns: ['<rootDir>/e2e/']` to fix this pre-existing config issue that was unmasked by running `npm test`.

## Known Limitations

- Test 22 (Contract Interaction) remains skipped due to Monad testnet deploy latency (30-90s). This is an infrastructure limitation documented in D008, not a code defect. The test would pass with a pre-funded test wallet and stable testnet.

## Follow-ups

- none — this is the final slice of M004. The milestone is complete.

## Files Created/Modified

- `/home/ahwlsqja/Vibe-Loom/e2e/full-stack.spec.ts` — 8 Korean text selectors replaced with English; Korean alternatives removed from 4 regex patterns
- `/home/ahwlsqja/Vibe-Loom/src/__tests__/VibeScoreDashboard.test.tsx` — Test description and assertion updated from Korean to English
- `/home/ahwlsqja/Vibe-Loom/src/app/layout.tsx` — `text-gray-100` → `text-text-primary`
- `/home/ahwlsqja/Vibe-Loom/jest.config.js` — Added `testPathIgnorePatterns` to exclude `e2e/` from Jest

## Forward Intelligence

### What the next slice should know
- M004 is complete. All 4 slices delivered: design system (S01), component migration (S02), motion + UX copy (S03), and regression verification (S04). The Vibe-Loom UI is fully redesigned with zero functional regressions.
- The design token vocabulary is: `bg-surface-{base,raised,overlay}`, `text-text-{primary,secondary,muted}`, `text-accent`, `border-border-{DEFAULT,subtle}`, `bg-accent`, plus motion tokens `animate-fade-in`, `.btn-press`, `ease-out-expo`.
- Monaco theme is a separate hex-based system (`src/lib/monaco-theme.ts` THEME_COLORS) synced manually with CSS oklch tokens.

### What's fragile
- **Monad testnet E2E tests** — Test 22 (Contract Interaction) is behind a `test.skip()` guard. If testnet becomes more responsive or a pre-funded wallet is configured, the skip should be removed.
- **Monaco theme / CSS token sync** — Hex values in `monaco-theme.ts` must be manually updated when CSS oklch tokens change. No build-time validation exists.

### Authoritative diagnostics
- `npx playwright test --reporter=list` — single command to verify all 22 E2E tests against live site
- `npm test` — 57 unit tests, includes design token compliance assertions
- The 5 anti-pattern grep commands in the Verification table above — canonical design compliance check

### What assumptions changed
- **Original assumption: only E2E selectors needed updating** — Actually, `jest.config.js` also needed a fix to prevent Jest from loading Playwright files. This was a pre-existing issue that T01 discovered and resolved.
