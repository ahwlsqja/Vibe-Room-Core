# S02: Core Components — 에디터 + 사이드바 + 콘솔 리팩토링

**Goal:** All 10 remaining component files (9 components + page.tsx) migrated from hardcoded Tailwind gray/amber/cyan utilities to the S01 design token vocabulary. Zero legacy color references in migrated files. Gradient text anti-pattern removed.
**Demo:** `npm run build` passes, `grep -rn "gray-[0-9]\|amber-[0-9]" <all 10 files>` returns 0 results (excluding semantic emerald/red status colors which stay), and the gradient text anti-pattern on page.tsx is eliminated.

## Must-Haves

- All 10 files migrated to design token utilities (`bg-surface-{base,raised,overlay}`, `text-{text-primary,text-secondary,muted}`, `border-border-subtle`, `text-accent`, `border-accent`, `bg-accent-bg`)
- Gradient text anti-pattern (`text-transparent bg-clip-text bg-gradient-to-r from-amber-500 to-orange-400`) removed from page.tsx title
- VibeScoreDashboard SVG background ring hex `#374151` updated to token-aligned value
- Semantic status colors preserved: emerald (success), red (error), amber (warning/pending) — only structural gray/amber/cyan replaced
- Zero functional changes: no component interface, state logic, event handler, or prop changes
- `npm run build` exits 0

## Verification

- `npm run build` exits 0 with zero errors
- `grep -rn "gray-[0-9]" src/components/ide/ContractInteraction.tsx src/components/ide/VibeScoreDashboard.tsx src/components/ide/TransactionConsole.tsx src/components/ide/AIDiffViewer.tsx src/components/ide/AIDiffViewerInner.tsx src/components/ide/MonacoEditor.tsx src/components/ide/MonacoEditorInner.tsx src/app/page.tsx src/components/VibeStatus.tsx src/components/WalletConnectModal.tsx` returns 0 results (all structural gray eliminated)
- Semantic amber usage is allowed: `text-amber-*` in ContractInteraction (write function distinction), `amber-*` in VibeStatus (status indicator), `bg-amber-600` in page.tsx (deploy button)
- `grep -rn "bg-clip-text\|text-transparent.*gradient" src/app/page.tsx` returns 0 results
- `grep -rn "bg-surface-\|text-text-\|border-border-\|text-accent\|border-accent\|bg-accent" src/app/page.tsx src/components/ide/ContractInteraction.tsx src/components/ide/VibeScoreDashboard.tsx` returns multiple results (tokens actually used)

## Integration Closure

- Upstream surfaces consumed: S01 design tokens in `src/app/globals.css` (@theme block), surface hierarchy convention (base→raised→overlay), Monaco theme in `src/lib/monaco-theme.ts`
- New wiring introduced in this slice: none — only className string replacements
- What remains before the milestone is truly usable end-to-end: S03 (motion/polish/mobile), S04 (E2E regression + final verification)

## Tasks

- [x] **T01: Migrate ContractInteraction, VibeScoreDashboard, and page.tsx to design tokens** `est:25m`
  - Why: These 3 files contain ~70% of all legacy color references (60+ refs) and are the most visually prominent components. ContractInteraction is the main sidebar content, VibeScoreDashboard is the score display, and page.tsx contains the toolbar with the gradient text anti-pattern.
  - Files: `src/components/ide/ContractInteraction.tsx`, `src/components/ide/VibeScoreDashboard.tsx`, `src/app/page.tsx`
  - Do: Replace all structural gray/amber/cyan with token utilities following the mapping in the research. Fix gradient anti-pattern on page title. Update VibeScoreDashboard SVG background ring hex. Preserve semantic emerald/red/amber for status indicators. Keep all component interfaces, props, state, and event handlers identical. Relevant skills: `frontend-design`, `make-interfaces-feel-better`.
  - Verify: `npm run build` passes AND `grep -c "gray-[0-9]\|amber-[0-9]" src/components/ide/ContractInteraction.tsx src/components/ide/VibeScoreDashboard.tsx src/app/page.tsx` returns 0 for each file AND `grep -c "bg-clip-text" src/app/page.tsx` returns 0
  - Done when: All 3 files use only token utilities for structural colors, gradient anti-pattern is gone, build succeeds

- [x] **T02: Migrate WalletConnectModal, VibeStatus, TransactionConsole, and AIDiffViewerInner to design tokens** `est:15m`
  - Why: These 4 medium-sized files (28 legacy refs combined) complete the non-trivial component migration. They cover the modal, status badge, console log, and diff viewer wrapper.
  - Files: `src/components/WalletConnectModal.tsx`, `src/components/VibeStatus.tsx`, `src/components/ide/TransactionConsole.tsx`, `src/components/ide/AIDiffViewerInner.tsx`
  - Do: Replace all structural gray/amber with token utilities. Preserve semantic emerald/red/amber for status indicators. Keep all component interfaces identical. Relevant skills: `frontend-design`.
  - Verify: `npm run build` passes AND `grep -c "gray-[0-9]\|amber-[0-9]" src/components/WalletConnectModal.tsx src/components/VibeStatus.tsx src/components/ide/TransactionConsole.tsx src/components/ide/AIDiffViewerInner.tsx` returns 0 for each file
  - Done when: All 4 files use only token utilities for structural colors, build succeeds

- [x] **T03: Migrate trivial files and run full-slice verification** `est:10m`
  - Why: Three files have single loading-placeholder color refs. This task completes them and runs the comprehensive verification to confirm the entire slice is done — zero legacy refs across all 10 files, build passes, anti-pattern gone.
  - Files: `src/components/ide/AIDiffViewer.tsx`, `src/components/ide/MonacoEditor.tsx`, `src/components/ide/MonacoEditorInner.tsx`
  - Do: Replace `text-gray-500` with `text-text-muted` in each file's loading placeholder. Run full-slice grep verification across all 10 files. Run `npm run build`. Confirm gradient anti-pattern removed.
  - Verify: `npm run build` passes AND full 10-file grep returns 0 legacy refs AND `grep -rn "bg-clip-text\|text-transparent.*gradient" src/app/page.tsx` returns 0
  - Done when: All 10 files fully migrated, zero legacy gray/amber references, build clean, gradient anti-pattern eliminated

## Observability / Diagnostics

- **Inspection surface:** `grep -rn "gray-[0-9]" <file>` on any migrated file returns 0 results to confirm migration completeness.
- **Token adoption:** `grep -c "bg-surface-\|text-text-\|border-border-\|text-accent\|border-accent\|bg-accent" <file>` returns >0 to confirm tokens are actually used.
- **Gradient anti-pattern:** `grep -rn "bg-clip-text\|text-transparent.*gradient" src/app/page.tsx` returns 0 after fix.
- **Build health:** `npm run build` exits 0 — pre-existing wagmi connector warnings are unrelated to this slice.
- **Visual regression:** No automated visual diff; manual browser check at `localhost:3000` after migration to verify no visual breakage.
- **Failure visibility:** If a token name is misspelled, Tailwind v4 build will not error but the class will be a no-op — visual inspection required for subtle regressions.
- **Redaction:** No secrets or sensitive data involved in this slice.

## Files Likely Touched

- `src/components/ide/ContractInteraction.tsx`
- `src/components/ide/VibeScoreDashboard.tsx`
- `src/app/page.tsx`
- `src/components/WalletConnectModal.tsx`
- `src/components/VibeStatus.tsx`
- `src/components/ide/TransactionConsole.tsx`
- `src/components/ide/AIDiffViewerInner.tsx`
- `src/components/ide/AIDiffViewer.tsx`
- `src/components/ide/MonacoEditor.tsx`
- `src/components/ide/MonacoEditorInner.tsx`
