---
id: S02
parent: M004
milestone: M004
provides:
  - All 10 remaining component files migrated from hardcoded Tailwind gray/amber/cyan to S01 design token vocabulary
  - Zero structural gray-[0-9] references across all migrated files (verified by grep)
  - Gradient text anti-pattern (text-transparent bg-clip-text) eliminated from page.tsx
  - VibeScoreDashboard SVG background ring hex updated from #374151 to #3d3a4e (oklch border-subtle)
  - 76+ design token references added across 10 files (surface, text, border, accent utilities)
  - Semantic status colors preserved: emerald (success), red (error), amber (warning/pending/deploy)
requires:
  - slice: S01
    provides: Design tokens in globals.css @theme block (24 tokens — oklch colors, fonts, spacing, motion), surface hierarchy convention (base→raised→overlay), Monaco theme in src/lib/monaco-theme.ts
affects:
  - S03
  - S04
key_files:
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/ContractInteraction.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/VibeScoreDashboard.tsx
  - /home/ahwlsqja/Vibe-Loom/src/app/page.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/WalletConnectModal.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/VibeStatus.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/TransactionConsole.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/AIDiffViewerInner.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/AIDiffViewer.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/MonacoEditor.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/MonacoEditorInner.tsx
key_decisions:
  - "Accent vs amber semantic distinction: text-accent for read function names and structural highlights, text-amber-300 for write function names and deploy/caution actions"
  - "Compile button unified to bg-accent (was bg-blue-600) for consistent accent action buttons"
  - "Deploy button retains bg-amber-600 — semantic caution color for irreversible deploy actions"
  - "AIDiffViewerInner summary banner bg-amber-900/30 → bg-accent-bg (structural accent, not warning)"
patterns_established:
  - "Surface mapping: bg-gray-800 → bg-surface-raised, bg-gray-900 → bg-surface-base, bg-gray-700 → bg-surface-overlay"
  - "Border mapping: border-gray-700/border-gray-600 → border-border-subtle, focus:border-cyan-500 → focus:border-accent"
  - "Text hierarchy: text-gray-200/300 → text-text-primary, text-gray-400 → text-text-secondary, text-gray-500/600 → text-text-muted"
  - "Accent mapping: text-cyan-* → text-accent, bg-cyan-* → bg-accent, focus:ring-cyan-* → focus:ring-accent"
  - "Placeholder mapping: placeholder-gray-600 → placeholder-text-muted"
  - "SVG hex alignment: #374151 (gray-700) → #3d3a4e (oklch border-subtle)"
  - "Loading placeholder text: text-gray-500 → text-text-muted (consistent across all lazy-loaded wrappers)"
observability_surfaces:
  - "grep -c 'gray-[0-9]' <any of 10 migrated files> returns 0 — confirms migration completeness"
  - "grep -c 'bg-surface-\\|text-text-\\|border-border-\\|text-accent\\|border-accent\\|bg-accent' <key file> returns >0 — confirms token adoption"
  - "grep -rn 'bg-clip-text\\|text-transparent.*gradient' src/app/page.tsx returns 0 — confirms anti-pattern removed"
  - "npm run build exits 0 — confirms no compilation errors from token migration"
drill_down_paths:
  - .gsd/milestones/M004/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M004/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M004/slices/S02/tasks/T03-SUMMARY.md
duration: 27m
verification_result: passed
completed_at: 2026-03-23
---

# S02: Core Components — 에디터 + 사이드바 + 콘솔 리팩토링

**Migrated all 10 remaining component files from hardcoded Tailwind gray/amber/cyan utilities to S01 design token vocabulary, achieving zero structural gray references and eliminating the gradient text anti-pattern. The entire IDE now renders with a visually consistent design system.**

## What Happened

This slice completed the component-level design token migration across the full Vibe-Loom frontend. The work was organized in three tasks by file complexity:

**T01 (12m)** tackled the three largest, most visually impactful files — ContractInteraction.tsx (22 token refs), VibeScoreDashboard.tsx (24 token refs), and page.tsx (9 token refs). These contained ~70% of all legacy color references. The gradient text anti-pattern (`text-transparent bg-clip-text bg-gradient-to-r from-amber-500 to-orange-400`) on the page title was replaced with a clean `text-accent`. The VibeScoreDashboard SVG gauge background ring hex was updated from `#374151` to `#3d3a4e` to align with the oklch border-subtle token. Key semantic decisions were established: accent color for read function names and structural highlights, amber preserved for write function names and deploy/caution actions.

**T02 (10m)** migrated four medium-complexity files — WalletConnectModal.tsx (9 refs), VibeStatus.tsx (3 refs), TransactionConsole.tsx (5 refs), and AIDiffViewerInner.tsx (4 refs). These followed the mapping patterns established in T01 without any new decisions needed. All semantic status colors (emerald/red/amber for success/error/warning indicators) were preserved.

**T03 (5m)** completed three trivial single-line migrations in lazy-loaded wrapper components (AIDiffViewer.tsx, MonacoEditor.tsx, MonacoEditorInner.tsx), each replacing `text-gray-500` with `text-text-muted` in loading placeholders. T03 then ran the comprehensive slice-wide verification confirming all 10 files are clean.

Zero functional changes throughout — only className strings were modified. No component interfaces, props, state, or event handlers were touched.

## Verification

All slice-level verification checks pass:

| Check | Result |
|-------|--------|
| `npm run build` exits 0 | ✅ pass (32.1s, zero errors) |
| `grep -rn "gray-[0-9]"` across all 10 files returns 0 results | ✅ pass |
| Semantic amber usage preserved (write fns, deploy btns, status indicators) | ✅ pass (17 matches across 6 files, all semantic) |
| `grep -rn "bg-clip-text\|text-transparent.*gradient" src/app/page.tsx` returns 0 | ✅ pass |
| `grep -rn "bg-surface-\|text-text-\|border-border-\|text-accent\|border-accent\|bg-accent"` on key files returns >0 | ✅ pass (76+ token refs across 10 files) |

## New Requirements Surfaced

- none

## Deviations

None. All three tasks executed exactly as planned.

## Known Limitations

- **Tailwind v4 silent no-op on misspelled tokens** — If a token class name is misspelled (e.g., `bg-surface-raisd`), Tailwind v4 generates no error; the class silently becomes a no-op. Visual inspection at localhost:3000 is the only way to catch such regressions. No automated visual diff is in place yet.
- **No visual verification performed** — Token migration was verified by grep (structural correctness) and build (compilation). Actual rendered appearance was not verified in a browser during this slice. S03 and S04 will address visual and E2E verification.

## Follow-ups

- S03 should add motion and polish to the now-tokenized components. All files are ready for animation work since className strings are the only thing that changed.
- S04 must verify all 22 E2E tests still pass with the new class names. DOM structure is unchanged, but any selectors based on specific Tailwind class names (e.g., `[class*="gray"]`) would break.

## Files Created/Modified

- `/home/ahwlsqja/Vibe-Loom/src/components/ide/ContractInteraction.tsx` — 22 token refs; all structural gray/cyan replaced; amber preserved for write function semantic distinction
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/VibeScoreDashboard.tsx` — 24 token refs; SVG background ring hex updated #374151 → #3d3a4e
- `/home/ahwlsqja/Vibe-Loom/src/app/page.tsx` — 9 token refs; gradient text anti-pattern eliminated; toolbar/selector/auth area tokenized
- `/home/ahwlsqja/Vibe-Loom/src/components/WalletConnectModal.tsx` — 9 token refs; amber-600 deploy button preserved
- `/home/ahwlsqja/Vibe-Loom/src/components/VibeStatus.tsx` — 3 token refs; semantic amber/emerald status indicators preserved
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/TransactionConsole.tsx` — 5 token refs; semantic status config objects preserved
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/AIDiffViewerInner.tsx` — 4 token refs; text-amber-200 summary label preserved
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/AIDiffViewer.tsx` — text-gray-500 → text-text-muted in loading placeholder
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/MonacoEditor.tsx` — text-gray-500 → text-text-muted in loading placeholder
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/MonacoEditorInner.tsx` — text-gray-500 → text-text-muted in Editor loading prop

## Forward Intelligence

### What the next slice should know
- All 10 component files now exclusively use design token utilities for structural colors. The token vocabulary is: `bg-surface-{base,raised,overlay}`, `text-text-{primary,secondary,muted}`, `border-border-subtle`, `text-accent`, `bg-accent`, `bg-accent-bg`, `placeholder-text-muted`.
- Semantic colors remain as-is: emerald (success), red (error), amber (warning/pending/deploy/write-function). These should NOT be tokenized — they're semantic indicators, not structural palette.
- The gradient text anti-pattern is gone. The page title is now `text-accent` — a single solid color from the design system.

### What's fragile
- **Misspelled token classes are invisible failures** — Tailwind v4 does not error on unknown utility classes. If a token name drifts between globals.css and component usage, the class becomes a no-op and the element reverts to browser defaults. Visual inspection is the only safety net until S04's E2E checks.
- **SVG hex values are manually synchronized** — VibeScoreDashboard's gauge ring uses `#3d3a4e` which must stay in sync with the oklch border-subtle token. There's no automated check for this.

### Authoritative diagnostics
- `grep -c "gray-[0-9]" <file>` — returns 0 for any fully migrated file. If it returns >0, migration was missed or reverted.
- `npm run build` — must exit 0. Pre-existing wagmi connector warnings are noise, not errors.
- Token adoption check: `grep -c "bg-surface-\|text-text-\|text-accent\|border-border-" <file>` — should return >0 for any component with structural UI (trivial wrappers may only have 1).

### What assumptions changed
- **Original assumption: amber was only for warnings** — Actually amber serves triple duty: write function name distinction (text-amber-300), deploy button caution (bg-amber-600), and status indicators (pending/medium). All three are semantic and should be preserved in future work.
