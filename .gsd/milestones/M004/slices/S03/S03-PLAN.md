# S03: Motion + Polish — 애니메이션, 모바일, UX 카피

**Goal:** Add orchestrated entry animations, interaction motion feedback, mobile tab crossfade, accessibility reduced-motion support, and unify all Korean UX copy to English across the Vibe-Loom frontend.
**Demo:** Page load shows staggered panel entry animation; button presses show scale feedback; mobile tab switching has smooth crossfade; all user-facing text is English; `prefers-reduced-motion` disables animations.

## Must-Haves

- `@keyframes fadeInUp` and `.stagger-item` utilities defined in globals.css using existing motion tokens
- `@media (prefers-reduced-motion: reduce)` disabling all animations in globals.css
- `active:scale-[0.96]` on all action buttons (Compile, Deploy, Vibe Score, Call, Send) with disabled-state guard
- Mobile tab switching uses opacity crossfade (not `block`/`hidden` toggle) in IDELayout.tsx
- All ~35 Korean user-facing strings replaced with English equivalents across 5 component files
- `<html lang="ko">` changed to `<html lang="en">` in layout.tsx
- `npm run build` exits 0

## Verification

- `npm run build` exits 0 (in `/home/ahwlsqja/Vibe-Loom`)
- `grep -c "@keyframes\|animation-delay\|active:scale" src/app/globals.css` returns ≥3
- `LC_ALL=C grep -rn $'[\xea-\xed][\x80-\xbf][\x80-\xbf]' src/ --include="*.tsx" | grep -v "node_modules\|\.next\|test\|//"` returns only JSDoc comments (lines with `*` or `//`)
- `grep -q 'lang="en"' src/app/layout.tsx`
- `grep -c "prefers-reduced-motion" src/app/globals.css` returns ≥1
- `grep -c "active:scale" src/app/page.tsx src/components/ide/ContractInteraction.tsx` returns >0 for both files
- `grep -c "opacity-0\|pointer-events-none" src/components/ide/IDELayout.tsx` returns >0

## Tasks

- [x] **T01: Add CSS animation foundation and apply motion to IDE components** `est:25m`
  - Why: The motion tokens (`--ease-snappy`, `--ease-fluid`, `--duration-*`) are defined but unused beyond `panel-transition`. This task builds the full animation system: entry stagger keyframes, button press feedback, mobile tab crossfade, and reduced-motion accessibility.
  - Files: `src/app/globals.css`, `src/components/ide/IDELayout.tsx`, `src/app/page.tsx`, `src/components/ide/ContractInteraction.tsx`
  - Do: (1) Add `@keyframes fadeInUp`, `.stagger-item` with nth-child delays, `.btn-press` utility, `@media (prefers-reduced-motion)` to globals.css. (2) Apply stagger classes to desktop panels and opacity crossfade to mobile tabs in IDELayout. (3) Add `active:not(:disabled):scale-[0.96]` + transition to action buttons in page.tsx and ContractInteraction.tsx. All animation must use existing motion tokens — no hardcoded values.
  - Verify: `npm run build` exits 0 && `grep -c "@keyframes\|animation-delay\|active:scale" src/app/globals.css` returns ≥3 && `grep -c "active:scale\|btn-press" src/app/page.tsx` returns >0
  - Done when: Entry animation keyframes + stagger system defined, button scale feedback applied, mobile crossfade replaces block/hidden, reduced-motion media query present, build passes.

- [x] **T02: Unify all Korean UX copy to English across 5 component files** `est:15m`
  - Why: ~35 Korean strings are mixed with English labels across button states, error messages, and status text. Since Vibe-Loom targets the international Monad developer ecosystem, all user-facing copy must be English. JSDoc/comments in Korean are intentionally left as-is.
  - Files: `src/app/page.tsx`, `src/components/VibeStatus.tsx`, `src/components/WalletConnectModal.tsx`, `src/components/ide/VibeScoreDashboard.tsx`, `src/app/layout.tsx`
  - Do: (1) Replace all Korean user-facing strings with English equivalents per the copy map in S03-RESEARCH.md. (2) Change `lang="ko"` to `lang="en"` in layout.tsx. (3) Do NOT modify Korean in JSDoc comments (VibeStatus.tsx lines 8-10) or inline comments (page.tsx `// 무시`).
  - Verify: `npm run build` exits 0 && `grep -q 'lang="en"' src/app/layout.tsx` && `LC_ALL=C grep -rn $'[\xea-\xed][\x80-\xbf][\x80-\xbf]' src/ --include="*.tsx" | grep -v "node_modules\|\.next\|test\|//" | grep -v "^\s*//" | grep -v "^\s*\*"` returns only comment lines
  - Done when: Zero Korean in rendered JSX text, `lang="en"` on html element, build passes.

## Observability / Diagnostics

- **Animation system inspection:** `grep -c "@keyframes\|animation-delay\|stagger-item\|btn-press\|tab-fade" src/app/globals.css` — confirms all motion utilities are defined (expect ≥5).
- **Reduced-motion guard:** `grep -c "prefers-reduced-motion" src/app/globals.css` — confirms accessibility media query exists (expect ≥1).
- **Button feedback coverage:** `grep -c "btn-press" src/app/page.tsx src/components/ide/ContractInteraction.tsx` — confirms press feedback applied to all action buttons.
- **Mobile tab crossfade:** `grep -c "opacity-0\|pointer-events-none\|tab-fade" src/components/ide/IDELayout.tsx` — confirms opacity-based toggle replaced block/hidden.
- **Failure visibility:** If animations don't play, check browser DevTools → Elements → computed styles on `.stagger-item` elements for `animation` property. Check `@media (prefers-reduced-motion)` state in DevTools → Rendering → Emulate CSS media feature.
- **Korean text audit:** `LC_ALL=C grep -rn $'[\xea-\xed][\x80-\xbf][\x80-\xbf]' src/ --include="*.tsx" | grep -v "node_modules\|\.next\|test\|//"` — any non-comment lines indicate untranslated Korean copy.

## Files Likely Touched

- `src/app/globals.css`
- `src/components/ide/IDELayout.tsx`
- `src/app/page.tsx`
- `src/components/ide/ContractInteraction.tsx`
- `src/components/VibeStatus.tsx`
- `src/components/WalletConnectModal.tsx`
- `src/components/ide/VibeScoreDashboard.tsx`
- `src/app/layout.tsx`
