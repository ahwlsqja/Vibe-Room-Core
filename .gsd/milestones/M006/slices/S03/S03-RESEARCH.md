# S03: Vibe-Loom — 매트릭스 히트맵 + Suggestion 카드 UI — Research

**Date:** 2026-03-24
**Depth:** Light — straightforward React UI addition using established design tokens and patterns already in the codebase.

## Summary

S03 adds two visual sections to the existing `VibeScoreDashboard` component: (1) a function×variable conflict matrix heatmap, and (2) structured suggestion cards that replace/augment the current plain-text suggestion list when `conflictAnalysis` data is present. The work spans 3 files: type extension in `api-client.ts`, props/rendering in `VibeScoreDashboard.tsx`, and prop-passing in `page.tsx`. No new dependencies needed — the heatmap is a small HTML table (typically 3-5 rows × 2-3 cols) with CSS background colors using the existing oklch design tokens.

The backend (S02) already returns `conflictAnalysis?: { conflicts: DecodedConflict[], matrix: ConflictMatrix }` in the API response. The frontend just needs to type it, receive it, and render it. All new UI is conditional on `conflictAnalysis` being present — existing functionality (gauge, stats, generic suggestions) must remain untouched when the field is absent.

## Recommendation

Pure CSS/HTML table heatmap with Tailwind classes. No charting library (d3, recharts, nivo). The matrix dimensions are tiny (≤10 functions × ≤5 variables for realistic contracts), making a charting library pure overhead. Use oklch color interpolation for heatmap intensity — map cell values 0→surface-base, 1+→amber-500/red-400 scale. Structured suggestion cards replace the simple string list when `conflictAnalysis.conflicts` exists, using the existing amber border-left card pattern but with added variable name, function list, and conflict type badges.

## Implementation Landscape

### Key Files

- `Vibe-Loom/src/lib/api-client.ts` — Add `DecodedConflict`, `ConflictMatrix`, `ConflictAnalysis` interfaces mirroring the backend DTO. Add `conflictAnalysis?: ConflictAnalysis` to `VibeScoreResult`.
- `Vibe-Loom/src/components/ide/VibeScoreDashboard.tsx` — Add `conflictAnalysis` prop. Render two new sections: (4) Matrix heatmap table below stats grid, (5) Structured suggestion cards replacing plain `suggestions` when conflict data exists.
- `Vibe-Loom/src/app/page.tsx` — Pass `vibeScore?.conflictAnalysis` as a new prop to `VibeScoreDashboard`. Currently passes `suggestions`, `conflicts`, `reExecutions`, `gasEfficiency`, `engineBased` — just add one more prop.
- `Vibe-Loom/src/__tests__/VibeScoreDashboard.test.tsx` — Add tests for: heatmap renders when conflictAnalysis present, heatmap absent when conflictAnalysis undefined, structured suggestion cards render variable names and function names, existing tests remain green.

### Existing Patterns to Follow

- **Props interface** — `VibeScoreDashboardProps` at top of file, all new fields optional (backward compat).
- **Conditional rendering** — Same pattern as `{suggestions.length > 0 && (...)}` for the existing suggestion section. Use `{conflictAnalysis && (...)}`.
- **Design tokens** — `bg-surface-base`, `bg-surface-raised`, `border-border-subtle`, `text-text-primary`, `text-text-secondary`, `text-text-muted`, `text-amber-400`, `text-accent`. All already used in the component.
- **Card pattern** — Existing suggestion cards: `border-l-2 border-amber-500` with `bg-surface-base rounded-lg p-2.5`. Extend this for structured cards.
- **Stagger animation** — CSS class `stagger-item` defined in globals.css for fade-in-up animation.
- **Test pattern** — `render(<VibeScoreDashboard ... />)` + `screen.getByText()` / `screen.queryByText()`.

### Build Order

1. **Types first** — Add interfaces to `api-client.ts`. Zero risk, unblocks everything.
2. **Component rendering** — Add heatmap + structured suggestion cards to `VibeScoreDashboard.tsx`. This is the bulk of the work.
3. **Wiring** — Pass prop in `page.tsx`. One-line change.
4. **Tests** — Add 4-6 tests covering new rendering paths + backward compat confirmation.

### Verification Approach

1. `cd /home/ahwlsqja/Vibe-Loom && npx jest src/__tests__/VibeScoreDashboard.test.tsx --verbose` — All existing 10 tests pass + new tests pass.
2. `cd /home/ahwlsqja/Vibe-Loom && npx jest src/__tests__/api-client.test.ts --verbose` — Existing 10 tests still pass (type-only changes shouldn't break anything, but verify).
3. `cd /home/ahwlsqja/Vibe-Loom && npx tsc --noEmit` — Type-check passes with new interfaces.
4. Visual: heatmap renders a table with colored cells when mock `conflictAnalysis` data is provided; section hidden when absent.

### API Schema (from S02 Summary — authoritative)

```typescript
interface DecodedConflict {
  variableName: string;   // e.g. "balances"
  variableType: string;   // e.g. "mapping(address => uint256)"
  slot: string;           // e.g. "0x3"
  functions: string[];    // e.g. ["transfer", "approve"]
  conflictType: string;   // "write-write" | "read-write"
  suggestion: string;     // actionable English text
}

interface ConflictMatrix {
  rows: string[];     // function names
  cols: string[];     // variable names
  cells: number[][];  // cells[i][j] = conflict count between rows[i] and cols[j]
}

interface ConflictAnalysis {
  conflicts: DecodedConflict[];
  matrix: ConflictMatrix;
}
```

`conflictAnalysis` is `undefined` (field absent from JSON) when there are no actionable conflicts.

## Constraints

- Must use M004 design tokens (oklch, 3-tier surface). No hardcoded hex colors outside the token system.
- D019: All user-facing text in English.
- No new npm dependencies — pure CSS heatmap.
- `conflictAnalysis` prop must be optional. Existing tests must pass unchanged.
- Heatmap color scale should be perceptually meaningful: 0 conflicts = neutral (surface-base), 1+ = warm scale (amber→red based on count).

## Common Pitfalls

- **Heatmap cell color for zero** — Zero-conflict cells should be visually distinct from "no data". Use `bg-surface-base` (same as current stat boxes) for zero, so it blends with the component background. Non-zero cells use amber/red tones.
- **Empty matrix** — If `conflictAnalysis` exists but `matrix.rows` or `matrix.cols` is empty, don't render the heatmap table (guard with length check). This shouldn't happen based on S02 logic but defensive rendering is free.
- **Long variable names** — Mapping type signatures like `mapping(address => uint256)` can be long. Use `truncate` or `text-xs` with `max-w-` constraints on column headers. Tooltip on hover is nice-to-have.
- **Structured vs plain suggestions** — When `conflictAnalysis.conflicts` exists, render structured cards (with variable name, functions, conflict type). When only plain `suggestions: string[]` exists (no conflict analysis), render existing plain cards. Both can coexist — structured cards for decoded conflicts, plain cards for any remaining generic suggestions.
