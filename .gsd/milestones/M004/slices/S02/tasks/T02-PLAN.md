---
estimated_steps: 4
estimated_files: 4
---

# T02: Migrate WalletConnectModal, VibeStatus, TransactionConsole, and AIDiffViewerInner to design tokens

**Slice:** S02 — Core Components — 에디터 + 사이드바 + 콘솔 리팩토링
**Milestone:** M004

## Description

Migrate the 4 medium-complexity component files from hardcoded Tailwind gray/amber utilities to the design token vocabulary. These files cover the wallet connection modal, paymaster status badge, transaction log, and diff viewer wrapper. Combined ~28 legacy refs.

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
- Only replace gray/amber used for STRUCTURAL or ACCENT purposes
- Zero functional changes — only className strings modified
- Note: WalletConnectModal is at `src/components/WalletConnectModal.tsx` (NOT in /ide/ subfolder)
- Note: VibeStatus is at `src/components/VibeStatus.tsx` (NOT in /ide/ subfolder)

**Relevant skills to load:** `frontend-design`

## Steps

1. **Migrate `src/components/WalletConnectModal.tsx`** (~201 lines, ~10 legacy refs):
   - `bg-gray-800` → `bg-surface-raised` (modal background)
   - `bg-gray-900` → `bg-surface-base` (inner panels)
   - `border-gray-700` → `border-border-subtle`
   - `border-gray-600` → `border-border-subtle`
   - `bg-gray-700` → `bg-surface-overlay` (connector buttons)
   - `text-gray-400`, `text-gray-500` → `text-text-secondary`
   - `text-gray-200`, `text-gray-300` → `text-text-primary`
   - `text-amber-400` (title) → `text-accent`
   - Keep `emerald-*`, `red-*` for success/error states

2. **Migrate `src/components/VibeStatus.tsx`** (~87 lines, ~6 legacy refs):
   - `bg-gray-800` → `bg-surface-raised` (wrapper)
   - `border-gray-700` → `border-border-subtle`
   - `text-gray-400` → `text-text-secondary`
   - Keep `emerald-*`, `amber-*` for conditional status indicators (these are semantic)

3. **Migrate `src/components/ide/TransactionConsole.tsx`** (~102 lines, ~8 legacy refs):
   - `text-gray-500` → `text-text-muted` (empty state, timestamps)
   - `text-gray-300` → `text-text-primary` (messages)
   - `text-gray-400` → `text-text-secondary` (detail text)
   - `bg-gray-950/80` → `bg-surface-base` (detail pre blocks)
   - Keep status color config objects (emerald/red/amber) — these are semantic

4. **Migrate `src/components/ide/AIDiffViewerInner.tsx`** (~79 lines, ~4 legacy refs):
   - `bg-gray-800 border-gray-700` → `bg-surface-raised border-border-subtle` (wrapper)
   - `bg-amber-900/30 border-gray-700` → `bg-accent-bg border-border-subtle` (summary banner — amber was accent usage)
   - `bg-gray-900/50 border-gray-700` → `bg-surface-base/50 border-border-subtle` (button area)
   - `text-gray-500` → `text-text-muted` (loading)

## Must-Haves

- [ ] WalletConnectModal.tsx has zero `gray-[0-9]` or `amber-[0-9]` matches
- [ ] VibeStatus.tsx has zero `gray-[0-9]` matches (semantic amber/emerald stays)
- [ ] TransactionConsole.tsx has zero `gray-[0-9]` matches
- [ ] AIDiffViewerInner.tsx has zero `gray-[0-9]` or `amber-[0-9]` matches
- [ ] `npm run build` passes
- [ ] No component interface, prop, state, or event handler changes

## Verification

- `npm run build` exits 0
- `grep -c "gray-[0-9]" src/components/WalletConnectModal.tsx` returns 0
- `grep -c "gray-[0-9]" src/components/VibeStatus.tsx` returns 0
- `grep -c "gray-[0-9]" src/components/ide/TransactionConsole.tsx` returns 0
- `grep -c "gray-[0-9]" src/components/ide/AIDiffViewerInner.tsx` returns 0

## Inputs

- `src/components/WalletConnectModal.tsx` — 201 lines, ~10 legacy refs
- `src/components/VibeStatus.tsx` — 87 lines, ~6 legacy refs
- `src/components/ide/TransactionConsole.tsx` — 102 lines, ~8 legacy refs
- `src/components/ide/AIDiffViewerInner.tsx` — 79 lines, ~4 legacy refs
- `src/app/globals.css` — Reference for token vocabulary

## Expected Output

- `src/components/WalletConnectModal.tsx` — All structural gray/amber replaced with token utilities
- `src/components/VibeStatus.tsx` — All structural gray replaced, semantic amber/emerald preserved
- `src/components/ide/TransactionConsole.tsx` — All structural gray replaced with token utilities
- `src/components/ide/AIDiffViewerInner.tsx` — All structural gray/amber replaced with token utilities

## Observability Impact

- **Signals changed:** Structural color classes in 4 component files replaced with design token utilities — zero runtime behavior change, only visual class names differ.
- **Inspection:** Run `grep -c "gray-[0-9]" <file>` on any migrated file to confirm 0 legacy references. Run `grep -c "bg-surface-\|text-text-\|border-border-\|text-accent\|border-accent\|bg-accent" <file>` to confirm token adoption (>0 expected).
- **Failure visibility:** Misspelled token names in Tailwind v4 produce no build error — the class becomes a no-op. Visual inspection at `localhost:3000` is the definitive check for subtle regressions.
- **No new runtime signals:** No new console logs, error boundaries, or metrics — this is a className-only migration.
