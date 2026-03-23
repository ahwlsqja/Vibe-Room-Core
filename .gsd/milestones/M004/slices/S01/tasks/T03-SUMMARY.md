---
id: T03
parent: S01
milestone: M004
provides:
  - All IDE shell components (IDELayout, EditorPanel, SidebarPanel, ConsolePanel) use design token utilities instead of hardcoded Tailwind gray/amber classes
  - Visual foundation complete — surface hierarchy, accent colors, and border tokens applied consistently
key_files:
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/EditorPanel.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/SidebarPanel.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/ConsolePanel.tsx
key_decisions:
  - Removed gradient (bg-gradient-to-r from-gray-800 to-gray-800/90) from SidebarPanel header in favor of flat bg-surface-raised for cleaner visual consistency
  - Used theme() function reference for accent inset shadow in SidebarPanel header
patterns_established:
  - Surface hierarchy convention — bg-surface-base for primary backgrounds, bg-surface-raised for elevated panels/headers/tab bars, bg-surface-overlay for hover states
  - Accent token convention — text-accent/border-accent/bg-accent-bg for active/selected states, replacing all amber-* references
  - Border token convention — border-border-subtle for all structural borders, replacing gray-700
observability_surfaces:
  - "DevTools Computed Styles: filter for --color-surface or --color-accent to see resolved oklch values on any element"
  - "Console: getComputedStyle(document.querySelector('.bg-surface-base')).backgroundColor returns the resolved color"
  - "Build failure: any invalid token reference surfaces as Tailwind parse error in npm run build stderr"
duration: 8min
verification_result: passed
completed_at: 2026-03-23
blocker_discovered: false
---

# T03: Apply design tokens to IDELayout shell and panel components

**Replaced all hardcoded gray/amber Tailwind classes with design-token utilities (bg-surface-base, text-accent, border-border-subtle, etc.) across IDELayout, EditorPanel, SidebarPanel, and ConsolePanel**

## What Happened

Applied the T01 design tokens to all four IDE shell components following the color mapping guide. Each component was edited surgically — only CSS class names changed, no DOM structure or props were modified. Specific replacements:

- **IDELayout.tsx**: Mobile root `bg-gray-900` → `bg-surface-base`, tab bar `bg-gray-800` → `bg-surface-raised`, active tab `bg-amber-600/20 text-amber-400 border-amber-500` → `bg-accent-bg text-accent border-accent`, inactive tab `text-gray-400 hover:text-gray-200 hover:bg-gray-700/50` → `text-text-secondary hover:text-text-primary hover:bg-surface-overlay`. Desktop separators `bg-gray-700 hover:bg-amber-500` → `bg-border-subtle hover:bg-accent`.
- **EditorPanel.tsx**: `bg-gray-900 text-gray-100` → `bg-surface-base text-text-primary`.
- **SidebarPanel.tsx**: `border-gray-700 bg-gray-800` → `border-border-subtle bg-surface-raised`. Header gradient removed in favor of flat `bg-surface-raised`. Inset shadow updated to use `theme(--color-accent/0.1)`. Header text `text-gray-200` → `text-text-primary`.
- **ConsolePanel.tsx**: `border-gray-700 bg-gray-900` → `border-border-subtle bg-surface-base`. Header `bg-gray-800` → `bg-surface-raised`. Text `text-gray-400`/`text-gray-300` → `text-text-secondary`. `font-mono` preserved for JetBrains Mono.

TAB_CONFIG labels ("Editor", "Results", "Console") confirmed unchanged. No DOM structure modifications.

## Verification

All task-level and slice-level verification checks pass. Build compiles successfully with zero errors.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cd /home/ahwlsqja/Vibe-Loom && npm run build` | 0 | ✅ pass | 48s |
| 2 | `! grep -q "bg-gray-900" src/components/ide/IDELayout.tsx` | 0 | ✅ pass | <1s |
| 3 | `! grep -q "bg-gray-800" src/components/ide/IDELayout.tsx` | 0 | ✅ pass | <1s |
| 4 | `grep -q "bg-surface-base" src/components/ide/IDELayout.tsx` | 0 | ✅ pass | <1s |
| 5 | `grep -q '"Editor"' src/components/ide/IDELayout.tsx` | 0 | ✅ pass | <1s |
| 6 | `grep -q '"Results"' src/components/ide/IDELayout.tsx` | 0 | ✅ pass | <1s |
| 7 | `grep -q '"Console"' src/components/ide/IDELayout.tsx` | 0 | ✅ pass | <1s |
| 8 | `grep -q "bg-surface-base" src/components/ide/EditorPanel.tsx` | 0 | ✅ pass | <1s |
| 9 | `grep -q "bg-surface-raised" src/components/ide/SidebarPanel.tsx` | 0 | ✅ pass | <1s |
| 10 | `grep -q "bg-surface-base" src/components/ide/ConsolePanel.tsx` | 0 | ✅ pass | <1s |
| 11 | `grep -q '@theme' src/app/globals.css` | 0 | ✅ pass | <1s |
| 12 | `grep -q 'vibe-loom-dark' src/lib/monaco-theme.ts` | 0 | ✅ pass | <1s |
| 13 | `grep -q 'JetBrains_Mono' src/app/layout.tsx` | 0 | ✅ pass | <1s |
| 14 | `! grep -q "bg-gray-900" src/components/ide/IDELayout.tsx` (slice) | 0 | ✅ pass | <1s |
| 15 | No gray-* remnants in EditorPanel | 0 | ✅ pass | <1s |
| 16 | No gray-* remnants in ConsolePanel | 0 | ✅ pass | <1s |
| 17 | No gray-* remnants in SidebarPanel | 0 | ✅ pass | <1s |
| 18 | No amber-* remnants in IDELayout | 0 | ✅ pass | <1s |

**Note on E2E tab label check**: The slice-level verification command `grep -q "'Editor'"` searches for single-quoted strings, but the source uses double quotes (`"Editor"`). The labels are confirmed intact — the grep just needs `'"Editor"'` syntax to match. Not a real failure.

## Diagnostics

- **Inspect token application**: DevTools → select any IDE panel element → Computed → filter `surface` or `accent` to see resolved oklch values
- **Confirm no hardcoded colors remain**: `grep -rn "gray-\|amber-" src/components/ide/` should return zero results for the four modified files
- **Build health**: `npm run build` — any invalid token reference surfaces as a Tailwind parse error
- **Failure shape**: If a token is undefined, Tailwind generates no utility class → element gets no background/color → visually transparent (no crash)

## Deviations

- SidebarPanel header shadow uses `theme(--color-accent/0.1)` instead of the plan's suggested `theme(colors.accent/0.1)` — Tailwind v4 uses CSS variable references with `theme()`, not the v3 dot-path syntax.

## Known Issues

- Slice-level E2E tab label verification commands use single-quote grep patterns (`'Editor'`) that don't match the double-quoted source. The actual labels are unchanged — this is a verification command syntax issue, not a code issue.

## Files Created/Modified

- `/home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx` — Replaced all bg-gray-900/bg-gray-800/amber-*/gray-* with design token utilities (bg-surface-base, bg-surface-raised, text-accent, etc.)
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/EditorPanel.tsx` — bg-gray-900 text-gray-100 → bg-surface-base text-text-primary
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/SidebarPanel.tsx` — Replaced gray/amber classes with tokens, removed gradient in favor of flat bg-surface-raised
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/ConsolePanel.tsx` — Replaced gray classes with tokens, preserved font-mono for JetBrains Mono
