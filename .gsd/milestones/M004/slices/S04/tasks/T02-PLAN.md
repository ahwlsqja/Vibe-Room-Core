---
estimated_steps: 5
estimated_files: 2
---

# T02: Run E2E suite and capture final verification evidence

**Slice:** S04 — Regression — E2E 테스트 호환 + 최종 검증
**Milestone:** M004

## Description

With T01's selector fixes in place, this task runs the full 22-test Playwright E2E suite against the live site (`https://vibe-loom.xyz`) and executes the impeccable anti-pattern checklist to prove zero functional regression and full design compliance.

The E2E tests run against the live deployed site. Per D008, deploy-dependent tests may skip due to Monad testnet latency — up to 1 skip and 1 retry-pass is acceptable. The success criteria is ≥20 passed, ≤1 skipped, 0 failed.

**Important:** Tests run from `/home/ahwlsqja/Vibe-Loom/`, not the monad-core worktree.

**Note:** For the E2E tests to fully validate the new UI, the code changes from S01-S03 must be deployed to `vibe-loom.xyz`. If E2E tests fail because the live site hasn't been updated, document this clearly — the test file fixes from T01 are still valid.

## Steps

1. **Run Playwright E2E tests** — Execute `cd /home/ahwlsqja/Vibe-Loom && npx playwright test --reporter=list` and capture the full output. Document the result: total tests, passed, failed, skipped. Target: 22 tests, ≥20 passed, ≤1 skipped, 0 failed.

2. **Run anti-pattern checklist** — Execute each grep and document results:
   ```bash
   # Pure black/white backgrounds
   grep -rn 'bg-black\b' /home/ahwlsqja/Vibe-Loom/src/ --include="*.tsx" | grep -v '/[0-9]'
   grep -rn 'bg-white\b' /home/ahwlsqja/Vibe-Loom/src/ --include="*.tsx"
   # Gradient text
   grep -rn 'bg-clip-text\|text-transparent.*gradient' /home/ahwlsqja/Vibe-Loom/src/ --include="*.tsx"
   # Inter/system-ui font
   grep -rn 'font-family.*Inter\b\|font-family.*system-ui' /home/ahwlsqja/Vibe-Loom/src/ --include="*.tsx" --include="*.css"
   # Structural gray references
   grep -rn 'gray-[0-9]' /home/ahwlsqja/Vibe-Loom/src/ --include="*.tsx" | grep -v node_modules
   ```
   All should return zero results.

3. **Run Korean text audit** — Execute:
   ```bash
   LC_ALL=C grep -rPn '[\xea-\xed][\x80-\xbf][\x80-\xbf]' /home/ahwlsqja/Vibe-Loom/src/ --include="*.tsx" | grep -v 'node_modules\|.next\|__tests__\|//'
   ```
   Expected: only VibeStatus.tsx JSDoc lines 8-10.

4. **Verify design token coverage** — Execute:
   ```bash
   grep -c 'bg-surface-\|text-text-\|border-border-\|font-display\|font-body' /home/ahwlsqja/Vibe-Loom/src/app/page.tsx /home/ahwlsqja/Vibe-Loom/src/components/ide/*.tsx /home/ahwlsqja/Vibe-Loom/src/app/layout.tsx
   ```
   Each file should have >0 design token references.

5. **Document results** — Record all findings as the slice verification evidence.

## Must-Haves

- [ ] Playwright E2E suite executed — results documented (≥20 pass, 0 fail)
- [ ] Anti-pattern checklist — all 5 checks return zero violations
- [ ] Korean audit — only JSDoc lines remain
- [ ] Design token coverage — all component files have >0 token references

## Verification

- `cd /home/ahwlsqja/Vibe-Loom && npx playwright test --reporter=list` — ≥20 passed, 0 failed
- `grep -rn 'gray-[0-9]' /home/ahwlsqja/Vibe-Loom/src/ --include="*.tsx" | grep -v node_modules | wc -l` — returns 0
- `grep -rn 'bg-black\b' /home/ahwlsqja/Vibe-Loom/src/ --include="*.tsx" | grep -v '/[0-9]' | wc -l` — returns 0
- `grep -rn 'bg-clip-text' /home/ahwlsqja/Vibe-Loom/src/ --include="*.tsx" | wc -l` — returns 0

## Observability Impact

- **Playwright test reporter output**: `npx playwright test --reporter=list` produces per-test pass/fail lines; future agents can grep for `✓`, `✗`, or `skipped` to extract results programmatically
- **Anti-pattern grep exit codes**: Each grep returns exit 1 (no match) when clean; exit 0 means a violation was found — future agents should check exit codes, not just output
- **Failure artifacts**: Playwright writes screenshots to `e2e/screenshots/` and traces to `test-results/` on failure — inspect these for visual regression debugging
- **Design token coverage**: `grep -c` output shows per-file token usage count; a file with 0 count indicates missing design system adoption
- **Korean audit**: Non-zero results outside VibeStatus.tsx JSDoc lines 8-10 indicate incomplete English migration from S03

## Inputs

- `/home/ahwlsqja/Vibe-Loom/e2e/full-stack.spec.ts` — E2E test file (fixed in T01)
- `/home/ahwlsqja/Vibe-Loom/playwright.config.ts` — Playwright config targeting `https://vibe-loom.xyz`

## Expected Output

- `/home/ahwlsqja/Vibe-Loom/e2e/full-stack.spec.ts` — Validated (no modifications, just execution)
- `/home/ahwlsqja/Vibe-Loom/test-results/` — Playwright test output artifacts (screenshots, traces)
