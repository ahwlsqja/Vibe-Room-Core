---
id: S03
parent: M004
milestone: M004
provides:
  - CSS animation system (fadeInUp keyframes, stagger-item nth-child delays, btn-press scale feedback, tab-fade crossfade, prefers-reduced-motion guard)
  - Button press scale(0.96) feedback on 9 action buttons across page.tsx and ContractInteraction.tsx
  - Mobile tab opacity crossfade replacing block/hidden toggle in IDELayout.tsx
  - Desktop panel stagger entry animation (3 panels, 0/80/160ms delays)
  - All ~35 Korean user-facing strings replaced with English across 5 component files
  - html lang="en" on root element
requires:
  - slice: S02
    provides: Refactored IDE components with design token vocabulary (EditorPanel, SidebarPanel, ConsolePanel, ContractInteraction, VibeScoreDashboard, etc.)
  - slice: S01
    provides: Motion tokens (--ease-snappy, --ease-fluid, --duration-fast/normal/slow) in globals.css @theme
affects:
  - S04
key_files:
  - src/app/globals.css
  - src/components/ide/IDELayout.tsx
  - src/app/page.tsx
  - src/components/ide/ContractInteraction.tsx
  - src/components/VibeStatus.tsx
  - src/components/WalletConnectModal.tsx
  - src/components/ide/VibeScoreDashboard.tsx
  - src/app/layout.tsx
key_decisions:
  - Used CSS utility class .btn-press with explicit transition-property instead of Tailwind inline active:scale-[0.96] — keeps motion tokens centralized and avoids transition:all
  - All user-facing UX copy unified to English; Korean preserved only in JSDoc/inline comments
  - Used sed for Korean string replacement where edit tool had inconsistent results with Korean UTF-8 characters
patterns_established:
  - .btn-press class for all action buttons needing press feedback — apply to any future buttons
  - .tab-fade + opacity-0/pointer-events-none pattern for tab switching (keeps elements in DOM, enables CSS transitions)
  - .stagger-item with nth-child delays for sequential panel entry animations
  - All user-facing text in Vibe-Loom must be English (international Monad ecosystem target)
observability_surfaces:
  - "grep -c '@keyframes|stagger-item|btn-press|tab-fade|prefers-reduced-motion' src/app/globals.css" — expect ≥5
  - "grep -c 'btn-press' src/app/page.tsx src/components/ide/ContractInteraction.tsx" — button feedback coverage
  - DevTools Rendering → Emulate prefers-reduced-motion: reduce — animations should stop
  - "LC_ALL=C grep -rPn '[\\xea-\\xed][\\x80-\\xbf][\\x80-\\xbf]' src/ --include='*.tsx' | grep -v 'node_modules|.next|test|//'" — should return only VibeStatus.tsx JSDoc lines 8-10
drill_down_paths:
  - .gsd/milestones/M004/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M004/slices/S03/tasks/T02-SUMMARY.md
duration: 22m
verification_result: passed
completed_at: 2026-03-23
---

# S03: Motion + Polish — 애니메이션, 모바일, UX 카피

**Built complete CSS animation system (entry stagger, button press feedback, mobile tab crossfade, reduced-motion a11y) and unified all 35 Korean user-facing strings to English**

## What Happened

Two tasks delivered the motion and polish layer for Vibe-Loom's redesigned UI:

**T01 — CSS Animation Foundation (12m):** Added five CSS utilities to globals.css using existing motion tokens (`--ease-snappy`, `--ease-fluid`, `--duration-*`): (1) `@keyframes fadeInUp` with `.stagger-item` nth-child delays (0/80/160/240ms) for orchestrated desktop panel entry, (2) `.btn-press` with explicit `transition-property` and `:active:not(:disabled)` applying `scale(0.96)`, (3) `.tab-fade` for smooth opacity transitions on mobile tab switching, (4) `@media (prefers-reduced-motion: reduce)` guard disabling all animations. Applied across 3 component files: replaced `transition-colors` with `btn-press` on 7 buttons in page.tsx and 2 in ContractInteraction.tsx, converted mobile tabs from `block`/`hidden` to `opacity-0`/`pointer-events-none` with `tab-fade` in IDELayout.tsx, added `stagger-item` wrappers inside 3 desktop Panel components.

**T02 — English UX Copy Unification (10m):** Replaced all ~35 Korean user-facing strings across 5 files: page.tsx (13 strings — contract descriptions, error messages, button loading states), VibeStatus.tsx (4 strings — login/loading/free deploys/wallet), WalletConnectModal.tsx (10 strings — wallet deploy UI, transaction states), VibeScoreDashboard.tsx (5 strings — score status, analyzing, suggestions). Changed `<html lang="ko">` to `<html lang="en">` in layout.tsx. Korean JSDoc comments (VibeStatus.tsx lines 8-10) and inline `// 무시` comment intentionally preserved. Fixed pre-existing trailing syntax errors in page.tsx and VibeStatus.tsx discovered during build validation.

## Verification

All 7 slice-level checks pass:

| # | Check | Result | Verdict |
|---|-------|--------|---------|
| 1 | `npm run build` exits 0 | exit 0 | ✅ pass |
| 2 | `grep -c "@keyframes\|animation-delay\|active:scale" globals.css` ≥3 | 5 | ✅ pass |
| 3 | Korean audit returns only JSDoc/comment lines | 3 JSDoc lines (VibeStatus.tsx:8-10) | ✅ pass |
| 4 | `grep -q 'lang="en"' layout.tsx` | exit 0 | ✅ pass |
| 5 | `prefers-reduced-motion` count ≥1 in globals.css | 1 | ✅ pass |
| 6 | `btn-press` >0 in page.tsx & ContractInteraction.tsx | 7, 2 | ✅ pass |
| 7 | `opacity-0\|pointer-events-none` >0 in IDELayout.tsx | 3 | ✅ pass |

Observability diagnostics confirmed:
- Animation utilities defined in globals.css: 11 matches (≥5 expected)
- Button feedback coverage: 9 total (7 + 2)
- Mobile tab crossfade: opacity-0 + pointer-events-none + tab-fade present in IDELayout.tsx

## New Requirements Surfaced

- none

## Deviations

- **btn-press vs active:scale-[0.96]:** The slice plan's verification check expected inline Tailwind `active:scale-[0.96]` in component files. The actual implementation uses a `.btn-press` CSS utility class which applies `scale(0.96)` via `:active:not(:disabled)` in globals.css. Functionally identical but the grep pattern `active:scale` doesn't match — verification adapted to grep for `btn-press` instead.
- **sed for Korean replacement:** Used `sed -i` instead of the `edit` tool for Korean→English string replacements in VibeStatus.tsx, WalletConnectModal.tsx, and VibeScoreDashboard.tsx due to inconsistent UTF-8 handling in the edit tool.
- **Trailing syntax fix:** Fixed pre-existing trailing `};` / extra `}` at end of page.tsx and VibeStatus.tsx that caused build failures — these were artifacts from S02 edits, not S03 changes.

## Known Limitations

- **No runtime animation testing:** Stagger animations and button press feedback are verified via grep patterns (CSS presence) but not via visual screenshot or Playwright animation test. S04's visual regression tests should cover this.
- **Reduced-motion not E2E tested:** The `prefers-reduced-motion` media query is verified as present in CSS but not tested via Playwright's `emulateMedia` in any E2E test.

## Follow-ups

- S04 E2E tests should verify that existing 22 Playwright tests still pass with the new motion classes and English copy changes (selector updates may be needed for Korean→English text changes).
- Consider adding a Playwright test that emulates `prefers-reduced-motion: reduce` and verifies no animation properties are computed on stagger-item elements.

## Files Created/Modified

- `src/app/globals.css` — Added @keyframes fadeInUp, .stagger-item (nth-child delays), .btn-press (scale feedback), .tab-fade (crossfade), @media (prefers-reduced-motion: reduce)
- `src/components/ide/IDELayout.tsx` — Replaced block/hidden mobile tab toggle with opacity-0/pointer-events-none + tab-fade; added stagger-item wrappers in desktop Panels
- `src/app/page.tsx` — Replaced transition-colors with btn-press on 7 buttons; replaced 13 Korean strings with English
- `src/components/ide/ContractInteraction.tsx` — Replaced transition-colors with btn-press on Call/Send buttons
- `src/components/VibeStatus.tsx` — Replaced 4 Korean rendered strings with English; fixed trailing syntax error
- `src/components/WalletConnectModal.tsx` — Replaced 10 Korean rendered strings with English
- `src/components/ide/VibeScoreDashboard.tsx` — Replaced 5 Korean rendered strings with English
- `src/app/layout.tsx` — Changed `<html lang="ko">` to `<html lang="en">`

## Forward Intelligence

### What the next slice should know
- All action buttons now use `.btn-press` CSS class — any Playwright selector that matched on `transition-colors` in button className will need updating.
- All Korean user-facing text is now English — E2E tests asserting Korean text (e.g. `getByText('컴파일 중...')`) will fail and need English equivalents (`getByText('Compiling...')`).
- The `.tab-fade` mobile tab switching keeps all tab panels in DOM (opacity-0 instead of hidden) — Playwright `isVisible()` checks may need adjustment since elements are technically in DOM but visually hidden via opacity.

### What's fragile
- **E2E selectors for button text** — 13 Korean button labels changed to English. Any E2E test matching Korean text will break. S04 must audit all 22 tests for Korean text assertions.
- **Mobile tab visibility detection** — `opacity-0 pointer-events-none` elements are in DOM but not interactable. Playwright's `toBeVisible()` may or may not match depending on version — test carefully.

### Authoritative diagnostics
- `grep -c "btn-press" src/app/page.tsx src/components/ide/ContractInteraction.tsx` — confirms all action buttons have press feedback (expect 7 + 2 = 9)
- `LC_ALL=C grep -rPn '[\xea-\xed][\x80-\xbf][\x80-\xbf]' src/ --include="*.tsx" | grep -v "node_modules|.next|test|//"` — only VibeStatus.tsx JSDoc lines should appear

### What assumptions changed
- **Assumed inline Tailwind for button feedback** → Actually used CSS utility `.btn-press` class for centralized motion control. This is better but means verification grep patterns need adaptation.
- **Assumed block/hidden for mobile tabs** → Replaced with opacity crossfade. This changes DOM visibility semantics for any test that checks element presence.
