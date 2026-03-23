# S03: Motion + Polish — 애니메이션, 모바일, UX 카피 — Research

**Date:** 2026-03-23
**Depth:** Targeted

## Summary

This slice adds three categories of polish to the already-tokenized IDE: (1) orchestrated page-load entry animations with staggered reveals, (2) interaction motion feedback (button press, tab transitions, new-item animations), and (3) UX copy unification to resolve pervasive Korean/English mixing. No animation library is installed and none should be added — the existing motion tokens (`--ease-snappy`, `--ease-fluid`, `--duration-fast/normal/slow`) in globals.css combined with CSS `@keyframes` and transitions provide everything needed. The `make-interfaces-feel-better` skill mandates CSS transitions for interactive elements (interruptible) and reserves keyframes for one-shot entry sequences.

The UX copy audit found ~35 Korean strings across 5 files mixed with English labels. The pattern is consistent: default button labels are English ("Compile", "Deploy"), but loading states and status messages switch to Korean ("컴파일 중...", "배포 완료"). Since Vibe-Loom targets the international Monad developer ecosystem, all user-facing copy should be unified to English. The `html lang` attribute should also change from `ko` to `en`.

## Recommendation

**CSS-only animation approach (no new dependencies).** Per the `make-interfaces-feel-better` skill: "Always prefer CSS transitions for interactive elements. Reserve keyframes for one-shot sequences." The codebase already has motion tokens defined but unused. Implementation should:

1. Define `@keyframes fadeInUp` and `.stagger-item` utility classes in globals.css
2. Apply staggered entry animation to the IDE shell panels on page load
3. Add `active:scale-[0.96]` with `transition-transform` to all action buttons (skill mandates exactly `0.96`)
4. Add mobile tab crossfade transition (CSS transition on opacity, not block/hidden toggle)
5. Unify all Korean UX copy to English in a single sweep
6. Add `@media (prefers-reduced-motion: reduce)` to disable animations for accessibility

## Implementation Landscape

### Key Files

- `src/app/globals.css` (81 lines) — Add @keyframes, stagger utilities, reduced-motion media query. The motion tokens (`--ease-snappy`, `--ease-fluid`, `--duration-*`) are already defined but only used by `panel-transition` and `focus-visible`
- `src/components/ide/IDELayout.tsx` (86 lines) — Mobile tab switching currently uses `block`/`hidden` toggle with no transition. Needs opacity-based crossfade. Desktop panels need entry animation classes
- `src/app/page.tsx` (394 lines) — Largest UX copy surface: 13 Korean strings (button loading states, error messages, CONTRACT_OPTIONS.desc, section headings). Also the toolbar where `active:scale-[0.96]` applies to Compile/Deploy/Vibe Score buttons
- `src/app/layout.tsx` — Change `lang="ko"` → `lang="en"` on the `<html>` tag
- `src/components/VibeStatus.tsx` (87 lines) — 5 Korean strings: "로그인 필요", "로딩 중...", "무료 배포", "지갑 연결 필요"
- `src/components/WalletConnectModal.tsx` (201 lines) — 8 Korean strings: modal header, description, wallet connection text, deploy button loading states, success message
- `src/components/ide/VibeScoreDashboard.tsx` (205 lines) — 4 Korean strings: score labels ("병렬 실행에 적합", "일부 최적화 권장", "성능 저하 위험 요소 존재"), loading text, suggestion heading. Score gauge SVG already has `transition-all duration-700 ease-out` — just needs initial animation
- `src/components/ide/TransactionConsole.tsx` (103 lines) — New transaction entries should animate in. Currently static render
- `src/components/ide/ContractInteraction.tsx` (448 lines) — No Korean strings. Needs `active:scale-[0.96]` on Call/Send buttons

### Complete Korean→English Copy Map

**page.tsx:**
| Line | Korean | English |
|------|--------|---------|
| 29-32 | `desc: "배포 에러 테스트"` etc. | `desc: "Deploy error test"`, `"Fixed version"`, `"Pectra opcode test"`, `"Parallel execution test"` |
| 126 | `"배포할 컨트랙트 소스가 없습니다."` | `"No contract source to deploy."` |
| 160 | `"배포 중 오류가 발생했습니다."` | `"An error occurred during deployment."` |
| 242 | `"컴파일 중..."` | `"Compiling..."` |
| 249 | `"배포 중..."` | `"Deploying..."` |
| 256 | `"분석 중..."` | `"Analyzing..."` |
| 268 | `로그아웃` | `Logout` |
| 292 | `배포 완료` | `Deploy Complete` |
| 317 | `AI가 수정 중...` | `AI fixing...` |
| 323 | `AI 수정 제안` | `AI Fix Suggestion` |

**VibeStatus.tsx:**
| Line | Korean | English |
|------|--------|---------|
| 40 | `로그인 필요` | `Login required` |
| 49 | `로딩 중...` | `Loading...` |
| 68 | `무료 배포` | `free deploys` |
| 73 | `지갑 연결 필요` | `Wallet required` |

**WalletConnectModal.tsx:**
| Line | Korean | English |
|------|--------|---------|
| 114 | `🔗 지갑 연결 배포` | `🔗 Wallet Deploy` |
| 126 | `무료 배포 횟수를 초과했습니다. 지갑을 연결하여 직접 배포하세요.` | `Free deploy quota exceeded. Connect your wallet to deploy directly.` |
| 133 | `지갑을 연결하세요:` | `Connect a wallet:` |
| 152 | `연결된 지갑` | `Connected wallet` |
| 161 | `연결 해제` | `Disconnect` |
| 172-176 | `컴파일 중...`, `트랜잭션 전송 중...`, `트랜잭션 확인 중...` | `Compiling...`, `Sending transaction...`, `Confirming transaction...` |
| 183 | `배포 완료!` | `Deploy complete!` |

**VibeScoreDashboard.tsx:**
| Line | Korean | English |
|------|--------|---------|
| 50 | `Monad 병렬 실행에 적합` | `Optimized for Monad parallel execution` |
| 52 | `일부 최적화 권장` | `Some optimization recommended` |
| 53 | `성능 저하 위험 요소 존재` | `Performance risk factors detected` |
| 94 | `분석 중...` | `Analyzing...` |
| 187 | `개선 제안` | `Suggestions` |

### Build Order

1. **globals.css animation foundation** — Define @keyframes, `.stagger-item` nth-child delays, `active:scale-[0.96]` pattern, `@media (prefers-reduced-motion)`. This unblocks all component-level motion work.
2. **Component motion application** — Apply entry animation classes to IDELayout panels, add mobile tab crossfade, add `active:scale-[0.96]` to all action buttons, add entry animation for TransactionConsole items.
3. **UX copy unification** — Pure string replacement sweep across 5 files + layout.tsx lang attribute. Completely independent of motion work — can run in parallel with task 2.

### Verification Approach

1. **Build check**: `npm run build` exits 0 — no compilation errors
2. **Animation presence**: `grep -c "@keyframes\|animation-delay\|active:scale" src/app/globals.css` returns >0
3. **Korean text eliminated**: `LC_ALL=C grep -rn $'[\xea-\xed][\x80-\xbf][\x80-\xbf]' src/ --include="*.tsx" | grep -v "node_modules\|\.next\|test\|//"` returns only JSDoc comments (lines starting with `*` or `//`), zero user-facing Korean text
4. **Layout lang**: `grep 'lang="en"' src/app/layout.tsx` returns a match
5. **Reduced motion**: `grep -c "prefers-reduced-motion" src/app/globals.css` returns >0
6. **Scale-on-press**: `grep -c "active:scale" src/app/page.tsx src/components/ide/ContractInteraction.tsx` returns >0 for both files
7. **Mobile tab transition**: `grep -c "transition" src/components/ide/IDELayout.tsx` increases from current count (4)
8. **Visual verification**: localhost:3000 page load shows staggered panel entry, button presses show scale feedback, mobile tab switching has smooth crossfade

## Constraints

- **No animation library** — framer-motion / motion / react-spring are NOT in package.json. All animation must be CSS-only (`@keyframes` + `transition`). Per the skill: "If no motion dependency, use CSS cross-fade pattern — don't add a dependency just for icon transitions."
- **Motion tokens already defined** — `--ease-snappy: cubic-bezier(0.2, 0, 0, 1)`, `--ease-fluid: cubic-bezier(0.4, 0, 0.2, 1)`, `--duration-fast: 100ms`, `--duration-normal: 200ms`, `--duration-slow: 350ms`. All animation should reference these tokens, not hardcoded values.
- **Tailwind v4 CSS-first** — No tailwind.config.ts. All customization in `@theme` block of globals.css. Custom utilities go below the `@theme` block as regular CSS.
- **`panel-transition` pattern** — Already establishes the convention for component-level transitions. New animation utilities should follow the same pattern (utility class in globals.css, applied via className).
- **Existing `transition-colors`** — Many buttons already have `transition-colors`. Adding `active:scale-[0.96]` requires changing to `transition-[colors,transform]` or using separate Tailwind utilities (`transition-colors` + `transition-transform` won't compose — need explicit `transition-property`).
- **Mobile tab show/hide** — Currently `block`/`hidden` Tailwind classes. Transitioning to opacity requires keeping elements in the DOM (not `display: none`). Use `opacity-0 pointer-events-none` instead of `hidden` for inactive tabs.

## Common Pitfalls

- **`transition: all` anti-pattern** — The skill explicitly says: "NEVER use `transition: all`. Always specify exact properties." When adding `active:scale-[0.96]`, use `transition-[transform]` or the more specific `transition-transform`, not `transition-all`. Buttons already have `transition-colors`, so the combined property list must be explicit: `transition-[color,background-color,transform]` or use Tailwind's `transition-[colors,transform]`.
- **Entry animation on mobile tab switch** — Stagger animations are for page load only. Tab switching should use CSS transitions (interruptible), not keyframe animations (which would restart on rapid switching). Use `AnimatePresence initial={false}` pattern equivalent: CSS entry keyframes should NOT fire when toggling tabs, only on first page load.
- **Scale-on-press with disabled state** — Buttons have `disabled:opacity-50`. Scale should not apply when disabled. Use `active:not-disabled:scale-[0.96]` pattern from the skill (Tailwind: `active:not(:disabled):scale-[0.96]`).
- **Korean in JSDoc comments** — Comments like `// 무시` and JSDoc describing component behavior in Korean should be LEFT AS-IS. Only user-facing rendered text needs translation. Don't touch `VibeStatus.tsx` lines 8-10 (JSDoc), `page.tsx` line 174 (`// 무시`).
- **Stagger animation + `prefers-reduced-motion`** — Users with motion sensitivity need a way to disable animations. Add `@media (prefers-reduced-motion: reduce) { .stagger-item { animation: none !important; } }` in globals.css.

## Open Risks

- **Mobile tab opacity transition performance** — Switching from `display: none` (hidden) to `opacity: 0 + pointer-events: none` keeps all three tab panels in the DOM. Monaco Editor is in the "editor" tab — it will remain mounted even when hidden. This is actually the CURRENT behavior (all three divs are always mounted, `hidden` class just sets `display: none`). Switching to opacity should be equivalent in memory footprint but may have different compositing behavior. Monitor for any layout/scroll issues.
- **Tailwind v4 `transition-[colors,transform]` syntax** — Need to verify Tailwind v4 supports the combined arbitrary transition shorthand. If not, may need to use a custom CSS class in globals.css instead of inline Tailwind utilities.
