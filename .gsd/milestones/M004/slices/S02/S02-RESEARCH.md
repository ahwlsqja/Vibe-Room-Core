# S02: Core Components — 에디터 + 사이드바 + 콘솔 리팩토링 — Research

**Date:** 2026-03-23
**Depth:** Light research — straightforward application of established design token patterns from S01 to remaining components. No new technology, no ambiguous scope.

## Summary

S02 migrates the remaining 7 IDE components + 2 non-IDE components + page.tsx from hardcoded Tailwind gray/amber/cyan utilities to the design token vocabulary established in S01. The 4 shell components (IDELayout, EditorPanel, SidebarPanel, ConsolePanel) are already migrated — S02 covers everything that renders *inside* them.

There are **137 legacy color references** across 10 files (33 in ContractInteraction, 30 in VibeScoreDashboard, 22 in page.tsx, 18 in WalletConnectModal, 13 in VibeStatus, 10 in TransactionConsole, 8 in AIDiffViewerInner, and 1 each in AIDiffViewer/MonacoEditor/MonacoEditorInner). One impeccable anti-pattern exists: gradient text on the "Monad Vibe-Loom" title in page.tsx. VibeScoreDashboard has 4 hardcoded hex colors in SVG that need updating.

The work is purely cosmetic — zero API changes, zero component interface changes, zero state logic changes. Every edit is a className string replacement following the exact token vocabulary from S01.

## Recommendation

Apply the S01 token vocabulary (`bg-surface-{base,raised,overlay}`, `text-{text-primary,text-secondary,muted}`, `border-border-subtle`, `text-accent`, `border-accent`, `bg-accent-bg`) for all structural gray/amber usage. **Keep** Tailwind's semantic palette (emerald for success, red for error) — these carry meaning and the token system intentionally left them untouched. Replace cyan accent usage with `text-accent`/`border-accent` since the accent hue (185 teal/cyan) matches the existing cyan intent. Remove the gradient text anti-pattern on the page title.

Group work by file complexity: tackle the 3 big files first (ContractInteraction, VibeScoreDashboard, page.tsx), then the 4 medium files (TransactionConsole, WalletConnectModal, VibeStatus, AIDiffViewerInner), then the 3 trivial ones (AIDiffViewer, MonacoEditor, MonacoEditorInner — just loading placeholder text color).

## Implementation Landscape

### Key Files

**Files to migrate (ordered by legacy reference count):**

- `src/components/ide/ContractInteraction.tsx` (448 lines, 33 legacy refs) — ABI-based function call UI. Read function cards use cyan, write cards use amber, success uses emerald, error uses red. Has `ReadFunctionCard`, `WriteFunctionCard`, `FunctionInput`, `Spinner` sub-components. Card backgrounds are `bg-gray-800 border-gray-700`, input fields are `bg-gray-900 border-gray-600`. **Mapping:** `bg-gray-800` → `bg-surface-raised`, `bg-gray-900` → `bg-surface-base`, `border-gray-700` → `border-border-subtle`, `border-gray-600` → `border-border-subtle`, `text-gray-400/500/600` → `text-text-secondary`/`text-text-muted`, `text-gray-200/300` → `text-text-primary`, `text-cyan-*` → `text-accent`, `bg-cyan-700` → accent button style, `text-amber-300` → `text-accent` for write fn names (or keep amber for read/write distinction — see constraint below).

- `src/components/ide/VibeScoreDashboard.tsx` (205 lines, 30 legacy refs) — Circular SVG gauge + stats grid + suggestion cards. **Has 4 hardcoded hex values** for SVG stroke: `#34d399` (emerald-400), `#fbbf24` (amber-400), `#f87171` (red-400), `#374151` (gray-700 background ring). Also uses `bg-gray-800` wrapper, `bg-gray-900` stat boxes, `border-gray-700` borders, `text-gray-400/500/100` text. Loading skeleton uses `bg-gray-700 animate-pulse`. **Mapping:** wrapper `bg-gray-800` → `bg-surface-raised`, stat boxes `bg-gray-900` → `bg-surface-base`, SVG background ring `#374151` → should use a token-aligned hex (oklch 0.30 at hue 260 ≈ `#3d3a4e`). The score-dependent emerald/amber/red colors are **semantic** and should stay.

- `src/app/page.tsx` (394 lines, 22 legacy refs) — Toolbar + sidebar content inline JSX. **Contains gradient text anti-pattern** on "Monad Vibe-Loom" title (`text-transparent bg-clip-text bg-gradient-to-r from-amber-500 to-orange-400`) — must be replaced with `text-accent font-bold`. Contract selector buttons use `bg-amber-600`/`bg-gray-700`. Action buttons: Compile=`bg-blue-600`, Deploy=`bg-amber-600`, Vibe Score=`bg-gray-600`. Auth area uses `bg-gray-800 border-gray-600`. Sidebar inline content uses emerald/red/amber for deploy result/error/AI fix states. **Mapping:** toolbar `border-gray-700 bg-gray-800` → `border-border-subtle bg-surface-raised`, selector active `bg-amber-600` → `bg-accent text-surface-base`, selector inactive `bg-gray-700` → `bg-surface-overlay text-text-secondary`, logout `bg-gray-700` → `bg-surface-overlay`, login `bg-gray-800 border-gray-600` → `bg-surface-raised border-border-subtle`. Compile/Deploy/Vibe Score buttons can use accent variants. Keep emerald/red for success/error states.

- `src/components/WalletConnectModal.tsx` (~170 lines, 18 legacy refs) — Modal overlay for wallet deploys. Uses `bg-gray-800 border-gray-700` for modal, `bg-gray-900 border-gray-600` for inner panels, `text-gray-400/500/300/200` for text, `bg-gray-700` for connector buttons, `text-amber-400` for title. **Mapping:** modal `bg-gray-800` → `bg-surface-raised`, inner `bg-gray-900` → `bg-surface-base`, buttons `bg-gray-700` → `bg-surface-overlay`, title amber → `text-accent`. Keep emerald/red for success/error.

- `src/components/VibeStatus.tsx` (~80 lines, 13 legacy refs) — Paymaster status badge + progress bar. Uses `bg-gray-800 border-gray-700` wrapper, `text-gray-400` text. Emerald/amber for status conditionals are **semantic** — keep them. **Mapping:** `bg-gray-800` → `bg-surface-raised`, `border-gray-700` → `border-border-subtle`, `text-gray-400` → `text-text-secondary`.

- `src/components/ide/TransactionConsole.tsx` (102 lines, 10 legacy refs) — Transaction log entry list. Uses `text-gray-500` for empty state and timestamps, `text-gray-300` for messages, `text-gray-400` for detail text, `bg-gray-950/80` for detail pre blocks. Status colors (emerald/red/amber) are semantic config objects — keep them. **Mapping:** `text-gray-500` → `text-text-muted`, `text-gray-300` → `text-text-primary`, `text-gray-400` → `text-text-secondary`, `bg-gray-950/80` → `bg-surface-base`.

- `src/components/ide/AIDiffViewerInner.tsx` (79 lines, 8 legacy refs) — Monaco diff editor wrapper. Uses `bg-gray-800 border-gray-700` for wrapper, `bg-amber-900/30 border-gray-700` for summary banner, `bg-gray-900/50 border-gray-700` for button area, `text-gray-500` for loading. **Mapping:** wrapper `bg-gray-800` → `bg-surface-raised`, summary `bg-amber-900/30` → keep amber semantic or use accent-bg, button area `bg-gray-900/50` → `bg-surface-base/50`, loading `text-gray-500` → `text-text-muted`.

- `src/components/ide/AIDiffViewer.tsx` (32 lines, 1 legacy ref) — Loading placeholder only: `text-gray-500` → `text-text-muted`.

- `src/components/ide/MonacoEditor.tsx` (35 lines, 1 legacy ref) — Loading placeholder only: `text-gray-500` → `text-text-muted`.

- `src/components/ide/MonacoEditorInner.tsx` (75 lines, 1 legacy ref) — Loading placeholder only: `text-gray-500` → `text-text-muted`.

### Token Vocabulary Reference (from S01)

Established in `globals.css` @theme block — use these exact utility names:

| Purpose | Utility | Token |
|---------|---------|-------|
| Primary background | `bg-surface-base` | oklch(0.145 0.014 260) |
| Panel/header/card bg | `bg-surface-raised` | oklch(0.185 0.014 260) |
| Hover/elevated state | `bg-surface-overlay` | oklch(0.22 0.014 260) |
| Structural borders | `border-border-subtle` | oklch(0.30 0.014 260) |
| Active border | `border-accent` | oklch(0.75 0.15 185) |
| Accent text | `text-accent` | oklch(0.75 0.15 185) |
| Muted accent | `text-accent-muted` | oklch(0.55 0.12 185) |
| Accent background | `bg-accent-bg` | oklch(0.25 0.06 185) |
| Active border | `border-border-active` | oklch(0.55 0.15 185) |
| Primary text | `text-text-primary` | oklch(0.93 0.005 260) |
| Secondary text | `text-text-secondary` | oklch(0.65 0.01 260) |
| Muted text | `text-text-muted` | oklch(0.50 0.01 260) |
| Body font | `font-sans` | Geist Sans |
| Code font | `font-mono` | JetBrains Mono |

### Build Order

1. **ContractInteraction.tsx + VibeScoreDashboard.tsx** — Highest legacy count (63 combined). These are the most visually impactful sidebar components. Independent of each other.
2. **page.tsx** — 22 refs including the gradient text anti-pattern. The toolbar is the most prominent UI element.
3. **TransactionConsole.tsx + AIDiffViewerInner.tsx** — Console/diff content (18 combined). Independent.
4. **WalletConnectModal.tsx + VibeStatus.tsx** — Non-IDE components (31 combined). Independent.
5. **AIDiffViewer.tsx + MonacoEditor.tsx + MonacoEditorInner.tsx** — Trivial (3 refs total, all just loading text color).
6. **Build verification** — `npm run build` + zero legacy refs in all files.

### Verification Approach

**Primary:** `npm run build` must exit 0 with zero errors.

**Secondary:** grep-based token migration completeness check:
```bash
# Must return 0 results — all gray-*/amber-* eliminated from all 10 migrated files
grep -rn "gray-[0-9]\|amber-[0-9]" \
  src/components/ide/ContractInteraction.tsx \
  src/components/ide/VibeScoreDashboard.tsx \
  src/components/ide/TransactionConsole.tsx \
  src/components/ide/AIDiffViewer.tsx \
  src/components/ide/AIDiffViewerInner.tsx \
  src/components/ide/MonacoEditor.tsx \
  src/components/ide/MonacoEditorInner.tsx \
  src/app/page.tsx \
  src/components/VibeStatus.tsx \
  src/components/WalletConnectModal.tsx
```

**Tertiary:** gradient text anti-pattern removed:
```bash
# Must return 0 results
grep -rn "bg-clip-text\|text-transparent.*gradient" src/app/page.tsx
```

**Functional preservation:** Tab labels, button text, component structure, component props, event handlers — all unchanged. Only className strings modified.

## Constraints

- **Token vocabulary is fixed** — S01 defined the exact utilities. Do NOT invent new token names. If a color need doesn't map to an existing token, use the closest token or keep the Tailwind semantic palette (emerald/red/blue/purple).
- **Semantic status colors stay** — `emerald-*` (success), `red-*` (error), `amber-*` (warning/pending) for state-dependent UI are intentional and meaningful. Only replace gray-*/amber-* used for structural/accent purposes, not for semantic status.
- **Read vs Write function distinction in ContractInteraction** — Currently cyan=read, amber=write. Both should map to accent for read functions. Write functions can use a slightly different treatment (e.g., `text-accent-muted` or keep a distinguishing color). The simplest approach: read functions use `text-accent`, write functions keep `text-amber-*` or use `text-text-primary` with a different indicator.
- **SVG hex colors in VibeScoreDashboard** — The SVG `stroke` attribute requires hex literals, not CSS utilities. The background ring hex `#374151` (gray-700) should be updated to match `--color-border-subtle` resolved hex. Score-dependent colors (emerald/amber/red) are semantic and can stay.
- **No `--color-*: initial`** — Adding this to @theme would wipe all Tailwind default colors. Tokens are additive only.
- **Tailwind v4 `theme()` syntax** — Use `theme(--color-accent/0.1)` not `theme(colors.accent/0.1)`.

## Common Pitfalls

- **Replacing semantic amber with accent** — page.tsx uses `bg-amber-600` for both the Deploy button (semantic: deploy action) AND the contract selector active state (structural: selected tab). Only the structural usage should change to accent. Deploy button amber is action-semantic and could either stay or switch to accent — but be consistent.
- **VibeScoreDashboard hardcoded hex** — The 4 hex values in the SVG JSX (`#34d399`, `#fbbf24`, `#f87171`, `#374151`) are easy to miss because they don't match the `gray-*` grep pattern. The background ring `#374151` must be updated; the score colors are semantic.
- **Loading placeholder consistency** — Three files (AIDiffViewer, MonacoEditor, MonacoEditorInner) have `text-gray-500` in loading placeholders. These should all become `text-text-muted` for consistency.
- **WalletConnectModal is outside /ide/ folder** — It's at `src/components/WalletConnectModal.tsx`, not `src/components/ide/`. Don't miss it.
- **VibeStatus is outside /ide/ folder** — Same: `src/components/VibeStatus.tsx`.
