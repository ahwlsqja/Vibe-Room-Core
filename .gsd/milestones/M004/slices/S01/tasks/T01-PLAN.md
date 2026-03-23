---
estimated_steps: 4
estimated_files: 2
---

# T01: Define Tailwind v4 design tokens in globals.css and load custom fonts in layout.tsx

**Slice:** S01 — Design Foundation — 디자인 시스템 + 레이아웃 셸
**Milestone:** M004

## Description

모든 후속 작업의 기반이 되는 디자인 토큰과 폰트 시스템을 구축한다. Tailwind v4의 CSS-first `@theme` 디렉티브를 사용하여 `globals.css`에 컬러, 폰트, 스페이싱, 모션 토큰을 정의하고, `layout.tsx`에서 `next/font/google`으로 커스텀 폰트를 로드하여 CSS variable로 연결한다.

**중요한 스킬을 로드하세요**: `frontend-design` 스킬과 `make-interfaces-feel-better` 스킬의 원칙을 적용하여 디자인 토큰을 설계하세요.

**핵심 제약사항:**
- Tailwind v4 CSS-first 설정: `tailwind.config.ts`가 없음. 모든 커스터마이징은 `globals.css`의 `@theme` 블록에서 CSS variable로 수행
- `--color-*: initial` 사용 금지! 기존 Tailwind 기본 컬러(`bg-gray-900`, `text-gray-300` 등)가 사라져서 기존 코드가 깨짐. 커스텀 컬러는 **추가**만 함
- `next/font`의 폰트 로더는 `layout.tsx`에서만 호출 가능 (컴포넌트 내 호출 불가)
- 폰트 variable 연결: `next/font`에서 `variable: '--font-geist-sans'`로 CSS variable 생성 → `@theme`에서 `--font-sans: var(--font-geist-sans)` 연결 → Tailwind `font-sans` 유틸리티가 동작
- 기존 globals.css의 `@import "tailwindcss"`, focus-visible, tabular-nums, panel-transition 규칙 유지

## Steps

1. **`layout.tsx` — 커스텀 폰트 로드 및 CSS variable 바인딩**
   - `import { Geist } from 'next/font/google'`와 `import { JetBrains_Mono } from 'next/font/google'` 추가
   - Geist Sans: `const geistSans = Geist({ subsets: ['latin'], variable: '--font-geist-sans', display: 'swap' })`
   - JetBrains Mono: `const jetbrainsMono = JetBrains_Mono({ subsets: ['latin'], variable: '--font-geist-mono', display: 'swap' })`
   - `<html>` 태그에 `className={`${geistSans.variable} ${jetbrainsMono.variable}`}` 추가
   - `<body>` 태그에 `className="bg-surface-base text-gray-100 font-sans"` 추가 (새 토큰 사용)
   - 기존 `<Providers>` 래퍼, metadata export 유지

2. **`globals.css` — `@theme` 블록에 디자인 토큰 정의**
   - 기존 `@import "tailwindcss"` 바로 아래에 `@theme` 블록 추가
   - **컬러 토큰** (Refined Technical 다크 팔레트 — Bloomberg Terminal 느낌의 cool blue tint):
     - `--color-surface-base: oklch(0.145 0.014 260)` — 메인 배경 (기존 gray-900 대체용)
     - `--color-surface-raised: oklch(0.185 0.014 260)` — 패널/카드 (기존 gray-800 대체용)
     - `--color-surface-overlay: oklch(0.22 0.014 260)` — 오버레이/호버
     - `--color-border-subtle: oklch(0.30 0.014 260)` — 기본 보더 (기존 gray-700 대체용)
     - `--color-border-active: oklch(0.55 0.15 185)` — 활성 보더/포커스 (teal)
     - `--color-accent: oklch(0.75 0.15 185)` — 주 액센트 (teal/cyan — amber 대체)
     - `--color-accent-muted: oklch(0.55 0.12 185)` — 약한 액센트
     - `--color-accent-bg: oklch(0.25 0.06 185)` — 액센트 배경
     - `--color-text-primary: oklch(0.93 0.005 260)` — 주 텍스트
     - `--color-text-secondary: oklch(0.65 0.01 260)` — 보조 텍스트
     - `--color-text-muted: oklch(0.50 0.01 260)` — 약한 텍스트
   - **폰트 토큰:**
     - `--font-sans: var(--font-geist-sans), ui-sans-serif, system-ui, sans-serif`
     - `--font-mono: var(--font-geist-mono), ui-monospace, monospace`
   - **스페이싱 토큰** (별도 네임스페이스, 기본 spacing 건드리지 않음):
     - `--space-xs: 0.25rem`, `--space-sm: 0.5rem`, `--space-md: 0.75rem`, `--space-lg: 1rem`, `--space-xl: 1.5rem`, `--space-2xl: 2rem`
   - **모션 토큰:**
     - `--ease-snappy: cubic-bezier(0.2, 0, 0, 1)`
     - `--ease-fluid: cubic-bezier(0.4, 0, 0.2, 1)`
     - `--duration-fast: 100ms`
     - `--duration-normal: 200ms`
     - `--duration-slow: 350ms`

3. **globals.css의 기존 규칙 업데이트**
   - `panel-transition`의 timing-function을 `var(--ease-snappy)`로 교체
   - focus-visible outline 색상을 `var(--color-accent)` 계열로 교체

4. **빌드 검증**
   - `cd /home/ahwlsqja/Vibe-Loom && npm run build` — 에러 없이 성공 확인
   - `@theme` 블록에 정의된 CSS variables가 Tailwind 유틸리티로 사용 가능한지 확인 (예: `bg-surface-base`, `text-accent`, `font-sans`)

## Must-Haves

- [ ] `@theme` 블록에 컬러 11개, 폰트 2개, 스페이싱 6개, 모션 5개 토큰 정의
- [ ] `next/font/google`으로 Geist Sans + JetBrains Mono 로드, CSS variable 바인딩
- [ ] `@theme`의 `--font-sans`/`--font-mono`가 `next/font` variable과 연결
- [ ] `--color-*: initial` 미사용 (기존 Tailwind 기본 컬러 유지)
- [ ] `npm run build` 성공

## Verification

- `cd /home/ahwlsqja/Vibe-Loom && npm run build` exits with code 0
- `grep -q '@theme' /home/ahwlsqja/Vibe-Loom/src/app/globals.css`
- `grep -q 'JetBrains_Mono' /home/ahwlsqja/Vibe-Loom/src/app/layout.tsx`
- `grep -c 'color-surface\|color-accent\|color-border\|color-text' /home/ahwlsqja/Vibe-Loom/src/app/globals.css` returns >= 10
- `! grep -q 'color-.*: initial' /home/ahwlsqja/Vibe-Loom/src/app/globals.css` — initial override 없음

## Inputs

- `/home/ahwlsqja/Vibe-Loom/src/app/globals.css` — 현재 29줄, @import "tailwindcss" + 글로벌 유틸리티
- `/home/ahwlsqja/Vibe-Loom/src/app/layout.tsx` — 현재 20줄, metadata + RootLayout, 폰트 로드 없음

## Expected Output

- `/home/ahwlsqja/Vibe-Loom/src/app/globals.css` — @theme 블록에 디자인 토큰 정의 추가 (컬러/폰트/스페이싱/모션)
- `/home/ahwlsqja/Vibe-Loom/src/app/layout.tsx` — Geist Sans + JetBrains Mono 폰트 로드, <html> className에 variable 바인딩, <body>에 기본 스타일 적용

## Observability Impact

- **Runtime signals:** `@theme` CSS variables are inspectable via browser DevTools → Computed Styles on any element. Font variable presence (`--font-geist-sans`, `--font-geist-mono`) on `<html>` confirms font loading.
- **Inspection surface:** `document.documentElement.style.getPropertyValue('--font-geist-sans')` in browser console shows font family if loaded correctly. Computed style `getComputedStyle(document.body).fontFamily` confirms Geist Sans is applied.
- **Failure visibility:** If font fails to load, `<html>` will lack the font variable class → `font-sans` utility falls back to `ui-sans-serif, system-ui` stack. If `@theme` block has syntax errors, `npm run build` fails with Tailwind CSS parse error (line number in build output).
- **Build-time check:** `grep -c 'color-surface\|color-accent\|color-border\|color-text' src/app/globals.css` returns >= 10 confirms all color tokens present.
