# S01: Design Foundation — 디자인 시스템 + 레이아웃 셸 — UAT

**Milestone:** M004
**Written:** 2026-03-23

## UAT Type

- UAT mode: mixed (artifact-driven + live-runtime)
- Why this mode is sufficient: Design tokens are verified via build + grep (artifact-driven), but visual quality (color rendering, font loading, Monaco theme appearance) requires live browser inspection (live-runtime). No E2E tests needed at this stage — S04 handles regression.

## Preconditions

1. `cd /home/ahwlsqja/Vibe-Loom && npm install` has been run (dependencies installed)
2. `npm run dev` is running and accessible at `http://localhost:3000`
3. Browser with DevTools available (Chrome/Edge recommended for oklch support)
4. No `.env.local` changes required for this slice — design tokens are CSS-only

## Smoke Test

Open `http://localhost:3000` in browser. The IDE should render with a **dark cool-blue background** (not the previous pure dark gray). If the background looks distinctly different from before (cooler, slightly blue-tinted), the design tokens are applying. If it looks identical to old `bg-gray-900`, something is wrong.

## Test Cases

### 1. Design Token Resolution

1. Open `http://localhost:3000`
2. Open DevTools → select any element (e.g. the main IDE container)
3. Go to Computed Styles → filter for `--color-surface`
4. **Expected:** `--color-surface-base`, `--color-surface-raised`, `--color-surface-overlay` all show resolved oklch values (e.g. `oklch(0.145 0.014 260)`)
5. Filter for `--color-accent`
6. **Expected:** `--color-accent` shows a teal-ish oklch value at hue ~185
7. Filter for `--font-sans` and `--font-mono`
8. **Expected:** Both variables resolve to font family strings containing "Geist" and "JetBrains Mono" respectively

### 2. Custom Font Loading

1. Open `http://localhost:3000`
2. Open DevTools Console
3. Run: `getComputedStyle(document.body).fontFamily`
4. **Expected:** Output includes "Geist" (not just system sans-serif fallback)
5. Open the Monaco editor area, select some code text
6. Inspect the font-family in Computed Styles
7. **Expected:** Code text uses JetBrains Mono (verify `font-mono` class resolves correctly)

### 3. Monaco vibe-loom-dark Theme

1. Open `http://localhost:3000` and wait for Monaco editor to load
2. Open DevTools Console
3. Run: `monaco.editor.getEditors()[0]?.getOption?.(monaco.editor.EditorOption.theme)` (may need to access via the global `monaco` object)
4. **Expected:** Returns `'vibe-loom-dark'` (not `'vs-dark'`)
5. Visually inspect the editor background color
6. **Expected:** Editor background is a very dark cool blue (#0f1219), distinctly different from vs-dark's default gray (#1e1e1e)
7. Type or view Solidity code with keywords (`contract`, `function`, `uint256`)
8. **Expected:** Syntax highlighting visible — keywords, types, strings, and comments each have distinct colors

### 4. Surface Hierarchy Visual Consistency

1. Open `http://localhost:3000` at desktop width (≥1024px)
2. Identify the three panel areas: Editor (center), Sidebar (right), Console (bottom)
3. **Expected:** Main backgrounds use the darkest surface color (surface-base)
4. **Expected:** Panel headers / tab bars use a slightly lighter shade (surface-raised)
5. Hover over an inactive tab in the mobile tab bar (or resize to mobile width)
6. **Expected:** Hover state shows surface-overlay (slightly lighter than raised)
7. **Expected:** All structural borders are a consistent subtle color (border-subtle), not varying gray-700 shades

### 5. Active Tab Accent Colors

1. Open `http://localhost:3000` (desktop width)
2. If mobile tab bar is visible (resize to <1024px), observe the active tab
3. **Expected:** Active tab has teal-ish accent coloring (text-accent + border-accent + bg-accent-bg)
4. **Expected:** No amber/orange colors remain in tab indicators
5. Switch between Editor, Results, Console tabs
6. **Expected:** Active state accent follows the clicked tab consistently

### 6. Tab Labels Preserved (E2E Compatibility)

1. Open `http://localhost:3000`
2. Resize viewport to mobile width (<1024px) to see tab bar
3. **Expected:** Three tabs visible with labels exactly: "Editor", "Results", "Console"
4. Click each tab
5. **Expected:** Each tab switches content correctly — Editor shows Monaco, Results shows sidebar content, Console shows log output area

### 7. DiffEditor Theme Consistency

1. Trigger an AI analysis error flow that shows the DiffEditor (AIDiffViewerInner)
2. **Expected:** DiffEditor uses the same vibe-loom-dark theme as the main editor
3. **Expected:** Diff additions/removals have visible background highlighting (greenish/reddish tints matching the theme)
4. **Expected:** DiffEditor background matches the main editor background (#0f1219)

### 8. Build Integrity

1. Run `cd /home/ahwlsqja/Vibe-Loom && npm run build`
2. **Expected:** Build completes with exit code 0
3. **Expected:** No Tailwind parse errors in stderr (no "@theme" related errors)
4. **Expected:** Page route `/` still generates static content successfully

## Edge Cases

### Font Loading Failure

1. In DevTools Network tab, block requests to `fonts.googleapis.com`
2. Reload the page
3. **Expected:** UI still renders — text falls back to system sans-serif/monospace. No blank text or layout collapse. The fallback should be visually acceptable (Geist is similar to Inter/system font).

### Monaco Theme Registration Failure

1. Temporarily break `src/lib/monaco-theme.ts` (e.g. rename the export)
2. Run dev server and open page
3. **Expected:** Editor falls back to `vs-dark` theme (lighter background). No crash, no white screen. Console may show an import/reference error.
4. Revert the file to restore normal state.

### Extreme Viewport Width

1. Resize browser to 320px width
2. **Expected:** Mobile tab bar renders without overflow. Tab labels may truncate but are still tappable.
3. Resize to 2560px+ width
4. **Expected:** IDE panels fill space appropriately. No horizontal scroll unless editor content exceeds viewport.

## Failure Signals

- Editor background is `#1e1e1e` (vs-dark default) instead of `#0f1219` → Monaco theme not registering
- Any amber/orange color visible in tab indicators → T03 migration incomplete
- `font-family` on body resolves to only system fonts → font loading or CSS variable bridging broken
- `--color-surface-base` not found in DevTools Computed Styles → @theme block not parsing
- `npm run build` fails with Tailwind parse error → @theme syntax broken
- Tab labels say anything other than "Editor", "Results", "Console" → E2E compatibility regression

## Not Proven By This UAT

- **Component-level design consistency**: Only 4 shell components (IDELayout, EditorPanel, SidebarPanel, ConsolePanel) are migrated. The remaining 9+ components (ContractInteraction, VibeScoreDashboard, AIErrorAnalysis, etc.) still use hardcoded gray/amber — that's S02 scope.
- **Motion/animation**: Motion tokens are defined but not yet applied to any component — that's S03 scope.
- **Mobile layout optimization**: Tab bar works at mobile width but full mobile responsive layout is S03 scope.
- **E2E test regression**: Not running the 22 Playwright E2E tests — that's S04 scope.
- **UX copy consistency**: Korean/English text mixing is not addressed — S03 scope.

## Notes for Tester

- The visual difference between old `bg-gray-900` (#111827) and new `bg-surface-base` (oklch 0.145 0.014 260, approximately #0f1219) is subtle — both are dark. The key difference is the cool blue undertone in the new scheme. Compare side-by-side screenshots if unsure.
- Pre-existing wagmi connector warnings in the console during build are normal and unrelated to this slice.
- Monaco's global `monaco` object may not be directly accessible in console — you may need to access it via `window.monaco` or through the editor instance.
- The DiffEditor test (Test Case 7) requires triggering an AI error analysis flow, which depends on the backend being reachable. If the backend is unavailable, skip this test case.
