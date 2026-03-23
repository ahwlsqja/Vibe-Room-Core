# S04: Regression — E2E 테스트 호환 + 최종 검증 — UAT

**Milestone:** M004
**Written:** 2026-03-23

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: All checks are automated (E2E suite, unit tests, grep-based anti-pattern audits). No human-experience testing required — this slice is purely regression verification, not new feature delivery.

## Preconditions

- Vibe-Loom dev dependencies installed (`cd /home/ahwlsqja/Vibe-Loom && npm install`)
- Playwright browsers installed (`npx playwright install chromium`)
- Live site `https://vibe-loom.xyz` is accessible
- Railway backend is running (for API-dependent E2E tests)

## Smoke Test

Run `cd /home/ahwlsqja/Vibe-Loom && npm run build && npm test` — should exit 0 with 57 tests passed across 5 suites. If this fails, stop and investigate before proceeding.

## Test Cases

### 1. Build succeeds with no TypeScript errors

1. `cd /home/ahwlsqja/Vibe-Loom && npm run build`
2. **Expected:** Exit code 0. Output shows "Route (app)" table with page sizes. No TypeScript or Next.js compilation errors.

### 2. Unit tests pass including English assertion

1. `cd /home/ahwlsqja/Vibe-Loom && npm test -- --verbose`
2. **Expected:** 5 suites, 57 tests, all passed. Look for test name containing `"Analyzing..."` in VibeScoreDashboard suite — confirms Korean→English update took effect.

### 3. Playwright E2E suite — zero failures

1. `cd /home/ahwlsqja/Vibe-Loom && npx playwright test --reporter=list`
2. **Expected:** 22 tests listed. ≥20 passed. ≤1 skipped (Contract Interaction may skip due to testnet timeout). 0 failed.
3. If any test fails, check `test-results/` for Playwright traces and `e2e/screenshots/` for visual state.

### 4. Anti-pattern: no pure black backgrounds

1. `cd /home/ahwlsqja/Vibe-Loom && grep -rn 'bg-black\b' src/ --include="*.tsx" | grep -v '/[0-9]'`
2. **Expected:** No output (exit code 1). Zero matches.

### 5. Anti-pattern: no pure white backgrounds

1. `cd /home/ahwlsqja/Vibe-Loom && grep -rn 'bg-white\b' src/ --include="*.tsx"`
2. **Expected:** No output (exit code 1). Zero matches.

### 6. Anti-pattern: no gradient text

1. `cd /home/ahwlsqja/Vibe-Loom && grep -rn 'bg-clip-text' src/ --include="*.tsx"`
2. **Expected:** No output (exit code 1). Zero matches.

### 7. Anti-pattern: no Inter/system-ui font declarations

1. `cd /home/ahwlsqja/Vibe-Loom && grep -rn 'font-family.*Inter\b\|font-family.*system-ui' src/ --include="*.tsx" --include="*.css"`
2. **Expected:** No output (exit code 1). Zero matches.

### 8. Anti-pattern: no structural gray-N references

1. `cd /home/ahwlsqja/Vibe-Loom && grep -rn 'gray-[0-9]' src/ --include="*.tsx" | grep -v node_modules`
2. **Expected:** No output (exit code 1). Zero matches.

### 9. Korean text audit — only JSDoc expected

1. `cd /home/ahwlsqja/Vibe-Loom && LC_ALL=C grep -rPn '[\xea-\xed][\x80-\xbf][\x80-\xbf]' src/ --include="*.tsx" | grep -v 'node_modules\|.next\|__tests__\|//'`
2. **Expected:** Only 3 lines from `src/components/VibeStatus.tsx` lines 8-10 (JSDoc comments). No user-facing Korean text.

### 10. Design token coverage — all components using tokens

1. `cd /home/ahwlsqja/Vibe-Loom && for f in src/app/page.tsx src/components/ide/*.tsx src/app/layout.tsx; do count=$(grep -c 'bg-surface-\|text-text-\|text-accent\|border-border\|bg-accent\|text-muted\|shadow-glow\|animate-fade\|btn-press\|font-display\|font-body' "$f" 2>/dev/null || echo 0); printf "%-50s %s\n" "$f" "$count"; done`
2. **Expected:** Every file shows count >0. Key files: page.tsx ≥10, ContractInteraction.tsx ≥20, VibeScoreDashboard.tsx ≥20.

## Edge Cases

### E2E test 22 (Contract Interaction) skip

1. Check Playwright output for test 22.
2. **Expected:** Shows as "skipped" (not "failed"). This is correct — the Monad testnet deploy timeout causes a `test.skip()` guard to activate. Documented in D008.

### Jest does not load Playwright spec

1. `cd /home/ahwlsqja/Vibe-Loom && npm test 2>&1 | grep -i "playwright\|e2e"`
2. **Expected:** No output about Playwright or e2e. The `testPathIgnorePatterns` in `jest.config.js` prevents Jest from loading `e2e/full-stack.spec.ts`.

## Failure Signals

- `npm run build` exit code ≠ 0 → TypeScript errors from text changes
- `npm test` showing failed tests → Unit test assertions not aligned with English UX copy
- Any Playwright test with status "failed" → E2E selector mismatch against live UI
- Any anti-pattern grep returning matches → Design token migration incomplete
- Korean text in user-facing strings (not JSDoc) → S03 UX copy migration missed a string
- Any component file with 0 design token references → S02 migration missed a file

## Not Proven By This UAT

- Visual design quality (typography, spacing, color harmony) — requires human visual inspection of screenshots
- Animation smoothness and timing — requires interactive browser testing
- Mobile layout correctness at 375×812 — E2E tests only verify desktop viewport
- Monaco editor theme rendering — E2E tests don't inspect editor syntax highlighting
- Performance impact of the redesign (bundle size, load time) — not measured

## Notes for Tester

- The E2E suite runs against the live site (`vibe-loom.xyz`), so network connectivity and backend availability affect results.
- Test 22 skip is expected behavior, not a defect. If Monad testnet is responsive at test time, the test may pass instead of skip — both outcomes are valid.
- Screenshots in `e2e/screenshots/` provide visual evidence of the redesigned UI at each test step — useful for spot-checking design quality even though it's not formally proven by this UAT.
- The deploy flow test (test 13) occasionally needs a retry due to testnet latency — Playwright's retry mechanism handles this automatically.
