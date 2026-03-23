---
estimated_steps: 4
estimated_files: 3
---

# T02: Create Monaco custom theme and wire into editor components

**Slice:** S01 — Design Foundation — 디자인 시스템 + 레이아웃 셸
**Milestone:** M004

## Description

기본 `vs-dark` Monaco 테마를 디자인 시스템 컬러와 조화되는 커스텀 `vibe-loom-dark` 테마로 교체한다. Monaco의 `defineTheme`은 CSS variable을 참조할 수 없으므로(hex 리터럴 필수), 별도 `monaco-theme.ts` 파일에서 hex 값을 상수로 관리하여 `globals.css`의 디자인 토큰과 "수동 동기화"한다.

**핵심 제약사항:**
- `defineTheme`은 반드시 `beforeMount` 콜백에서 호출해야 함 (onMount에서 호출하면 기본 테마 flash 발생)
- Monaco 테마는 CSS variable 참조 불가 — hex 리터럴만 사용
- AIDiffViewerInner.tsx에는 현재 `beforeMount` 콜백이 없으므로 추가 필요
- 기존 MonacoEditorInner.tsx의 `handleBeforeMount`에서 `registerSolidityLanguage`를 호출 중 — 이를 유지하면서 테마 등록 추가

**T01의 디자인 토큰 컬러와 동기화할 hex 값 (oklch → hex 근사치):**
- surface-base: oklch(0.145 0.014 260) ≈ `#0f1219` — editor.background
- surface-raised: oklch(0.185 0.014 260) ≈ `#161b24`
- border-subtle: oklch(0.30 0.014 260) ≈ `#2a3040`
- accent: oklch(0.75 0.15 185) ≈ `#3dd8c5` (teal)
- accent-muted: oklch(0.55 0.12 185) ≈ `#1d8c80`
- text-primary: oklch(0.93 0.005 260) ≈ `#e8eaed`
- text-secondary: oklch(0.65 0.01 260) ≈ `#8b95a5`
- text-muted: oklch(0.50 0.01 260) ≈ `#636d7d`

## Steps

1. **`src/lib/monaco-theme.ts` 신규 생성**
   - 디자인 토큰과 동기화된 hex 상수 객체 export: `THEME_COLORS` (각 컬러에 주석으로 대응하는 CSS variable 명시)
   - `VIBE_LOOM_THEME_NAME = 'vibe-loom-dark'` 상수 export
   - `defineVibeLoomTheme(monaco: any)` 함수 export — `monaco.editor.defineTheme()` 호출
   - 테마 정의:
     - `base: 'vs-dark'`
     - `inherit: true`
     - `rules`: keyword, string, comment, number, type, function 등 syntax highlighting 토큰 (teal accent 계열)
     - `colors`: `editor.background`, `editor.foreground`, `editor.lineHighlightBackground`, `editorCursor.foreground`, `editor.selectionBackground`, `editor.inactiveSelectionBackground`, `editorLineNumber.foreground`, `editorLineNumber.activeForeground`, `editorGutter.background`, `editor.selectionHighlightBackground`, `editorBracketMatch.border`, `editorBracketMatch.background`, `editorWidget.background`, `editorWidget.border`

2. **`MonacoEditorInner.tsx` — 커스텀 테마 적용**
   - `import { defineVibeLoomTheme, VIBE_LOOM_THEME_NAME } from '@/lib/monaco-theme'` 추가
   - `handleBeforeMount` 콜백 내에서 `registerSolidityLanguage(monaco)` 다음에 `defineVibeLoomTheme(monaco)` 호출
   - `<Editor>` 컴포넌트의 `theme="vs-dark"` → `theme={VIBE_LOOM_THEME_NAME}` 변경

3. **`AIDiffViewerInner.tsx` — 커스텀 테마 적용**
   - `import { defineVibeLoomTheme, VIBE_LOOM_THEME_NAME } from '@/lib/monaco-theme'` 추가
   - `beforeMount` 콜백 추가: `const handleBeforeMount = (monaco: any) => { defineVibeLoomTheme(monaco); }`
   - `<DiffEditor>` 컴포넌트에 `beforeMount={handleBeforeMount}` prop 추가
   - `theme="vs-dark"` → `theme={VIBE_LOOM_THEME_NAME}` 변경

4. **빌드 검증**
   - `cd /home/ahwlsqja/Vibe-Loom && npm run build` — 에러 없이 성공

## Must-Haves

- [ ] `src/lib/monaco-theme.ts` 생성 — `defineVibeLoomTheme()`, `VIBE_LOOM_THEME_NAME` export
- [ ] Monaco 테마 hex 컬러가 T01의 디자인 토큰 oklch 값과 동기화 (주석 매핑)
- [ ] MonacoEditorInner.tsx에서 `beforeMount` 시 커스텀 테마 등록, `theme` prop 변경
- [ ] AIDiffViewerInner.tsx에서 `beforeMount` 콜백 추가, 커스텀 테마 등록, `theme` prop 변경
- [ ] `npm run build` 성공

## Verification

- `cd /home/ahwlsqja/Vibe-Loom && npm run build` exits with code 0
- `test -f /home/ahwlsqja/Vibe-Loom/src/lib/monaco-theme.ts`
- `grep -q 'vibe-loom-dark' /home/ahwlsqja/Vibe-Loom/src/lib/monaco-theme.ts`
- `grep -q 'VIBE_LOOM_THEME_NAME' /home/ahwlsqja/Vibe-Loom/src/components/ide/MonacoEditorInner.tsx`
- `grep -q 'VIBE_LOOM_THEME_NAME' /home/ahwlsqja/Vibe-Loom/src/components/ide/AIDiffViewerInner.tsx`
- `! grep -q "theme=\"vs-dark\"" /home/ahwlsqja/Vibe-Loom/src/components/ide/MonacoEditorInner.tsx` — vs-dark 제거 확인
- `! grep -q "theme=\"vs-dark\"" /home/ahwlsqja/Vibe-Loom/src/components/ide/AIDiffViewerInner.tsx` — vs-dark 제거 확인

## Inputs

- `/home/ahwlsqja/Vibe-Loom/src/components/ide/MonacoEditorInner.tsx` — 현재 `theme="vs-dark"`, `handleBeforeMount`에서 `registerSolidityLanguage` 호출
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/AIDiffViewerInner.tsx` — 현재 `theme="vs-dark"`, beforeMount 콜백 없음
- `/home/ahwlsqja/Vibe-Loom/src/app/globals.css` — T01에서 정의된 디자인 토큰 (hex 동기화 참조용)

## Expected Output

- `/home/ahwlsqja/Vibe-Loom/src/lib/monaco-theme.ts` — 신규 파일, Monaco 커스텀 테마 정의 + hex 컬러 상수
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/MonacoEditorInner.tsx` — 커스텀 테마 import + beforeMount에서 등록 + theme prop 변경
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/AIDiffViewerInner.tsx` — 커스텀 테마 import + beforeMount 콜백 추가 + theme prop 변경
