---
id: T01
parent: S03
milestone: M004
provides:
  - CSS animation foundation (fadeInUp keyframes, stagger-item delays, btn-press utility, tab-fade crossfade, prefers-reduced-motion guard)
  - Button press scale(0.96) feedback on all action buttons in page.tsx and ContractInteraction.tsx
  - Mobile tab opacity crossfade replacing block/hidden toggle in IDELayout.tsx
  - Desktop panel stagger entry animation in IDELayout.tsx
key_files:
  - src/app/globals.css
  - src/components/ide/IDELayout.tsx
  - src/app/page.tsx
  - src/components/ide/ContractInteraction.tsx
key_decisions:
  - Used CSS utility class .btn-press with explicit transition-property instead of Tailwind inline active:scale-[0.96] — keeps motion tokens centralized and avoids transition:all
  - Wrapped Panel children in <div className="stagger-item h-full"> since react-resizable-panels Panel does not accept arbitrary className
patterns_established:
  - .btn-press class for all action buttons needing press feedback — apply to future buttons too
  - .tab-fade + opacity-0/pointer-events-none pattern for tab switching (keeps elements in DOM, enables CSS transitions)
  - .stagger-item with nth-child delays for sequential panel entry animations
observability_surfaces:
  - grep -c "btn-press" on component files to verify press feedback coverage
  - DevTools computed styles on .stagger-item for animation property inspection
  - DevTools Rendering → Emulate prefers-reduced-motion to verify accessibility guard
duration: 12m
verification_result: passed
blocker_discovered: false
---

# T01: Add CSS animation foundation and apply motion to IDE components

**Added CSS animation system with fadeInUp stagger entries, btn-press scale(0.96) feedback on all action buttons, mobile tab opacity crossfade, and prefers-reduced-motion accessibility guard using existing motion tokens**

## What Happened

Built the complete CSS animation system for Vibe-Loom using existing motion tokens (`--ease-snappy`, `--ease-fluid`, `--duration-fast/normal/slow`). Five utility classes were added to globals.css:

1. `@keyframes fadeInUp` with `.stagger-item` nth-child delays (0/80/160/240ms) for orchestrated panel entry
2. `.btn-press` with explicit `transition-property` (color, background-color, border-color, transform) and `:active:not(:disabled)` applying `scale(0.96)`
3. `.tab-fade` for smooth opacity transitions on mobile tab switching
4. `@media (prefers-reduced-motion: reduce)` guard disabling both stagger animations and press transforms

Applied these utilities across 3 component files: replaced all `transition-colors` with `btn-press` on 7 buttons in page.tsx (contract selectors, Compile, Deploy, Vibe Score, logout, GitHub login, Retry Deployment) and 2 buttons in ContractInteraction.tsx (Call, Send). Converted mobile tabs in IDELayout.tsx from `block`/`hidden` toggle to `opacity-100`/`opacity-0 pointer-events-none` with `tab-fade`. Added `stagger-item` wrappers inside 3 desktop Panel components.

## Verification

- `npm run build` exits 0 — production build succeeds with no errors (only pre-existing wagmi connector warnings)
- `@keyframes fadeInUp` count: 1 ✅
- `prefers-reduced-motion` count: 1 ✅
- `btn-press` in page.tsx: 7 (≥3 required) ✅
- `btn-press` in ContractInteraction.tsx: 2 (≥2 required) ✅
- `opacity-0|pointer-events-none` in IDELayout.tsx: 3 (≥3 required) ✅
- `tab-fade` in IDELayout.tsx: found ✅
- `stagger-item` in IDELayout.tsx: 3 (≥2 required) ✅
- Zero `transition-colors` remaining in page.tsx or ContractInteraction.tsx ✅

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm run build` | 0 | ✅ pass | 30.6s |
| 2 | `grep -c "@keyframes fadeInUp" src/app/globals.css` | 0 (→1) | ✅ pass | <1s |
| 3 | `grep -c "prefers-reduced-motion" src/app/globals.css` | 0 (→1) | ✅ pass | <1s |
| 4 | `grep -c "btn-press" src/app/page.tsx` | 0 (→7) | ✅ pass | <1s |
| 5 | `grep -c "btn-press" src/components/ide/ContractInteraction.tsx` | 0 (→2) | ✅ pass | <1s |
| 6 | `grep -c "opacity-0\|pointer-events-none" src/components/ide/IDELayout.tsx` | 0 (→3) | ✅ pass | <1s |
| 7 | `grep -q "tab-fade" src/components/ide/IDELayout.tsx` | 0 | ✅ pass | <1s |
| 8 | `grep -c "stagger-item" src/components/ide/IDELayout.tsx` | 0 (→3) | ✅ pass | <1s |

### Slice-Level Checks (partial — T01 is intermediate task)

| # | Command | Exit Code | Verdict | Notes |
|---|---------|-----------|---------|-------|
| 1 | `npm run build` exits 0 | 0 | ✅ pass | — |
| 2 | `grep -c "@keyframes\|animation-delay\|active:scale" globals.css` ≥3 | 0 (→5) | ✅ pass | — |
| 3 | Korean-only JSDoc check | — | ⏳ T02 | Not this task's scope |
| 4 | `grep -q 'lang="en"' layout.tsx` | — | ⏳ T02 | Not this task's scope |
| 5 | `prefers-reduced-motion` ≥1 | 0 (→1) | ✅ pass | — |
| 6 | `grep -c "active:scale"` >0 in both files | 1 (→0) | ⚠️ expected | Task uses `.btn-press` CSS class, not inline `active:scale-[0.96]` — functionally equivalent. Slice plan grep should check `btn-press` or `scale(0.96)` instead |
| 7 | `opacity-0\|pointer-events-none` in IDELayout >0 | 0 (→3) | ✅ pass | — |

## Diagnostics

- **Inspect animation utilities:** `grep -A3 "@keyframes fadeInUp\|\.stagger-item\|\.btn-press\|\.tab-fade\|prefers-reduced-motion" src/app/globals.css`
- **Verify button coverage:** `grep -n "btn-press" src/app/page.tsx src/components/ide/ContractInteraction.tsx` — shows all lines where press feedback is applied
- **Check mobile crossfade:** `grep -n "opacity-0\|tab-fade" src/components/ide/IDELayout.tsx` — shows opacity toggle + transition class on tab panels
- **Test reduced-motion:** In Chrome DevTools → Rendering → Emulate CSS media → `prefers-reduced-motion: reduce` — stagger-item animations should stop, btn-press scale should not apply

## Deviations

- Slice plan verification check #6 (`grep -c "active:scale"`) expects inline Tailwind `active:scale-[0.96]` class in component files. The task plan prescribed a `.btn-press` CSS utility class which applies `scale(0.96)` via `:active:not(:disabled)` pseudo-selector in globals.css. Functionally identical — both produce scale(0.96) on active press with disabled guard — but the grep pattern doesn't match. T02 or slice completion should adjust the slice verification grep to check for `btn-press` or `scale(0.96)`.

## Known Issues

None.

## Files Created/Modified

- `src/app/globals.css` — Added @keyframes fadeInUp, .stagger-item with nth-child delays, .btn-press utility with explicit transition-property, .tab-fade crossfade, @media (prefers-reduced-motion: reduce) guard
- `src/components/ide/IDELayout.tsx` — Replaced block/hidden mobile tab toggle with opacity-0/pointer-events-none + tab-fade; added stagger-item wrappers inside desktop Panel components
- `src/app/page.tsx` — Replaced transition-colors with btn-press on 7 action buttons (contract selectors, Compile, Deploy, Vibe Score, logout, GitHub login, Retry Deployment)
- `src/components/ide/ContractInteraction.tsx` — Replaced transition-colors with btn-press on Call and Send buttons
