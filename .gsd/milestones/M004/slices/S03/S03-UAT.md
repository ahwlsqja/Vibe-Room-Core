# S03: Motion + Polish — UAT

**Milestone:** M004
**Written:** 2026-03-23

## UAT Type

- UAT mode: mixed (artifact-driven + live-runtime)
- Why this mode is sufficient: CSS animation presence is verified via grep (artifact-driven), but visual behavior of stagger animations, button press feedback, and mobile tab crossfade requires live browser inspection (live-runtime). Korean→English copy changes are verifiable both ways.

## Preconditions

- Vibe-Loom dev server running: `cd /home/ahwlsqja/Vibe-Loom && npm run dev` (http://localhost:3000)
- Chrome or Chromium browser available
- `npm run build` exits 0 (already verified — production build passes)

## Smoke Test

Open http://localhost:3000 in Chrome. Page should load with visible stagger animation (3 panels fade in sequentially from bottom). All visible text should be in English. Click any "Compile" or "Deploy" button — a subtle scale-down should be visible on press.

## Test Cases

### 1. Desktop Panel Stagger Entry Animation

1. Open http://localhost:3000 in a desktop-width browser (≥1024px)
2. Hard-refresh the page (Ctrl+Shift+R / Cmd+Shift+R)
3. Observe the 3 main panels (Editor, Sidebar, Console)
4. **Expected:** Panels appear sequentially with a slight upward fade-in motion. First panel appears immediately, second ~80ms later, third ~160ms later. Total animation takes ~500ms.
5. Open DevTools → Elements → select any panel wrapper with class `stagger-item`
6. **Expected:** Computed style shows `animation: fadeInUp 0.5s var(--ease-fluid) both` with a delay value

### 2. Button Press Scale Feedback

1. Navigate to http://localhost:3000
2. Click and hold the "Compile" button (do not release immediately)
3. **Expected:** Button visually scales down to ~96% size while pressed
4. Release the button
5. **Expected:** Button returns to normal size smoothly
6. Repeat with "Deploy", "Vibe Score", and a contract selector button
7. **Expected:** All show the same scale-down press feedback
8. Open DevTools → inspect the Compile button → check computed styles
9. **Expected:** `transition-property` includes `transform` (not `all`)

### 3. Button Press Disabled Guard

1. Navigate to http://localhost:3000
2. Click "Compile" to start compilation (button shows "Compiling..." and becomes disabled)
3. While disabled, click the button
4. **Expected:** No scale-down feedback on the disabled button — the `:active:not(:disabled)` guard prevents it

### 4. Mobile Tab Opacity Crossfade

1. Open http://localhost:3000 in a mobile viewport (375×812 or DevTools device toolbar → iPhone 14)
2. Observe the tab bar (Editor, Results, Console tabs)
3. Click "Results" tab
4. **Expected:** Editor panel fades out (opacity transition), Results panel fades in. No instant block/hidden snap — smooth crossfade visible
5. Click "Console" tab
6. **Expected:** Same smooth crossfade from Results to Console
7. Open DevTools → select a hidden tab panel
8. **Expected:** Has classes `opacity-0 pointer-events-none` and class `tab-fade`. Computed opacity is 0, not `display: none`

### 5. Reduced-Motion Accessibility

1. Open http://localhost:3000
2. Open DevTools → Rendering tab → "Emulate CSS media feature prefers-reduced-motion" → select "reduce"
3. Hard-refresh the page
4. **Expected:** No stagger animation — panels appear instantly without fade-in-up motion
5. Click and hold any action button
6. **Expected:** No scale-down transform on press
7. Switch to mobile viewport and test tab switching
8. **Expected:** Tabs still switch content, but without crossfade animation (opacity change may be instant)

### 6. English UX Copy — page.tsx

1. Navigate to http://localhost:3000
2. Check the contract selector dropdown descriptions
3. **Expected:** All descriptions in English ("Deploy error test contract", "Fixed version contract", "Pectra opcode test contract", "Parallel execution test contract")
4. Click "Compile" → check loading state
5. **Expected:** Button text shows "Compiling..." (not "컴파일 중...")
6. If compilation succeeds, click "Deploy" → check loading state
7. **Expected:** Button text shows "Deploying..." (not "배포 중...")

### 7. English UX Copy — WalletConnectModal

1. Trigger the WalletConnect modal (requires >3 deploys or mock the condition)
2. **Expected:** Modal header shows "Wallet Deploy" (not "지갑 배포")
3. **Expected:** Deploy quota message in English ("You have exceeded your free deployment quota")
4. **Expected:** Transaction status states show "Compiling...", "Sending transaction...", "Confirming...", "Deploy complete!" (all English)

### 8. English UX Copy — VibeScoreDashboard

1. Click "Vibe Score" button to trigger analysis
2. **Expected:** Loading state shows "Analyzing..." (not "분석 중...")
3. When results appear, check score status messages
4. **Expected:** Status messages in English ("Optimized for parallel execution", "Some optimization recommended", or "Performance risk factors detected")
5. **Expected:** Suggestions section header shows "Suggestions" (not "제안")

### 9. HTML Lang Attribute

1. Open http://localhost:3000
2. Open DevTools → Elements → inspect the `<html>` tag
3. **Expected:** `<html lang="en">` (not `lang="ko"`)

### 10. Korean Text Audit — No Rendered Korean

1. View all visible text on the page (desktop and mobile viewports)
2. **Expected:** Zero Korean characters visible in any rendered UI text
3. Note: Korean in JSDoc/source comments (not rendered) is intentionally preserved

## Edge Cases

### Rapid Tab Switching on Mobile

1. Open mobile viewport (375×812)
2. Rapidly click through tabs: Editor → Results → Console → Editor in quick succession (<500ms between clicks)
3. **Expected:** Tab content switches correctly without glitching. The last-clicked tab should be fully visible (opacity-1) and all others should be opacity-0.

### Animation During Page Navigation

1. If the app has any client-side navigation, navigate away and back
2. **Expected:** Stagger animations replay on page re-entry (since they're CSS-based, not JS state-dependent)

### Button Feedback on Touch Devices

1. Open mobile viewport or use a touch device
2. Tap and hold an action button
3. **Expected:** Scale-down feedback should work on touch (:active pseudo-class fires on touch devices)

## Failure Signals

- Panels appear instantly with no fade-in animation on desktop → stagger-item CSS not applied or animation overridden
- Button click has no visual scale feedback → btn-press class missing or transition-property not including transform
- Mobile tab switch is instant (block/hidden snap) → tab-fade crossfade not applied, opacity-0/pointer-events-none pattern missing
- Korean text visible anywhere in the UI → incomplete string replacement in T02
- `<html lang="ko">` still present → layout.tsx change missed
- `prefers-reduced-motion: reduce` still shows animations → media query guard not working

## Not Proven By This UAT

- E2E test compatibility with new English text and motion classes (deferred to S04)
- Visual pixel-perfect comparison with design mockup (no mockup exists — design was code-driven)
- Animation performance profiling (frame rate, jank measurement)
- Playwright `emulateMedia('prefers-reduced-motion')` automated test

## Notes for Tester

- The stagger animation is subtle and fast (~500ms total). Watch carefully on first page load or use DevTools → Animation tab to slow it down (10% speed).
- The btn-press scale(0.96) is intentionally subtle — a 4% scale change. Look carefully or compare pressed vs unpressed states.
- Pre-existing wagmi connector warnings appear in the build output — these are expected and unrelated to S03 changes.
- The VibeStatus.tsx file has Korean JSDoc comments on lines 8-10 — this is intentional and should NOT be flagged as a bug.
