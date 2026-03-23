---
estimated_steps: 5
estimated_files: 3
---

# T01: Migrate ContractInteraction, VibeScoreDashboard, and page.tsx to design tokens

**Slice:** S02 — Core Components — 에디터 + 사이드바 + 콘솔 리팩토링
**Milestone:** M004

## Description

Migrate the 3 largest and most visually impactful component files from hardcoded Tailwind gray/amber/cyan utilities to the design token vocabulary established in S01. This covers ~70% of all legacy color references in the slice. Includes fixing the gradient text anti-pattern on the page title and updating SVG hex colors in VibeScoreDashboard.

**Design Token Vocabulary (use these exact utilities — do NOT invent new ones):**

| Purpose | Utility |
|---------|---------|
| Primary background | `bg-surface-base` |
| Panel/header/card bg | `bg-surface-raised` |
| Hover/elevated state | `bg-surface-overlay` |
| Structural borders | `border-border-subtle` |
| Active border | `border-accent` |
| Accent text | `text-accent` |
| Muted accent | `text-accent-muted` |
| Accent background | `bg-accent-bg` |
| Primary text | `text-text-primary` |
| Secondary text | `text-text-secondary` |
| Muted text | `text-text-muted` |

**Rules:**
- Semantic status colors STAY: `emerald-*` (success), `red-*` (error), `amber-*` (warning/pending in status-dependent contexts)
- Only replace gray/amber/cyan used for STRUCTURAL or ACCENT purposes
- Zero functional changes — only className strings modified
- `blue-600` for compile button → replace with accent or keep — be consistent with other action buttons
- Tailwind v4 `theme()` syntax: `theme(--color-accent/0.1)` NOT `theme(colors.accent/0.1)`

**Relevant skills to load:** `frontend-design`, `make-interfaces-feel-better`

## Steps

1. **Migrate `src/components/ide/ContractInteraction.tsx`** (~448 lines, ~27 legacy refs):
   - `bg-gray-800` → `bg-surface-raised` (card backgrounds)
   - `bg-gray-900` → `bg-surface-base` (input fields, inner areas)
   - `border-gray-700` → `border-border-subtle`
   - `border-gray-600` → `border-border-subtle`
   - `text-gray-400`, `text-gray-500`, `text-gray-600` → `text-text-secondary` or `text-text-muted` (context-dependent)
   - `text-gray-200`, `text-gray-300` → `text-text-primary`
   - `text-cyan-*` → `text-accent` (for read function names/labels)
   - `bg-cyan-700 hover:bg-cyan-600` → `bg-accent hover:bg-accent/80` or similar accent button style
   - `focus:border-cyan-500` → `focus:border-accent`
   - Keep `text-amber-*` for write function names (read=accent, write=amber is a meaningful semantic distinction)
   - Keep `emerald-*`, `red-*` for success/error states

2. **Migrate `src/components/ide/VibeScoreDashboard.tsx`** (~205 lines, ~27 legacy refs):
   - `bg-gray-800` → `bg-surface-raised` (wrapper)
   - `bg-gray-900` → `bg-surface-base` (stat boxes)
   - `border-gray-700` → `border-border-subtle`
   - `text-gray-400`, `text-gray-500` → `text-text-secondary`
   - `text-gray-100` → `text-text-primary`
   - `bg-gray-700 animate-pulse` → `bg-surface-overlay animate-pulse` (loading skeleton)
   - SVG hex `#374151` (gray-700 background ring) → `#3d3a4e` (matches oklch 0.30 0.014 260 = border-subtle)
   - Keep `#34d399` (emerald), `#fbbf24` (amber), `#f87171` (red) — these are semantic score colors

3. **Migrate `src/app/page.tsx`** (~394 lines, ~11 legacy refs + gradient anti-pattern):
   - **Remove gradient anti-pattern**: Replace `text-transparent bg-clip-text bg-gradient-to-r from-amber-500 to-orange-400` on the title with `text-accent font-bold`
   - `border-gray-700 bg-gray-800` (toolbar) → `border-border-subtle bg-surface-raised`
   - `bg-amber-600` (contract selector active) → `bg-accent text-surface-base` (structural selected state)
   - `bg-gray-700` (selector inactive, logout, vibe score btn) → `bg-surface-overlay text-text-secondary`
   - `bg-gray-800 border-gray-600` (auth area) → `bg-surface-raised border-border-subtle`
   - `text-gray-400` → `text-text-secondary`
   - `bg-blue-600` (compile button) → `bg-accent hover:bg-accent/80` (unify action buttons to accent)
   - `bg-amber-600` (deploy button) → keep amber for deploy action (semantic: deploy = caution)
   - `bg-gray-600` (vibe score button) → `bg-surface-overlay`
   - Keep `emerald-*` / `red-*` for success/error states in sidebar content

4. **Run `npm run build`** and confirm exit 0.

5. **Run verification greps** to confirm zero legacy gray/amber refs in all 3 files and gradient anti-pattern removed.

## Must-Haves

- [ ] ContractInteraction.tsx has zero `gray-[0-9]` or `amber-[0-9]` matches (except `text-amber-*` for write function names which is semantic)
- [ ] VibeScoreDashboard.tsx has zero `gray-[0-9]` matches and SVG background ring hex updated from `#374151`
- [ ] page.tsx has zero `gray-[0-9]`, zero `bg-clip-text`, zero gradient text
- [ ] `npm run build` passes
- [ ] No component interface, prop, state, or event handler changes — only className strings

## Verification

- `npm run build` exits 0
- `grep -c "gray-[0-9]" src/components/ide/ContractInteraction.tsx` returns 0
- `grep -c "gray-[0-9]" src/components/ide/VibeScoreDashboard.tsx` returns 0
- `grep -c "gray-[0-9]" src/app/page.tsx` returns 0
- `grep -c "bg-clip-text" src/app/page.tsx` returns 0
- `grep -c "bg-surface-\|text-text-\|text-accent\|border-border-" src/components/ide/ContractInteraction.tsx` returns > 0

## Inputs

- `src/components/ide/ContractInteraction.tsx` — 448 lines, ~27 legacy gray/amber/cyan refs to replace
- `src/components/ide/VibeScoreDashboard.tsx` — 205 lines, ~27 legacy gray refs + 4 SVG hex colors
- `src/app/page.tsx` — 394 lines, ~11 legacy gray/amber refs + gradient text anti-pattern
- `src/app/globals.css` — Reference for token vocabulary (@theme block with 24 tokens)

## Expected Output

- `src/components/ide/ContractInteraction.tsx` — All structural gray/cyan replaced with token utilities, amber kept for write function semantic distinction
- `src/components/ide/VibeScoreDashboard.tsx` — All structural gray replaced, SVG background ring hex updated to `#3d3a4e`
- `src/app/page.tsx` — Gradient anti-pattern removed, all structural gray/amber replaced with token utilities

## Observability Impact

- **Signals changed:** className strings only — no runtime state, event handlers, or data flow changes. No new console.log, metrics, or error boundaries introduced.
- **Inspection surface:** Run `grep -c "gray-[0-9]" <file>` on each of the 3 files to confirm migration. Run `grep -c "bg-surface-\|text-text-\|text-accent\|border-border-" <file>` to confirm token adoption (should return >0).
- **Failure state visibility:** Misspelled token names produce no build error in Tailwind v4 — they become no-op classes. Visual inspection at `localhost:3000` is the ultimate check. The SVG hex change (`#374151` → `#3d3a4e`) is only visible in VibeScoreDashboard's gauge background ring.
- **No new runtime observability:** This task is purely presentational; the existing `onCallResult` callbacks, console.error guards, and React DevTools inspection surfaces remain unchanged.
