---
id: T01
parent: S01
milestone: M004
provides:
  - Tailwind v4 @theme design tokens (11 colors, 2 fonts, 6 spacing, 5 motion)
  - Custom font loading (Geist Sans + JetBrains Mono) via next/font/google
  - CSS variable bridge between next/font and Tailwind @theme
key_files:
  - /home/ahwlsqja/Vibe-Loom/src/app/globals.css
  - /home/ahwlsqja/Vibe-Loom/src/app/layout.tsx
key_decisions:
  - oklch color space for all custom colors (better perceptual uniformity than hex/hsl)
  - Additive token strategy — no --color-*: initial to preserve Tailwind defaults
  - Cool blue hue 260 for surfaces, teal hue 185 for accents (Bloomberg Terminal aesthetic)
patterns_established:
  - "@theme block in globals.css is the single source of truth for design tokens"
  - "next/font variable names (--font-geist-sans, --font-geist-mono) bridged to Tailwind via @theme --font-sans/--font-mono"
  - "Panel transition timing uses var(--ease-snappy), focus-visible uses var(--color-accent)"
observability_surfaces:
  - "DevTools Computed Styles: CSS custom properties from @theme block on any element"
  - "Browser console: getComputedStyle(document.body).fontFamily confirms Geist Sans"
  - "Build failure: Tailwind parse error in stderr if @theme syntax breaks"
duration: 15m
verification_result: passed
completed_at: 2026-03-23
blocker_discovered: false
---

# T01: Define Tailwind v4 design tokens in globals.css and load custom fonts in layout.tsx

**Added @theme design system tokens (24 total: colors, fonts, spacing, motion) and loaded Geist Sans + JetBrains Mono fonts with CSS variable bridging to Tailwind v4**

## What Happened

Implemented the design token foundation for the Vibe-Loom IDE redesign using Tailwind v4's CSS-first `@theme` directive. All tokens are additive — no existing Tailwind default colors were overridden with `initial`, preserving existing `bg-gray-900` / `text-gray-300` utilities used throughout the codebase.

In `globals.css`: added a `@theme` block right after `@import "tailwindcss"` with 11 color tokens (surface hierarchy, border, accent, text), 2 font tokens bridged to next/font CSS variables, 6 spacing tokens in a `--space-*` namespace, and 5 motion tokens (2 easing curves, 3 durations). Updated existing `panel-transition` to use `var(--ease-snappy)` and `focus-visible` outline to use `var(--color-accent)`.

In `layout.tsx`: loaded Geist (sans) and JetBrains Mono (mono) via `next/font/google` with `display: 'swap'` for FOUT prevention. Font CSS variables are bound to `<html>` className. Body receives `bg-surface-base text-gray-100 font-sans` for immediate visual effect.

## Verification

All 5 task-level verification checks pass. Build completes successfully (exit 0, with pre-existing wagmi connector warnings unrelated to this task). Slice-level checks: 3/6 pass (T01 scope), 2 expected to fail (T02 Monaco theme, T03 gray-900 removal), 1 has a pre-existing quote-style mismatch in plan (labels use double quotes in source but plan greps for single quotes — labels are intact).

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cd /home/ahwlsqja/Vibe-Loom && npm run build` | 0 | ✅ pass | 43.8s |
| 2 | `grep -q '@theme' src/app/globals.css` | 0 | ✅ pass | <1s |
| 3 | `grep -q 'JetBrains_Mono' src/app/layout.tsx` | 0 | ✅ pass | <1s |
| 4 | `grep -c 'color-surface\|color-accent\|color-border\|color-text' src/app/globals.css` (>=10) | 0 (count=12) | ✅ pass | <1s |
| 5 | `! grep -q 'color-.*: initial' src/app/globals.css` | 0 | ✅ pass | <1s |
| 6 | `grep -q 'vibe-loom-dark' src/lib/monaco-theme.ts` (slice check) | 1 | ⏳ T02 scope | <1s |
| 7 | `! grep -q "bg-gray-900" src/components/ide/IDELayout.tsx` (slice check) | 1 | ⏳ T03 scope | <1s |
| 8 | E2E tab label grep (slice check) | 1 | ⚠️ plan uses single quotes, source uses double — labels intact | <1s |

## Diagnostics

- **Inspect tokens:** Open DevTools → select any element → Computed → filter for `--color-` or `--font-` to see resolved values
- **Confirm font loading:** Console: `getComputedStyle(document.body).fontFamily` should include "Geist" when fonts are loaded
- **Confirm token count:** `grep -c 'color-surface\|color-accent\|color-border\|color-text' src/app/globals.css` should return >= 10
- **Build health:** `npm run build` in Vibe-Loom root — any @theme syntax error surfaces as Tailwind parse error

## Deviations

- Reworded a comment in globals.css that contained the literal string `--color-*: initial` which triggered a false positive on the verification grep `! grep -q 'color-.*: initial'`. The comment now avoids that pattern while preserving the same guidance.

## Known Issues

- Slice plan's E2E label verification grep uses single quotes (`'Editor'`) but source code uses double quotes (`"Editor"`). The labels themselves are unchanged — this is a plan authoring issue, not a regression. T03 should note this when running slice verification.

## Files Created/Modified

- `/home/ahwlsqja/Vibe-Loom/src/app/globals.css` — Added @theme block with 24 design tokens (colors, fonts, spacing, motion); updated panel-transition and focus-visible to use token variables
- `/home/ahwlsqja/Vibe-Loom/src/app/layout.tsx` — Added Geist Sans + JetBrains Mono font loading via next/font/google, CSS variable binding on <html>, body styling with new tokens
- `.gsd/milestones/M004/slices/S01/S01-PLAN.md` — Added Observability / Diagnostics section
- `.gsd/milestones/M004/slices/S01/tasks/T01-PLAN.md` — Added Observability Impact section
