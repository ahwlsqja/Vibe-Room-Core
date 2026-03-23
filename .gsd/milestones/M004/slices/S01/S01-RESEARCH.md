# S01: Design Foundation — 디자인 시스템 + 레이아웃 셸 — Research

**Date:** 2026-03-23
**Depth:** Deep research — new design system, Tailwind v4 CSS-first @theme, Monaco custom theming, font strategy

## Summary

이 슬라이스는 Vibe-Loom 전체 UI 리디자인의 기초를 세우는 작업이다. 현재 코드베이스는 **디자인 시스템이 전혀 없다** — Tailwind 기본 gray-900/800/700 + amber 액센트를 하드코딩으로 사용하며, 커스텀 폰트 없음(시스템 기본), 스페이싱/모션 토큰 없음, Monaco는 기본 `vs-dark` 테마를 사용한다. 이 슬라이스의 산출물은: (1) `globals.css`에 Tailwind v4 `@theme` 디렉티브를 사용한 디자인 토큰 정의, (2) `layout.tsx`에 `next/font`로 커스텀 폰트 적용, (3) Monaco 커스텀 테마 정의, (4) `IDELayout.tsx`를 새 디자인 시스템으로 리팩토링.

현재 Tailwind v4는 **CSS-first 설정**을 사용하므로 `tailwind.config.ts`가 없고 `globals.css` 내 `@theme` 블록에서 모든 커스텀 토큰을 정의한다. 이미 `@import "tailwindcss"`가 설정되어 있어 기반은 준비되어 있다. Monaco 테마는 `beforeMount` 콜백에서 `monaco.editor.defineTheme()`으로 등록하는데, 이미 `MonacoEditorInner.tsx`에 `handleBeforeMount` 콜백이 있어 여기에 테마 정의를 추가하면 된다.

**주요 리스크:** E2E 테스트 셀렉터가 `getByText('Compile')`, `getByText('Deploy')`, `getByRole('button', { name: 'Editor' })` 등 텍스트 기반이므로, 버튼 텍스트와 tab label을 변경하면 안 된다. 레이아웃 구조(3-panel resizable + 모바일 탭)는 유지하면서 시각적으로만 변경해야 한다.

## Recommendation

**Refined Technical 다크 팔레트 + JetBrains Mono + Geist Sans 조합**으로 디자인 시스템을 구축한다.

1. **컬러:** pure gray-900 (#111827)을 벗어나 oklch 기반의 의도적인 다크 팔레트 구성. 배경은 약간의 cool blue 틴트를 가진 `--color-surface-*` 시리즈(3~4단계), 액센트는 amber에서 더 정제된 teal/cyan 계열로 전환(Bloomberg Terminal 느낌).
2. **타이포그래피:** `next/font/google`으로 Geist Sans (UI)와 JetBrains Mono (코드/콘솔) 로드. Inter/system-ui가 아닌 의도적 선택.
3. **스페이싱:** 4px 기반 스페이싱 스케일을 `@theme`에 정의. 기존 Tailwind 기본값 대신 커스텀 스케일 사용.
4. **모션 토큰:** `--ease-snappy`, `--ease-fluid`, `--duration-fast/normal/slow` 정의. S03에서 실제 애니메이션 적용 시 사용.
5. **Monaco 테마:** 디자인 시스템 컬러와 조화되는 커스텀 테마를 `defineTheme()`으로 등록. 배경색, 키워드 색, 주석 색 등 주요 토큰 커스터마이징.
6. **IDELayout:** 새 컬러 토큰 적용, 패널 분리선 스타일 개선, 모바일 탭바 재디자인. **구조/기능은 동일 유지.**

## Implementation Landscape

### Key Files

- **`src/app/globals.css`** (29줄) — 현재 최소한의 글로벌 스타일. 여기에 `@theme` 블록으로 전체 디자인 토큰 정의 추가. 가장 핵심 파일.
- **`src/app/layout.tsx`** (20줄) — `next/font/google`로 Geist Sans + JetBrains Mono 로드하고, CSS variable로 `<html>`에 바인딩. `<body>`에 기본 배경색/폰트 적용.
- **`src/components/ide/MonacoEditorInner.tsx`** (58줄) — `handleBeforeMount`에서 커스텀 Monaco 테마를 `defineTheme()`으로 등록. `theme` prop을 `"vs-dark"`에서 커스텀 테마명으로 변경.
- **`src/components/ide/AIDiffViewerInner.tsx`** (52줄) — DiffEditor도 동일한 커스텀 테마 적용. `theme="vs-dark"` → 커스텀 테마명.
- **`src/components/ide/IDELayout.tsx`** (86줄) — 데스크톱 3-panel + 모바일 탭 레이아웃. 하드코딩된 `bg-gray-900`, `border-gray-700`, `bg-amber-600/20` 등을 새 디자인 토큰 기반 유틸리티로 교체.
- **`src/components/ide/EditorPanel.tsx`** (19줄) — `bg-gray-900 text-gray-100` → 새 토큰.
- **`src/components/ide/SidebarPanel.tsx`** (20줄) — `border-gray-700 bg-gray-800` + gradient 헤더 → 새 토큰 기반.
- **`src/components/ide/ConsolePanel.tsx`** (18줄) — `border-gray-700 bg-gray-900` → 새 토큰.

### Build Order

1. **`globals.css` — 디자인 토큰 정의** (최우선)
   - `@theme` 블록에 컬러, 폰트, 스페이싱, 모션 토큰 정의
   - 이것이 모든 후속 작업의 기반. 이 파일이 완성되어야 나머지 컴포넌트에서 토큰을 참조할 수 있음
   - 추가 글로벌 유틸리티 클래스도 여기에 정의

2. **`layout.tsx` — 폰트 로딩** (globals.css와 거의 동시)
   - `next/font/google`으로 Geist Sans + JetBrains Mono import
   - CSS variable 바인딩 (`--font-sans`, `--font-mono`)
   - `@theme`의 `--font-sans`/`--font-mono`와 연결

3. **Monaco 커스텀 테마** (`MonacoEditorInner.tsx` + `AIDiffViewerInner.tsx`)
   - `handleBeforeMount`에서 `monaco.editor.defineTheme('vibe-loom-dark', {...})` 호출
   - 디자인 토큰 컬러 참조하여 `editor.background`, `editor.foreground`, syntax 토큰 정의
   - 별도 파일로 테마 JSON 추출 가능 (`src/lib/monaco-theme.ts`)

4. **IDELayout + 패널 컴포넌트** (글로벌 토큰 완성 후)
   - `IDELayout.tsx`, `EditorPanel.tsx`, `SidebarPanel.tsx`, `ConsolePanel.tsx` 스타일 업데이트
   - 하드코딩된 Tailwind 기본 클래스 → 커스텀 토큰 기반 유틸리티
   - 레이아웃 구조(Panel/Group/Separator)는 변경 없이 유지

### Verification Approach

1. **빌드 성공:** `npm run build` — Tailwind v4 `@theme` 파싱 에러 없이 빌드 성공
2. **시각 확인:** `npm run dev` 후 브라우저에서 IDE 레이아웃이 새 컬러/폰트로 렌더링 확인
3. **Monaco 테마:** 에디터 영역이 커스텀 테마로 렌더링되는지 확인 (기본 vs-dark의 #1e1e1e 배경이 아닌 커스텀 배경)
4. **E2E 기본 호환:** `getByText('Compile')`, `getByText('Deploy')`, `getByRole('button', { name: 'Editor' })` 등 핵심 셀렉터가 여전히 매칭되는지 수동 확인 (전체 E2E는 S04에서 실행)
5. **모바일 레이아웃:** 375×812 뷰포트에서 탭바가 정상 렌더링되는지 확인

## Don't Hand-Roll

| Problem | Existing Solution | Why Use It |
|---------|------------------|------------|
| 폰트 최적화/셀프호스팅 | `next/font/google` | Next.js 빌트인. 자동 셀프호스팅, layout shift 방지, subset 최적화 |
| CSS 디자인 토큰 | Tailwind v4 `@theme` | 이미 Tailwind v4 사용 중. `@theme`에 정의하면 자동으로 유틸리티 클래스 생성 |
| Monaco 테마 | `monaco.editor.defineTheme()` | Monaco 빌트인 API. 별도 라이브러리 불필요 |
| 리사이저블 패널 | `react-resizable-panels` | 이미 사용 중. 교체 불필요 |

## Constraints

- **Tailwind v4 CSS-first:** `tailwind.config.ts`가 없음. 모든 커스터마이징은 `globals.css`의 `@theme` 블록에서 CSS variable로 수행. JS 설정 파일 생성하지 않는다.
- **E2E 텍스트 셀렉터 보존:** 버튼 텍스트(`Compile`, `Deploy`, `Vibe Score`, `Editor`, `Results`, `Console`, `FixedContract`, `FailingContract` 등)와 상태 텍스트(`컴파일 중...`, `배포 중...`, `분석 중...`)를 변경하면 E2E 깨짐. S01에서는 텍스트 변경 금지.
- **레이아웃 DOM 구조 유지:** `react-resizable-panels`의 `Group`/`Panel`/`Separator` 구조, 모바일 탭바의 `<button>` 구조를 변경하면 E2E 셀렉터 및 기능 깨짐. 스타일만 변경.
- **Monaco 테마 한계:** Monaco `defineTheme`은 CSS variable을 직접 참조할 수 없음 — hex/rgb 리터럴 필요. 디자인 토큰과 Monaco 테마를 "수동 동기화"해야 함. 별도 `monaco-theme.ts` 파일에서 hex 값을 상수로 관리하고, CSS 변수와 동일한 값을 사용.
- **`next/font`는 `layout.tsx` 전용:** `next/font`의 폰트 로더는 `layout.tsx` (또는 route segment)에서만 호출 가능. 컴포넌트 내에서 호출 불가.

## Common Pitfalls

- **`@theme`에서 기존 Tailwind 유틸리티 override 주의** — `--color-*: initial`로 모든 기본 컬러를 리셋하면 `bg-gray-900` 같은 기존 유틸리티가 사라짐. 기존 기본 컬러를 유지하면서 커스텀 컬러만 **추가**해야 함. `--color-*: initial`은 사용하지 않는다.
- **폰트 variable 연결 누락** — `next/font`에서 `variable: '--font-sans'`로 CSS variable을 생성하고, `@theme`에서 `--font-sans: var(--font-sans)`처럼 연결해야 Tailwind의 `font-sans` 유틸리티가 동작. 두 단계를 모두 해야 함.
- **Monaco 테마 등록 타이밍** — `defineTheme`은 반드시 `beforeMount`에서 호출. `onMount` 시점에서 호출하면 초기 렌더링에 기본 테마가 잠깐 보이는 flash 발생.
- **oklch 브라우저 지원** — oklch는 2024년 기준 모든 주요 브라우저에서 지원하지만, 안전하게 hex 폴백을 주석으로 남겨두는 것이 좋음.
- **`-webkit-font-smoothing: antialiased`** — 이미 globals.css에 적용되어 있음. 중복 적용하지 않도록 주의.

## Open Risks

- **Tailwind v4 `@theme`에서 커스텀 spacing 정의 시 기본 spacing 스케일과의 충돌** — `--spacing: 0.25rem`으로 base unit을 변경하면 모든 spacing 유틸리티(`p-4`, `gap-2` 등)의 값이 변경됨. 기존 코드의 spacing 의도와 달라질 수 있음. 안전하게 커스텀 spacing은 별도 네임스페이스(`--space-*`)로 정의하고 기본 spacing은 건드리지 않는 전략이 나을 수 있음.
- **Geist Sans 한글 지원** — Geist Sans는 한글 글리프가 없을 수 있음. Korean subset이 없으면 한글은 시스템 폰트로 폴백. Pretendard 등 한글 지원 폰트를 fallback으로 추가하거나, 한영 혼용 UX 카피 정리(S03에서 수행)와 조합하여 해결.

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| Tailwind CSS v4 | `secondsky/claude-skills@tailwind-v4-shadcn` (425 installs) | available — 설치 권장 |
| Tailwind CSS v4 | `josiahsiegel/claude-plugin-marketplace@tailwindcss-fundamentals-v4` (151 installs) | available |
| Frontend Design | `frontend-design` | installed ✓ |
| Interface Polish | `make-interfaces-feel-better` | installed ✓ |
| Monaco Editor | no relevant skill found | none |

## Sources

- Tailwind v4 `@theme` 디렉티브 사용법 (source: [Tailwind CSS v4 docs via Context7](https://context7.com/tailwindlabs/tailwindcss.com))
- Monaco React `defineTheme` / `beforeMount` API (source: [monaco-react docs via Context7](https://context7.com/suren-atoyan/monaco-react))
- Next.js `next/font/google` 폰트 최적화 (source: [Next.js docs via Context7](https://github.com/vercel/next.js/blob/canary/docs/01-app/01-getting-started/13-fonts.mdx))
- impeccable 디자인 안티패턴: gray text on colored bg, pure black/white, Inter/system fonts, cards-in-cards, gradient text — 마일스톤 로드맵에서 참조된 원칙
- `make-interfaces-feel-better` 스킬: concentric border radius, stagger animations, font smoothing, tabular-nums, shadows over borders — 구현 시 적용할 원칙들
