---
id: T02
parent: S01
milestone: M004
provides:
  - Custom Monaco theme 'vibe-loom-dark' registered in both Editor and DiffEditor
  - Hex color constants synced with T01's oklch design tokens (annotated mapping)
  - defineVibeLoomTheme() utility for beforeMount registration pattern
key_files:
  - /home/ahwlsqja/Vibe-Loom/src/lib/monaco-theme.ts
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/MonacoEditorInner.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/AIDiffViewerInner.tsx
key_decisions:
  - Hex constants in THEME_COLORS object with JSDoc comments mapping each to its oklch CSS variable for manual sync
  - Solidity-specific syntax token rules added (keyword.sol, type.sol) alongside general-purpose tokens
  - Diff editor colors (insertedTextBackground, removedTextBackground) included in theme definition for visual consistency
patterns_established:
  - "defineVibeLoomTheme(monaco) called in beforeMount callback — never in onMount — to prevent default theme flash"
  - "THEME_COLORS hex values are the single source of truth for Monaco colors; each annotated with the corresponding CSS variable name from globals.css"
  - "DiffEditor requires its own beforeMount callback (separate from Editor) for theme registration"
observability_surfaces:
  - "Browser console: monaco.editor.getEditors()[0].getOption(monaco.editor.EditorOption.theme) returns 'vibe-loom-dark'"
  - "Theme fallback: if vibe-loom-dark fails to register, editor visually falls back to vs-dark (lighter background)"
  - "Build failure: import errors in monaco-theme.ts surface as Next.js compile error"
duration: 10m
verification_result: passed
completed_at: 2026-03-23
blocker_discovered: false
---

# T02: Create Monaco custom theme and wire into editor components

**Created vibe-loom-dark Monaco theme with design-token-synced hex colors and wired it into both Editor and DiffEditor via beforeMount callbacks**

## What Happened

Created `/home/ahwlsqja/Vibe-Loom/src/lib/monaco-theme.ts` with:
- `THEME_COLORS` — hex constant object with 9 colors, each annotated with its corresponding CSS variable from the T01 @theme block (e.g. `surfaceBase: '#0f1219'` ← `--color-surface-base: oklch(0.145 0.014 260)`)
- `VIBE_LOOM_THEME_NAME = 'vibe-loom-dark'` constant
- `defineVibeLoomTheme(monaco)` function that calls `monaco.editor.defineTheme()` with `base: 'vs-dark'`, `inherit: true`, 20 syntax token rules (keywords, strings, comments, numbers, types, functions, Solidity-specific), and 17 editor chrome colors (background, cursor, selection, line numbers, brackets, widgets, scrollbar, diff)

In `MonacoEditorInner.tsx`: added import, inserted `defineVibeLoomTheme(monaco)` after existing `registerSolidityLanguage(monaco)` in `handleBeforeMount`, changed `theme="vs-dark"` to `theme={VIBE_LOOM_THEME_NAME}`.

In `AIDiffViewerInner.tsx`: added import, created new `handleBeforeMount` callback with `defineVibeLoomTheme(monaco)`, wired it as `beforeMount` prop on `<DiffEditor>`, changed `theme="vs-dark"` to `theme={VIBE_LOOM_THEME_NAME}`.

## Verification

All 7 task-level checks pass. Build succeeds (exit 0). Slice-level: 5/6 pass — the remaining S4 (bg-gray-900 removal from IDELayout) is T03 scope.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cd /home/ahwlsqja/Vibe-Loom && npm run build` | 0 | ✅ pass | 33.8s |
| 2 | `test -f src/lib/monaco-theme.ts` | 0 | ✅ pass | <1s |
| 3 | `grep -q 'vibe-loom-dark' src/lib/monaco-theme.ts` | 0 | ✅ pass | <1s |
| 4 | `grep -q 'VIBE_LOOM_THEME_NAME' src/components/ide/MonacoEditorInner.tsx` | 0 | ✅ pass | <1s |
| 5 | `grep -q 'VIBE_LOOM_THEME_NAME' src/components/ide/AIDiffViewerInner.tsx` | 0 | ✅ pass | <1s |
| 6 | `! grep -q 'theme="vs-dark"' src/components/ide/MonacoEditorInner.tsx` | 0 | ✅ pass | <1s |
| 7 | `! grep -q 'theme="vs-dark"' src/components/ide/AIDiffViewerInner.tsx` | 0 | ✅ pass | <1s |
| 8 | `grep -q '@theme' src/app/globals.css` (slice) | 0 | ✅ pass | <1s |
| 9 | `grep -q 'JetBrains_Mono' src/app/layout.tsx` (slice) | 0 | ✅ pass | <1s |
| 10 | `! grep -q "bg-gray-900" src/components/ide/IDELayout.tsx` (slice) | 1 | ⏳ T03 scope | <1s |
| 11 | Tab labels preserved (Editor/Results/Console) (slice) | 0 | ✅ pass | <1s |

## Diagnostics

- **Inspect theme at runtime:** Browser console: `monaco.editor.getEditors()[0].getOption(monaco.editor.EditorOption.theme)` should return `'vibe-loom-dark'`
- **Visual check:** Editor background should be `#0f1219` (very dark cool blue), not the lighter `vs-dark` default
- **Sync check:** Compare `THEME_COLORS` hex values in `src/lib/monaco-theme.ts` against oklch tokens in `src/app/globals.css` — each hex constant has a JSDoc comment naming its CSS variable counterpart
- **Failure shape:** If theme registration fails, editor falls back to `vs-dark` visually (no crash, just lighter background)

## Deviations

None — all steps followed the task plan exactly.

## Known Issues

- Pre-existing wagmi connector warnings during build (unrelated to this task)
- Slice plan's E2E label grep uses single quotes (`'Editor'`) but source uses double quotes — labels are intact (noted in T01, persists)

## Files Created/Modified

- `/home/ahwlsqja/Vibe-Loom/src/lib/monaco-theme.ts` — New file: THEME_COLORS hex constants, VIBE_LOOM_THEME_NAME, defineVibeLoomTheme() function
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/MonacoEditorInner.tsx` — Added import, defineVibeLoomTheme in handleBeforeMount, theme prop swap
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/AIDiffViewerInner.tsx` — Added import, new handleBeforeMount callback, beforeMount prop, theme prop swap
- `.gsd/milestones/M004/slices/S01/tasks/T02-PLAN.md` — Added Observability Impact section (pre-flight fix)
- `.gsd/milestones/M004/slices/S01/S01-PLAN.md` — Marked T02 as [x]
