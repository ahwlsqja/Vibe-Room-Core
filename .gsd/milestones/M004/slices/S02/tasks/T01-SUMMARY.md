---
id: T01
parent: S02
milestone: M004
provides:
  - ContractInteraction.tsx fully migrated to design token utilities (22 token refs, 0 legacy gray/cyan)
  - VibeScoreDashboard.tsx fully migrated to design token utilities (24 token refs, 0 legacy gray, SVG hex updated)
  - page.tsx fully migrated — gradient text anti-pattern eliminated, toolbar/selector/auth area tokenized (9 token refs, 0 legacy gray)
key_files:
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/ContractInteraction.tsx
  - /home/ahwlsqja/Vibe-Loom/src/components/ide/VibeScoreDashboard.tsx
  - /home/ahwlsqja/Vibe-Loom/src/app/page.tsx
key_decisions:
  - "read=accent, write=amber semantic distinction preserved in ContractInteraction (text-accent for read function names, text-amber-300 for write function names)"
  - "Contract selector active state uses bg-accent text-surface-base instead of bg-amber-600 — amber reserved for deploy/warning semantics"
  - "Compile button unified to bg-accent (was bg-blue-600) for consistent accent action buttons"
  - "Deploy button keeps bg-amber-600 — semantic caution color for deploy actions"
patterns_established:
  - "Surface mapping: bg-gray-800 → bg-surface-raised, bg-gray-900 → bg-surface-base, bg-gray-700 → bg-surface-overlay"
  - "Border mapping: border-gray-700/border-gray-600 → border-border-subtle, focus:border-cyan-500 → focus:border-accent"
  - "Text mapping: text-gray-200/300 → text-text-primary, text-gray-400 → text-text-secondary, text-gray-500/600 → text-text-muted"
  - "Accent mapping: text-cyan-* → text-accent, bg-cyan-* → bg-accent, focus:ring-cyan-* → focus:ring-accent"
  - "SVG hex alignment: #374151 (gray-700) → #3d3a4e (oklch 0.30 0.014 260 = border-subtle)"
  - "placeholder-gray-600 → placeholder-text-muted for input placeholder text"
observability_surfaces:
  - "grep -c 'gray-[0-9]' <file> returns 0 to verify migration completeness"
  - "grep -c 'bg-surface-\\|text-text-\\|text-accent\\|border-border-' <file> returns >0 to confirm token adoption"
duration: 12m
verification_result: passed
completed_at: 2026-03-23
blocker_discovered: false
---

# T01: Migrate ContractInteraction, VibeScoreDashboard, and page.tsx to design tokens

**Replaced all structural gray/cyan/blue Tailwind classes with design token utilities across 3 core component files, eliminated gradient text anti-pattern on page title, and updated SVG hex color in VibeScoreDashboard gauge.**

## What Happened

Migrated the 3 largest and most visually impactful component files from hardcoded Tailwind gray/amber/cyan utilities to the S01 design token vocabulary. This covers ~70% of all legacy color references in the S02 slice.

**ContractInteraction.tsx** (22 token references added): All `bg-gray-800` → `bg-surface-raised`, `bg-gray-900` → `bg-surface-base`, `border-gray-700`/`border-gray-600` → `border-border-subtle`, `text-gray-*` → `text-text-primary`/`text-text-secondary`/`text-text-muted` context-dependently, `text-cyan-*` → `text-accent`, `bg-cyan-700` → `bg-accent`, `focus:border-cyan-500` → `focus:border-accent`. Preserved `text-amber-300` for write function names (semantic read=accent/write=amber distinction) and all emerald/red status colors.

**VibeScoreDashboard.tsx** (24 token references added): Full loading skeleton and rendered dashboard migrated. SVG background ring hex `#374151` updated to `#3d3a4e` to match oklch 0.30 0.014 260 (border-subtle). All semantic score colors preserved (#34d399 emerald, #fbbf24 amber, #f87171 red).

**page.tsx** (9 token references added): Gradient text anti-pattern (`text-transparent bg-clip-text bg-gradient-to-r from-amber-500 to-orange-400`) replaced with `text-accent`. Contract selector active state changed from `bg-amber-600` to `bg-accent text-surface-base`. Compile button unified from `bg-blue-600` to `bg-accent`. Deploy button kept as `bg-amber-600` (semantic caution). All structural gray in toolbar, auth area, and buttons migrated to surface/border/text tokens.

Zero functional changes — only className strings modified. No component interfaces, props, state, or event handlers touched.

## Verification

All 6 task-level verification checks pass. Build succeeds with zero errors.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm run build` (Vibe-Loom) | 0 | ✅ pass | 33.6s |
| 2 | `grep -c "gray-[0-9]" src/components/ide/ContractInteraction.tsx` | 1 (0 matches) | ✅ pass | <1s |
| 3 | `grep -c "gray-[0-9]" src/components/ide/VibeScoreDashboard.tsx` | 1 (0 matches) | ✅ pass | <1s |
| 4 | `grep -c "gray-[0-9]" src/app/page.tsx` | 1 (0 matches) | ✅ pass | <1s |
| 5 | `grep -c "bg-clip-text" src/app/page.tsx` | 1 (0 matches) | ✅ pass | <1s |
| 6 | `grep -c "bg-surface-\|text-text-\|text-accent\|border-border-" src/components/ide/ContractInteraction.tsx` → 21 | 0 | ✅ pass | <1s |
| 7 | `grep -c "#374151" src/components/ide/VibeScoreDashboard.tsx` | 1 (0 matches) | ✅ pass | <1s |
| 8 | Token usage across all 3 files: 55 total references | 0 | ✅ pass | <1s |

### Slice-level verification (partial — T01 covers 3 of 10 files):

| Check | Status |
|-------|--------|
| `npm run build` exits 0 | ✅ pass |
| gray-[0-9] eliminated in T01's 3 files | ✅ pass (remaining 7 files in T02/T03) |
| Semantic amber preserved | ✅ pass (write fn names, deploy button, status indicators) |
| Gradient anti-pattern removed from page.tsx | ✅ pass |
| Token utilities actually used in T01's 3 files | ✅ pass (55 references) |

## Diagnostics

- Run `grep -c "gray-[0-9]" <file>` on any migrated file to confirm zero legacy references.
- Run `grep -c "bg-surface-\|text-text-\|text-accent\|border-border-" <file>` to confirm token adoption (>0 expected).
- Visual inspection at `localhost:3000` is the ultimate check — misspelled token names produce no build error in Tailwind v4 (they become no-op classes).
- Pre-existing wagmi connector warnings in build output are unrelated to this slice.

## Deviations

None. All steps executed as planned.

## Known Issues

None.

## Files Created/Modified

- `/home/ahwlsqja/Vibe-Loom/src/components/ide/ContractInteraction.tsx` — All structural gray/cyan replaced with design token utilities (22 token refs); amber preserved for write function semantic distinction
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/VibeScoreDashboard.tsx` — All structural gray replaced (24 token refs); SVG background ring hex updated from #374151 to #3d3a4e
- `/home/ahwlsqja/Vibe-Loom/src/app/page.tsx` — Gradient text anti-pattern eliminated; toolbar/selector/auth area migrated to tokens (9 token refs)
- `.gsd/milestones/M004/slices/S02/S02-PLAN.md` — Added Observability / Diagnostics section
- `.gsd/milestones/M004/slices/S02/tasks/T01-PLAN.md` — Added Observability Impact section
