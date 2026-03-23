# S02: Core Components — 에디터 + 사이드바 + 콘솔 리팩토링 — UAT

**Milestone:** M004
**Written:** 2026-03-23

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: This slice is purely a className string replacement (CSS token migration). No functional changes, no new state logic, no new APIs. Verification is structural (grep for legacy patterns) + build (compilation succeeds) + visual (rendered appearance matches design intent). Live runtime testing is deferred to S04 where E2E tests validate the full integrated flow.

## Preconditions

- Vibe-Loom repository at `/home/ahwlsqja/Vibe-Loom` has S01 design tokens in `src/app/globals.css` (@theme block)
- `npm install` has been run (node_modules present)
- `npm run build` succeeds as baseline

## Smoke Test

Run `grep -rn "gray-[0-9]" src/components/ide/ContractInteraction.tsx src/components/ide/VibeScoreDashboard.tsx src/components/ide/TransactionConsole.tsx src/components/ide/AIDiffViewer.tsx src/components/ide/AIDiffViewerInner.tsx src/components/ide/MonacoEditor.tsx src/components/ide/MonacoEditorInner.tsx src/app/page.tsx src/components/VibeStatus.tsx src/components/WalletConnectModal.tsx` — must return 0 results. If any structural gray remains, migration is incomplete.

## Test Cases

### 1. Zero structural gray references across all 10 files

1. `cd /home/ahwlsqja/Vibe-Loom`
2. Run: `grep -rn "gray-[0-9]" src/components/ide/ContractInteraction.tsx src/components/ide/VibeScoreDashboard.tsx src/components/ide/TransactionConsole.tsx src/components/ide/AIDiffViewer.tsx src/components/ide/AIDiffViewerInner.tsx src/components/ide/MonacoEditor.tsx src/components/ide/MonacoEditorInner.tsx src/app/page.tsx src/components/VibeStatus.tsx src/components/WalletConnectModal.tsx`
3. **Expected:** Exit code 1 (no matches). Zero lines of output.

### 2. Gradient text anti-pattern eliminated from page.tsx

1. Run: `grep -rn "bg-clip-text\|text-transparent.*gradient" src/app/page.tsx`
2. **Expected:** Exit code 1 (no matches). The page title should use `text-accent` instead.
3. Confirm replacement: `grep -n "text-accent" src/app/page.tsx`
4. **Expected:** At least one match on the page title `<h1>` element.

### 3. Design tokens actively adopted in key components

1. Run: `grep -c "bg-surface-\|text-text-\|border-border-\|text-accent\|border-accent\|bg-accent" src/app/page.tsx`
2. **Expected:** ≥7 matches (page.tsx has 9 token references)
3. Run: `grep -c "bg-surface-\|text-text-\|border-border-\|text-accent\|border-accent\|bg-accent" src/components/ide/ContractInteraction.tsx`
4. **Expected:** ≥20 matches (ContractInteraction has 22 token references)
5. Run: `grep -c "bg-surface-\|text-text-\|border-border-\|text-accent\|border-accent\|bg-accent" src/components/ide/VibeScoreDashboard.tsx`
6. **Expected:** ≥20 matches (VibeScoreDashboard has 24 token references)

### 4. Semantic status colors preserved

1. Run: `grep -n "emerald" src/components/ide/ContractInteraction.tsx`
2. **Expected:** Emerald references present (success indicators for function call results)
3. Run: `grep -n "red-" src/components/ide/ContractInteraction.tsx`
4. **Expected:** Red references present (error indicators for function call failures)
5. Run: `grep -n "amber-" src/components/ide/ContractInteraction.tsx`
6. **Expected:** Amber references present (write function name distinction: `text-amber-300`)
7. Run: `grep -n "amber-600" src/app/page.tsx`
8. **Expected:** At least one match (deploy button retains bg-amber-600)

### 5. SVG hex color updated in VibeScoreDashboard

1. Run: `grep -n "#374151" src/components/ide/VibeScoreDashboard.tsx`
2. **Expected:** Zero matches (old gray-700 hex removed)
3. Run: `grep -n "#3d3a4e" src/components/ide/VibeScoreDashboard.tsx`
4. **Expected:** At least one match (new oklch-aligned border-subtle hex present)

### 6. Build succeeds with zero errors

1. Run: `cd /home/ahwlsqja/Vibe-Loom && npm run build`
2. **Expected:** Exit code 0. Route table shows `○ /` with ~101 kB size. Pre-existing wagmi connector warnings are expected and not errors.

### 7. Loading placeholder text uses design tokens

1. Run: `grep -n "text-text-muted" src/components/ide/AIDiffViewer.tsx src/components/ide/MonacoEditor.tsx src/components/ide/MonacoEditorInner.tsx`
2. **Expected:** Each file has exactly 1 match on its loading placeholder div.
3. Run: `grep -n "text-gray-500" src/components/ide/AIDiffViewer.tsx src/components/ide/MonacoEditor.tsx src/components/ide/MonacoEditorInner.tsx`
4. **Expected:** Zero matches (legacy gray removed).

## Edge Cases

### Amber references are all semantic (not structural)

1. Run: `grep -n "amber-[0-9]" src/components/ide/ContractInteraction.tsx src/app/page.tsx src/components/VibeStatus.tsx src/components/WalletConnectModal.tsx src/components/ide/AIDiffViewerInner.tsx src/components/ide/TransactionConsole.tsx src/components/ide/VibeScoreDashboard.tsx`
2. **Expected:** All matches are semantic uses: write function names (text-amber-300), deploy buttons (bg-amber-600), status indicators (amber-400/500), score colors (#fbbf24). No structural gray-replacement amber should exist.

### No cyan references remain in migrated files

1. Run: `grep -rn "cyan-[0-9]" src/components/ide/ContractInteraction.tsx src/components/ide/VibeScoreDashboard.tsx src/app/page.tsx src/components/WalletConnectModal.tsx`
2. **Expected:** Zero matches. All structural cyan should be replaced with `text-accent`, `bg-accent`, or `focus:ring-accent`.

## Failure Signals

- `grep "gray-[0-9]"` returning any match on the 10 migrated files → migration incomplete
- `npm run build` failing with exit code >0 → token name typo or syntax error
- `grep "bg-clip-text"` returning matches on page.tsx → gradient anti-pattern not removed
- Token adoption grep returning 0 on a major component file → tokens not actually applied (possible revert)
- Visual inspection at localhost:3000 showing default browser colors (black text on white bg) → misspelled token names (Tailwind v4 no-op classes)

## Not Proven By This UAT

- **Visual appearance quality** — This UAT verifies structural correctness (right tokens used, no legacy references) but does not verify the rendered visual result looks good. Visual verification requires browser inspection at localhost:3000.
- **E2E test compatibility** — Whether the 22 existing Playwright E2E tests still pass with new class names is deferred to S04.
- **Mobile responsive rendering** — Whether the tokenized components render correctly at mobile viewport (375×812) is not tested here. Deferred to S03.
- **Motion/animation behavior** — No animations were added or modified in this slice. Motion work is S03.

## Notes for Tester

- The grep commands are the primary verification tool. If all greps pass and build succeeds, the structural migration is correct.
- For visual confidence, run `npm run dev` and inspect localhost:3000. The IDE should show a cohesive dark theme with purple/violet accent colors instead of the previous cyan/gray palette.
- Pre-existing wagmi connector warnings in build output (`Attempted import error: 'coinbaseWallet'...`) are unrelated to this slice and can be ignored.
- All files are at `/home/ahwlsqja/Vibe-Loom/` (the Vibe-Loom repo), not in the GSD worktree directory.
