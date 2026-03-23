---
estimated_steps: 5
estimated_files: 4
---

# T01: Add CSS animation foundation and apply motion to IDE components

**Slice:** S03 — Motion + Polish — 애니메이션, 모바일, UX 카피
**Milestone:** M004

## Description

Build the full CSS animation system for Vibe-Loom and apply it to IDE components. The motion tokens (`--ease-snappy`, `--ease-fluid`, `--duration-fast/normal/slow`) are already defined in globals.css but only used by `panel-transition` and focus-visible. This task adds: (1) `@keyframes fadeInUp` entry animation with `.stagger-item` nth-child delay utilities, (2) `active:scale-[0.96]` press feedback on all action buttons, (3) opacity-based mobile tab crossfade replacing the current `block`/`hidden` toggle, and (4) `@media (prefers-reduced-motion: reduce)` accessibility guard. **No animation library** — CSS-only using existing tokens.

**Relevant skill:** `make-interfaces-feel-better` — mandates CSS transitions for interactive elements, scale `0.96` exactly, never `transition: all`, split-and-stagger enter animations with ~100ms delay.

## Steps

1. **Add animation utilities to `src/app/globals.css`** (append after the existing `.panel-transition` block):
   - Define `@keyframes fadeInUp` — `from { opacity: 0; transform: translateY(12px); } to { opacity: 1; transform: translateY(0); }` using `var(--ease-snappy)` timing
   - Define `.stagger-item` class — `animation: fadeInUp var(--duration-slow) var(--ease-snappy) both;`
   - Define `.stagger-item:nth-child(1)` through `:nth-child(4)` with delays: `0ms`, `80ms`, `160ms`, `240ms`
   - Define `.btn-press` utility — `transition-property: color, background-color, border-color, transform; transition-timing-function: var(--ease-snappy); transition-duration: var(--duration-fast);`
   - Define `.btn-press:active:not(:disabled)` — `transform: scale(0.96);`
   - Define `@media (prefers-reduced-motion: reduce)` block — disable `.stagger-item` animation (`animation: none !important;`), disable `.btn-press:active:not(:disabled)` transform (`transform: none !important;`)
   - Define `.tab-fade` utility — `transition-property: opacity; transition-timing-function: var(--ease-fluid); transition-duration: var(--duration-normal);`

2. **Apply mobile tab crossfade in `src/components/ide/IDELayout.tsx`**:
   - Replace `${activeTab === "editor" ? "block" : "hidden"}` with `${activeTab === "editor" ? "opacity-100" : "opacity-0 pointer-events-none"} tab-fade`
   - Same for `sidebar` and `console` tab panels
   - This keeps all panels in the DOM (they're already all mounted) but transitions between them with opacity instead of display toggle

3. **Apply stagger entry animation to desktop layout in `src/components/ide/IDELayout.tsx`**:
   - Add `stagger-item` class to the desktop panels wrapping `{editor}`, `{consolePanelContent}`, and `{sidebar}` — apply it to the `<Panel>` components or wrap content in a div if Panel doesn't accept className
   - Note: `react-resizable-panels` `<Panel>` may not accept arbitrary className. Check if it does. If not, wrap content in `<div className="stagger-item h-full">` inside each Panel.

4. **Add btn-press class to action buttons in `src/app/page.tsx`**:
   - Find the Compile button (line ~240, `className="bg-accent hover:bg-accent/80 transition-colors ..."`): replace `transition-colors` with `btn-press` class
   - Find the Deploy button (line ~247, `className="bg-amber-600 hover:bg-amber-500 transition-colors ..."`): same replacement
   - Find the Vibe Score button (line ~254, `className="bg-surface-overlay hover:bg-surface-overlay/80 transition-colors ..."`): same replacement
   - Find the contract selector buttons (line ~221): add `btn-press` to each button's className
   - Find the logout button (line ~266): add `btn-press`
   - Find the "Apply Fix" button (line ~334): add `btn-press`

5. **Add btn-press class to action buttons in `src/components/ide/ContractInteraction.tsx`**:
   - Find the Call button (line ~208, `transition-colors`): replace `transition-colors` with `btn-press`
   - Find the Send button (line ~358, `transition-colors`): replace `transition-colors` with `btn-press`

## Must-Haves

- [ ] `@keyframes fadeInUp` defined in globals.css using motion tokens (not hardcoded values)
- [ ] `.stagger-item` with nth-child delays (80ms intervals, 4 steps)
- [ ] `.btn-press` utility with `transition-property` listing exact properties (never `transition: all`)
- [ ] `.btn-press:active:not(:disabled)` applies `scale(0.96)` — disabled buttons don't scale
- [ ] `@media (prefers-reduced-motion: reduce)` disabling animations
- [ ] Mobile tab panels use `opacity-0 pointer-events-none` instead of `hidden` class
- [ ] `transition-colors` replaced with `btn-press` on action buttons (not added alongside — replaced)

## Verification

- `npm run build` exits 0 (run from `/home/ahwlsqja/Vibe-Loom`)
- `grep -c "@keyframes fadeInUp" src/app/globals.css` returns 1
- `grep -c "prefers-reduced-motion" src/app/globals.css` returns ≥1
- `grep -c "btn-press" src/app/page.tsx` returns ≥3
- `grep -c "btn-press" src/components/ide/ContractInteraction.tsx` returns ≥2
- `grep -c "opacity-0\|pointer-events-none" src/components/ide/IDELayout.tsx` returns ≥3
- `grep -q "tab-fade" src/components/ide/IDELayout.tsx`
- `grep -c "stagger-item" src/components/ide/IDELayout.tsx` returns ≥2

## Inputs

- `src/app/globals.css` — existing 81 lines with motion tokens defined in @theme block (lines 46-52) and `.panel-transition` utility (lines 77-80)
- `src/components/ide/IDELayout.tsx` — 86 lines; mobile tabs use `block`/`hidden` toggle (lines 48-56); desktop has Panel components from react-resizable-panels
- `src/app/page.tsx` — 394 lines; action buttons with `transition-colors` at lines 240, 247, 254, 266, 274, 334
- `src/components/ide/ContractInteraction.tsx` — 448 lines; Call/Send buttons with `transition-colors` at lines 208, 358

## Expected Output

- `src/app/globals.css` — expanded with @keyframes, .stagger-item, .btn-press, .tab-fade, @media (prefers-reduced-motion) utilities
- `src/components/ide/IDELayout.tsx` — mobile tabs use opacity crossfade; desktop panels have stagger-item entry animation
- `src/app/page.tsx` — action buttons use btn-press instead of transition-colors
- `src/components/ide/ContractInteraction.tsx` — Call/Send buttons use btn-press instead of transition-colors
