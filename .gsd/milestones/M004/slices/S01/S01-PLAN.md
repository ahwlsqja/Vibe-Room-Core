# S01: Design Foundation — 디자인 시스템 + 레이아웃 셸

**Goal:** 디자인 토큰(폰트, 컬러, 스페이싱, 모션), globals.css, 커스텀 Monaco 테마가 정의되고 IDELayout이 새 디자인으로 렌더링됨
**Demo:** 브라우저에서 `npm run dev` 실행 시 IDE가 새로운 컬러 팔레트(cool dark tones), 커스텀 폰트(Geist Sans + JetBrains Mono), 커스텀 Monaco 에디터 테마로 렌더링됨. 기존 기능(버튼, 탭, 패널 리사이즈)은 모두 동일하게 동작.

## Must-Haves

- Tailwind v4 `@theme` 블록에 컬러(`--color-surface-*`, `--color-accent-*`), 폰트(`--font-sans`, `--font-mono`), 스페이싱(`--space-*`), 모션(`--ease-*`, `--duration-*`) 토큰 정의
- `next/font/google`으로 Geist Sans(UI) + JetBrains Mono(코드) 로드, CSS variable로 `<html>`에 바인딩
- Monaco 커스텀 테마(`vibe-loom-dark`) 정의 및 에디터/DiffEditor에 적용 (기본 `vs-dark` 대체)
- IDELayout, EditorPanel, SidebarPanel, ConsolePanel의 하드코딩된 Tailwind 기본 클래스를 새 토큰 기반 유틸리티로 교체
- E2E 셀렉터 호환성 유지: 버튼 텍스트(`Compile`, `Deploy`, `Vibe Score`, `Editor`, `Results`, `Console`)와 DOM 구조 변경 없음
- `npm run build` 성공 (Tailwind v4 `@theme` 파싱 에러 없음)

## Proof Level

- This slice proves: contract (디자인 토큰이 정의되고 레이아웃 셸에 적용됨)
- Real runtime required: yes (브라우저에서 시각적 렌더링 확인 필요)
- Human/UAT required: yes (시각적 품질은 스크린샷으로 판단)

## Verification

- `cd /home/ahwlsqja/Vibe-Loom && npm run build` — 빌드 성공, @theme 파싱 에러 없음
- `grep -q '@theme' /home/ahwlsqja/Vibe-Loom/src/app/globals.css` — @theme 블록 존재
- `grep -q 'vibe-loom-dark' /home/ahwlsqja/Vibe-Loom/src/lib/monaco-theme.ts` — Monaco 테마 파일 존재
- `grep -q 'JetBrains_Mono' /home/ahwlsqja/Vibe-Loom/src/app/layout.tsx` — 커스텀 폰트 로드 확인
- `! grep -q "bg-gray-900" /home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx` — 하드코딩된 기본 Tailwind gray 제거 확인
- E2E 텍스트 보존: `grep -q "'Editor'" /home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx && grep -q "'Results'" /home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx && grep -q "'Console'" /home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx` — 탭 라벨 변경 없음

## Integration Closure

- Upstream surfaces consumed: none (첫 슬라이스)
- New wiring introduced in this slice: `@theme` 디자인 토큰 → Tailwind 유틸리티 클래스, `next/font` CSS variable → `@theme` 연결, Monaco `defineTheme` → `beforeMount` 콜백
- What remains before the milestone is truly usable end-to-end: S02(코어 컴포넌트 리팩토링), S03(모션+모바일+UX카피), S04(E2E 회귀 검증)

## Observability / Diagnostics

- **Runtime signals:** CSS custom properties from `@theme` block are inspectable in DevTools Computed Styles. Font variables on `<html>` element confirm font-loading state. Monaco theme name `vibe-loom-dark` visible in editor's theme picker / `monaco.editor.getModels()`.
- **Inspection surfaces:** Browser console: `getComputedStyle(document.documentElement).getPropertyValue('--color-accent')` returns oklch value. `document.querySelector('html').className` contains `--font-geist-sans` and `--font-geist-mono` variable classes. `monaco.editor.getEditors()[0].getOption(monaco.editor.EditorOption.theme)` returns `'vibe-loom-dark'`.
- **Failure visibility:** Broken `@theme` syntax → `npm run build` fails with Tailwind parse error (stderr). Missing font → `font-sans` falls back to system stack (visible in computed fontFamily). Monaco theme registration failure → console error on editor mount, editor falls back to `vs-dark`.
- **Redaction constraints:** No secrets or PII in this slice. All tokens are CSS design values.

## Tasks

- [x] **T01: Define Tailwind v4 design tokens in globals.css and load custom fonts in layout.tsx** `est:45m`
  - Why: 모든 후속 작업의 기반. @theme 토큰이 정의되어야 컴포넌트에서 참조 가능하고, 폰트가 로드되어야 타이포그래피 개선 가능
  - Files: `/home/ahwlsqja/Vibe-Loom/src/app/globals.css`, `/home/ahwlsqja/Vibe-Loom/src/app/layout.tsx`
  - Do: (1) globals.css에 `@theme` 블록 추가 — 컬러(surface 3단계 + accent + semantic), 폰트(sans/mono), 스페이싱(별도 네임스페이스 `--space-*`), 모션(ease/duration) 토큰 정의. 기존 Tailwind 기본 컬러는 override하지 않음(추가만). (2) layout.tsx에 `next/font/google`으로 Geist Sans + JetBrains Mono import, `variable` 옵션으로 CSS variable 생성, `<html>`의 `className`에 바인딩. (3) body에 새 배경색/폰트 적용. 기존 globals.css의 focus-visible, tabular-nums, panel-transition 유틸리티 유지.
  - Verify: `cd /home/ahwlsqja/Vibe-Loom && npm run build` 성공 + `grep -q '@theme' src/app/globals.css`
  - Done when: @theme 블록에 컬러/폰트/스페이싱/모션 토큰이 정의되고, layout.tsx에서 커스텀 폰트가 로드되며, `npm run build` 성공
- [x] **T02: Create Monaco custom theme and wire into editor components** `est:30m`
  - Why: 기본 `vs-dark` 테마는 디자인 시스템 컬러와 불일치. 커스텀 테마로 에디터 영역의 시각적 통일성 확보
  - Files: `/home/ahwlsqja/Vibe-Loom/src/lib/monaco-theme.ts` (신규), `/home/ahwlsqja/Vibe-Loom/src/components/ide/MonacoEditorInner.tsx`, `/home/ahwlsqja/Vibe-Loom/src/components/ide/AIDiffViewerInner.tsx`
  - Do: (1) `src/lib/monaco-theme.ts` 신규 생성 — `VIBE_LOOM_THEME_NAME` 상수 + `defineVibeLoomTheme(monaco)` 함수 export. 테마 base는 `'vs-dark'`, 디자인 시스템 컬러와 동일한 hex 값 사용(CSS variable 참조 불가, hex 리터럴 필수). editor.background, editor.foreground, syntax token colors 정의. (2) MonacoEditorInner.tsx의 `handleBeforeMount`에서 `defineVibeLoomTheme(monaco)` 호출, `theme` prop을 `VIBE_LOOM_THEME_NAME`으로 변경. (3) AIDiffViewerInner.tsx에도 동일한 테마 적용 — `beforeMount` 콜백 추가 + `theme` prop 변경.
  - Verify: `cd /home/ahwlsqja/Vibe-Loom && npm run build` 성공 + `grep -q 'vibe-loom-dark' src/lib/monaco-theme.ts`
  - Done when: Monaco 에디터와 DiffEditor가 `vibe-loom-dark` 커스텀 테마를 사용하고, 빌드 성공
- [x] **T03: Apply design tokens to IDELayout shell and panel components** `est:45m`
  - Why: 디자인 토큰이 정의되었으므로, 레이아웃 셸과 패널 컴포넌트의 하드코딩된 Tailwind 기본 클래스를 새 토큰 기반 유틸리티로 교체. 이것이 S01의 시각적 산출물.
  - Files: `/home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx`, `/home/ahwlsqja/Vibe-Loom/src/components/ide/EditorPanel.tsx`, `/home/ahwlsqja/Vibe-Loom/src/components/ide/SidebarPanel.tsx`, `/home/ahwlsqja/Vibe-Loom/src/components/ide/ConsolePanel.tsx`
  - Do: (1) IDELayout.tsx — `bg-gray-900` → 새 surface 토큰, `border-gray-700` → 새 border 토큰, `bg-gray-800` → surface 변형, `bg-amber-600/20` → accent 토큰, `text-amber-400` → accent text 토큰, `hover:bg-amber-500` separator hover 교체. TAB_CONFIG의 label 텍스트(`Editor`, `Results`, `Console`)는 절대 변경 금지. DOM 구조(Group/Panel/Separator/button) 변경 금지. (2) EditorPanel.tsx — `bg-gray-900 text-gray-100` → 새 토큰. (3) SidebarPanel.tsx — `border-gray-700 bg-gray-800`, gradient 헤더 → 새 토큰 기반 스타일. (4) ConsolePanel.tsx — `border-gray-700 bg-gray-900`, `bg-gray-800`, 텍스트 색상 → 새 토큰. (5) `npm run build`로 전체 빌드 검증.
  - Verify: `cd /home/ahwlsqja/Vibe-Loom && npm run build` 성공 + `! grep -q "bg-gray-900" src/components/ide/IDELayout.tsx`
  - Done when: 4개 레이아웃 컴포넌트가 새 디자인 토큰 기반 유틸리티를 사용하고, 기존 텍스트/DOM 구조 유지, 빌드 성공

## Files Likely Touched

- `/home/ahwlsqja/Vibe-Loom/src/app/globals.css`
- `/home/ahwlsqja/Vibe-Loom/src/app/layout.tsx`
- `/home/ahwlsqja/Vibe-Loom/src/lib/monaco-theme.ts` (신규)
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/MonacoEditorInner.tsx`
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/AIDiffViewerInner.tsx`
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx`
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/EditorPanel.tsx`
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/SidebarPanel.tsx`
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/ConsolePanel.tsx`
