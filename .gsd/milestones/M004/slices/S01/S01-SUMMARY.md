---
id: S01
parent: M004
milestone: M004
provides:
  - Tailwind v4 @theme design tokens — 24 total (11 colors in oklch, 2 fonts, 6 spacing, 5 motion)
  - Custom font loading (Geist Sans + JetBrains Mono) via next/font/google with CSS variable bridging
  - Custom Monaco editor theme 'vibe-loom-dark' with design-token-synced hex colors (20 syntax rules, 17 chrome colors)
  - IDE shell components (IDELayout, EditorPanel, SidebarPanel, ConsolePanel) fully migrated to design token utilities
  - Surface hierarchy convention (base → raised → overlay) established for all components
requires:
  - slice: none
    provides: first slice — no dependencies
affects:
  - S02
key_files:
  - /home/ahwlsqja/Vibe-Loom/src/app/globals.css
  - /home/ahwlsqja/Vibe-Loom/src/app/layout.tsx
  - /home/ahwlsqja/Vibe-Loom/src/lib/monaco-theme.ts
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/MonacoEditorInner.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/AIDiffViewerInner.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/EditorPanel.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/SidebarPanel.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/ConsolePanel.tsx
key_decisions:
  - oklch color space for perceptual uniformity across surface hierarchy (D012)
  - Tailwind v4 @theme block as CSS-first single source of truth for all tokens (D013)
  - Monaco hex constants with JSDoc-annotated CSS variable mapping for manual sync (D014)
  - 3-tier surface hierarchy — base/raised/overlay (D015)
  - Additive token strategy — no --color-*: initial to preserve Tailwind defaults during migration
  - SidebarPanel gradient removed in favor of flat bg-surface-raised for cleaner consistency
patterns_established:
  - "@theme block in globals.css is the single source of truth for design tokens — 24 tokens across 4 categories"
  - "Surface hierarchy: bg-surface-base (primary) → bg-surface-raised (panels/headers/tab bars) → bg-surface-overlay (hover states)"
  - "Accent convention: text-accent / border-accent / bg-accent-bg for active/selected states"
  - "Border convention: border-border-subtle for all structural borders"
  - "Text convention: text-text-primary / text-text-secondary for content hierarchy"
  - "next/font variable names bridged to @theme via --font-sans: var(--font-geist-sans)"
  - "defineVibeLoomTheme(monaco) called in beforeMount — never onMount — to prevent default theme flash"
  - "THEME_COLORS hex object is the single source of truth for Monaco colors, each annotated with CSS variable counterpart"
observability_surfaces:
  - "DevTools Computed Styles: filter for --color-surface or --color-accent on any element"
  - "Console: getComputedStyle(document.documentElement).getPropertyValue('--color-accent') returns oklch value"
  - "Console: getComputedStyle(document.body).fontFamily confirms Geist Sans"
  - "Console: monaco.editor.getEditors()[0].getOption(monaco.editor.EditorOption.theme) returns 'vibe-loom-dark'"
  - "Build failure: Tailwind @theme parse error in npm run build stderr"
  - "Monaco fallback: theme registration failure → editor falls back to vs-dark (lighter background, no crash)"
drill_down_paths:
  - .gsd/milestones/M004/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M004/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M004/slices/S01/tasks/T03-SUMMARY.md
duration: 33min
verification_result: passed
completed_at: 2026-03-23
---

# S01: Design Foundation — 디자인 시스템 + 레이아웃 셸

**Defined 24 design tokens (oklch colors, fonts, spacing, motion) in Tailwind v4 @theme, loaded custom fonts, created vibe-loom-dark Monaco theme, and migrated all 4 IDE shell components from hardcoded Tailwind gray/amber to token-based utilities**

## What Happened

Built the complete design foundation for the Vibe-Loom IDE redesign in three sequential tasks:

**T01 — Design Tokens + Fonts** (15min): Added a `@theme` block to `globals.css` with 24 tokens — 11 colors in oklch (3-tier surface hierarchy at hue 260, teal accent at hue 185, semantic border/text), 2 font tokens bridged to `next/font/google` CSS variables (Geist Sans for UI, JetBrains Mono for code), 6 spacing tokens (`--space-*`), and 5 motion tokens (2 easing curves, 3 durations). All tokens are additive — no `--color-*: initial` declarations, preserving existing Tailwind gray/amber utilities for gradual migration. Layout.tsx loads both fonts with `display: 'swap'` and binds CSS variables to `<html>`.

**T02 — Monaco Theme** (10min): Created `src/lib/monaco-theme.ts` with a `THEME_COLORS` hex constant object (9 colors, each annotated with its oklch CSS variable counterpart), `VIBE_LOOM_THEME_NAME = 'vibe-loom-dark'`, and `defineVibeLoomTheme()` function. The theme includes 20 syntax token rules (keywords, strings, comments, types, functions, Solidity-specific) and 17 editor chrome colors (background, cursor, selection, line numbers, brackets, widgets, scrollbar, diff). Wired into both `MonacoEditorInner.tsx` and `AIDiffViewerInner.tsx` via `beforeMount` callbacks, replacing `vs-dark`.

**T03 — Shell Component Migration** (8min): Replaced all hardcoded `bg-gray-900`, `bg-gray-800`, `amber-*`, `gray-700`, `gray-400` etc. across `IDELayout.tsx`, `EditorPanel.tsx`, `SidebarPanel.tsx`, and `ConsolePanel.tsx` with design token utilities. Established surface hierarchy convention: `bg-surface-base` for primary backgrounds, `bg-surface-raised` for panels/headers/tab bars, `bg-surface-overlay` for hover states. Removed SidebarPanel's gradient header in favor of flat `bg-surface-raised`. All tab labels ("Editor", "Results", "Console") and DOM structure preserved exactly.

## Verification

**Build**: `npm run build` completes successfully (exit 0) with zero errors. Pre-existing wagmi connector warnings are unrelated.

**Slice-level checks** (6/6 pass):
| # | Check | Result |
|---|-------|--------|
| 1 | `@theme` block in globals.css | ✅ |
| 2 | `vibe-loom-dark` in monaco-theme.ts | ✅ |
| 3 | `JetBrains_Mono` in layout.tsx | ✅ |
| 4 | `bg-gray-900` removed from IDELayout.tsx | ✅ |
| 5 | Tab labels (Editor/Results/Console) preserved | ✅ |
| 6 | `npm run build` success | ✅ |

**Extended checks**:
- Zero `gray-*` or `amber-*` remnants in IDELayout, EditorPanel, SidebarPanel, ConsolePanel
- `bg-surface-base` present in all 4 components
- `VIBE_LOOM_THEME_NAME` wired in both MonacoEditorInner and AIDiffViewerInner
- No `theme="vs-dark"` remaining in either editor component

## Requirements Advanced

- **R016** — 디자인 토큰 시스템과 레이아웃 셸이 완성됨. S02~S04의 기반 확립. 전체 4/13 컴포넌트가 새 토큰으로 마이그레이션됨.

## New Requirements Surfaced

- none

## Deviations

- SidebarPanel header shadow uses `theme(--color-accent/0.1)` instead of `theme(colors.accent/0.1)` — Tailwind v4 uses CSS variable references, not v3 dot-path syntax. The plan's suggested syntax was incorrect for v4.
- Slice plan's E2E label verification grep used single quotes (`'Editor'`) but source uses double quotes (`"Editor"`). Labels are intact — verification command syntax issue only.

## Known Limitations

- **Monaco-CSS sync is manual**: THEME_COLORS hex values in `monaco-theme.ts` must be updated by hand when oklch tokens in `globals.css` change. No build-time automation.
- **Remaining gray/amber usage**: Other components outside the 4 shell components (ContractInteraction, VibeScoreDashboard, AIErrorAnalysis, etc.) still use hardcoded gray/amber — deferred to S02.
- **No visual verification**: Build and grep checks pass, but actual visual rendering has not been confirmed in a browser (UAT required).

## Follow-ups

- S02 must consume the surface/accent/border/text token conventions and apply them to the remaining 9+ components
- Monaco THEME_COLORS hex values should be double-checked against resolved oklch values in a browser's DevTools color picker
- Consider build-time oklch→hex extraction if frequent token changes cause sync drift

## Files Created/Modified

- `/home/ahwlsqja/Vibe-Loom/src/app/globals.css` — Added @theme block with 24 design tokens (oklch colors, font bridges, spacing, motion); updated panel-transition and focus-visible utilities
- `/home/ahwlsqja/Vibe-Loom/src/app/layout.tsx` — Added Geist Sans + JetBrains Mono font loading via next/font/google, CSS variable binding, body styling with tokens
- `/home/ahwlsqja/Vibe-Loom/src/lib/monaco-theme.ts` — **New file**: THEME_COLORS hex constants, VIBE_LOOM_THEME_NAME, defineVibeLoomTheme() with 20 syntax rules + 17 chrome colors
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/MonacoEditorInner.tsx` — defineVibeLoomTheme in beforeMount, theme prop swap from vs-dark
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/AIDiffViewerInner.tsx` — New beforeMount callback with defineVibeLoomTheme, theme prop swap
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx` — All gray/amber → token utilities (bg-surface-base, text-accent, border-border-subtle, etc.)
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/EditorPanel.tsx` — bg-gray-900 text-gray-100 → bg-surface-base text-text-primary
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/SidebarPanel.tsx` — Gray/amber → tokens, gradient removed for flat bg-surface-raised
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/ConsolePanel.tsx` — Gray → tokens, font-mono preserved for JetBrains Mono

## Forward Intelligence

### What the next slice should know
- The design token vocabulary is: `bg-surface-{base,raised,overlay}`, `text-{text-primary,text-secondary}`, `border-border-subtle`, `text-accent`, `border-accent`, `bg-accent-bg`. Use these exact utilities — do not invent new ones.
- `font-sans` resolves to Geist Sans, `font-mono` resolves to JetBrains Mono. No additional font-family declarations needed.
- Motion tokens are defined but **unused** so far: `--ease-snappy`, `--ease-smooth`, `--duration-fast`, `--duration-normal`, `--duration-slow`. S03 should consume these for entry animations and interaction feedback.

### What's fragile
- **Monaco hex ↔ oklch sync** — If S02 changes any color token in globals.css, the hex values in `src/lib/monaco-theme.ts` THEME_COLORS must be updated manually. Each hex has a JSDoc comment naming its CSS variable counterpart.
- **Tailwind v4 @theme syntax** — No `--color-*: initial` declarations exist. Adding one would wipe out all default Tailwind colors and break any component still using `bg-gray-*` or `text-gray-*`.

### Authoritative diagnostics
- `npm run build` exit code — zero means @theme parses correctly and all token references resolve. Any invalid token reference creates a Tailwind parse error in stderr.
- `grep -rn "gray-\|amber-" src/components/ide/{IDELayout,EditorPanel,SidebarPanel,ConsolePanel}.tsx` — should return 0 results for the 4 migrated files. Any match means a token migration regression.
- DevTools Computed Styles filtering for `--color-` on any element shows the full resolved token set.

### What assumptions changed
- **Plan assumed** `theme(colors.accent/0.1)` v3 dot-path syntax would work — **actually** Tailwind v4 requires `theme(--color-accent/0.1)` CSS variable reference syntax.
- **Plan's verification grep** used single quotes for tab labels — **actually** source code uses double quotes. Labels are intact, but any future verification should grep for `'"Editor"'` not `"'Editor'"`.
