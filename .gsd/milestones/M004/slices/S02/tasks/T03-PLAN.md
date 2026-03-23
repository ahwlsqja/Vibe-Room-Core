---
estimated_steps: 4
estimated_files: 3
---

# T03: Migrate trivial files and run full-slice verification

**Slice:** S02 — Core Components — 에디터 + 사이드바 + 콘솔 리팩토링
**Milestone:** M004

## Description

Complete the final 3 trivial file migrations (one-line changes each — loading placeholder text color) and run comprehensive verification across all 10 migrated files to confirm the slice is done. This task closes the loop on the entire S02 scope.

**Design Token Mapping:**
- `text-gray-500` → `text-text-muted` (loading placeholder text)

## Steps

1. **Migrate `src/components/ide/AIDiffViewer.tsx`** (~32 lines, 1 legacy ref):
   - Replace `text-gray-500` with `text-text-muted` in the loading placeholder

2. **Migrate `src/components/ide/MonacoEditor.tsx`** (~35 lines, 1 legacy ref):
   - Replace `text-gray-500` with `text-text-muted` in the loading placeholder

3. **Migrate `src/components/ide/MonacoEditorInner.tsx`** (~75 lines, 1 legacy ref):
   - Replace `text-gray-500` with `text-text-muted` in the loading placeholder

4. **Run full-slice verification:**
   - `npm run build` must exit 0
   - Run the comprehensive 10-file legacy color grep:
     ```bash
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
     Must return 0 results for gray. **Allowed semantic amber:** `text-amber-*` / `bg-amber-*` / `border-amber-*` in ContractInteraction.tsx (write function distinction), VibeStatus.tsx (status indicator), and page.tsx deploy button.
   - Gradient anti-pattern check: `grep -rn "bg-clip-text\|text-transparent.*gradient" src/app/page.tsx` must return 0
   - Token presence check: `grep -c "bg-surface-\|text-text-\|text-accent\|border-border-" src/app/page.tsx` must return > 0

## Must-Haves

- [ ] All 3 trivial files have `text-text-muted` instead of `text-gray-500`
- [ ] `npm run build` passes
- [ ] Full 10-file grep confirms zero structural gray/amber legacy refs
- [ ] Gradient anti-pattern confirmed removed from page.tsx

## Verification

- `npm run build` exits 0
- `grep -c "gray-[0-9]" src/components/ide/AIDiffViewer.tsx src/components/ide/MonacoEditor.tsx src/components/ide/MonacoEditorInner.tsx` returns 0 for each
- Full 10-file legacy grep returns 0 results (or only semantic amber in ContractInteraction write functions)
- `grep -rn "bg-clip-text\|text-transparent.*gradient" src/app/page.tsx` returns 0
- `grep -c "text-text-muted" src/components/ide/AIDiffViewer.tsx` returns > 0

## Inputs

- `src/components/ide/AIDiffViewer.tsx` — 32 lines, 1 legacy ref (text-gray-500)
- `src/components/ide/MonacoEditor.tsx` — 35 lines, 1 legacy ref (text-gray-500)
- `src/components/ide/MonacoEditorInner.tsx` — 75 lines, 1 legacy ref (text-gray-500)
- `src/components/ide/ContractInteraction.tsx` — T01 output, verify migration
- `src/components/ide/VibeScoreDashboard.tsx` — T01 output, verify migration
- `src/app/page.tsx` — T01 output, verify migration
- `src/components/WalletConnectModal.tsx` — T02 output, verify migration
- `src/components/VibeStatus.tsx` — T02 output, verify migration
- `src/components/ide/TransactionConsole.tsx` — T02 output, verify migration
- `src/components/ide/AIDiffViewerInner.tsx` — T02 output, verify migration

## Expected Output

- `src/components/ide/AIDiffViewer.tsx` — text-gray-500 → text-text-muted
- `src/components/ide/MonacoEditor.tsx` — text-gray-500 → text-text-muted
- `src/components/ide/MonacoEditorInner.tsx` — text-gray-500 → text-text-muted
