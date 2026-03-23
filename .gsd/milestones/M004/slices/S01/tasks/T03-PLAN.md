---
estimated_steps: 5
estimated_files: 4
---

# T03: Apply design tokens to IDELayout shell and panel components

**Slice:** S01 — Design Foundation — 디자인 시스템 + 레이아웃 셸
**Milestone:** M004

## Description

T01에서 정의한 디자인 토큰을 IDELayout, EditorPanel, SidebarPanel, ConsolePanel에 적용한다. 하드코딩된 Tailwind 기본 클래스(`bg-gray-900`, `border-gray-700`, `bg-amber-600/20` 등)를 새 커스텀 토큰 기반 유틸리티(`bg-surface-base`, `border-border-subtle`, `bg-accent-bg` 등)로 교체하여 IDE 셸의 시각적 기반을 완성한다.

**핵심 스킬**: `make-interfaces-feel-better` — shadows over borders, concentric border radius 등의 원칙 적용

**절대 변경 금지:**
- TAB_CONFIG의 label 텍스트: `"Editor"`, `"Results"`, `"Console"` — E2E 셀렉터가 이 텍스트에 의존
- DOM 구조: `react-resizable-panels`의 `Group`/`Panel`/`Separator` 구조, 모바일 탭바의 `<button>` 구조
- 컴포넌트 props 인터페이스 (다른 컴포넌트가 이 인터페이스에 의존)

**컬러 매핑 가이드** (T01의 `@theme` 토큰 참조):
| 기존 클래스 | 새 클래스 | 용도 |
|---|---|---|
| `bg-gray-900` | `bg-surface-base` | 메인 배경 |
| `bg-gray-800` | `bg-surface-raised` | 패널/탭바/헤더 배경 |
| `border-gray-700` | `border-border-subtle` | 보더 |
| `text-gray-100` | `text-text-primary` | 주 텍스트 |
| `text-gray-200` | `text-text-primary` | 주 텍스트 (약간 밝은) |
| `text-gray-300` | `text-text-secondary` 또는 그대로 유지 | 보조 텍스트 |
| `text-gray-400` | `text-text-secondary` | 보조 텍스트 |
| `bg-amber-600/20` | `bg-accent-bg` | 활성 탭 배경 |
| `text-amber-400` | `text-accent` | 활성 탭 텍스트 |
| `text-amber-500` | `text-accent` | 활성 탭 보더 |
| `border-amber-500` | `border-accent` | 활성 탭 보더 |
| `hover:bg-amber-500` | `hover:bg-accent` | separator 호버 |
| `hover:bg-gray-700/50` | `hover:bg-surface-overlay` | 비활성 탭 호버 |

## Steps

1. **`IDELayout.tsx` — 모바일 레이아웃 토큰 적용**
   - 모바일 루트: `bg-gray-900` → `bg-surface-base`
   - 탭바: `border-gray-700 bg-gray-800` → `border-border-subtle bg-surface-raised`
   - 활성 탭: `bg-amber-600/20 text-amber-400 border-amber-500` → `bg-accent-bg text-accent border-accent`
   - 비활성 탭: `text-gray-400 hover:text-gray-200 hover:bg-gray-700/50` → `text-text-secondary hover:text-text-primary hover:bg-surface-overlay`
   - TAB_CONFIG의 `label` 값은 절대 변경하지 않음

2. **`IDELayout.tsx` — 데스크톱 레이아웃 토큰 적용**
   - 데스크톱 루트: `bg-gray-900` → `bg-surface-base`
   - 수평 Separator: `bg-gray-700 hover:bg-amber-500` → `bg-border-subtle hover:bg-accent`
   - 수직 Separator: 동일 교체

3. **`EditorPanel.tsx` — 토큰 적용**
   - `bg-gray-900 text-gray-100` → `bg-surface-base text-text-primary`

4. **`SidebarPanel.tsx` — 토큰 적용**
   - `border-gray-700 bg-gray-800` → `border-border-subtle bg-surface-raised`
   - 헤더: `border-gray-700/80 bg-gradient-to-r from-gray-800 to-gray-800/90` → `border-border-subtle bg-surface-raised` (gradient 제거, 단색으로 정제)
   - 헤더 shadow: `shadow-[inset_0_-1px_0_rgb(251_191_36/0.1)]` → `shadow-[inset_0_-1px_0_theme(colors.accent/0.1)]` 또는 새 accent 토큰 기반
   - `text-gray-200` → `text-text-primary`

5. **`ConsolePanel.tsx` — 토큰 적용**
   - 루트: `border-gray-700 bg-gray-900` → `border-border-subtle bg-surface-base`
   - 헤더: `bg-gray-800` → `bg-surface-raised`
   - 헤더 텍스트: `text-gray-400` → `text-text-secondary`
   - 내용: `text-gray-300` → `text-text-secondary`
   - font-mono 유지 (JetBrains Mono가 `font-mono`에 연결됨)

## Must-Haves

- [ ] IDELayout.tsx의 모든 하드코딩된 gray/amber 클래스가 새 토큰 기반 유틸리티로 교체됨
- [ ] EditorPanel.tsx, SidebarPanel.tsx, ConsolePanel.tsx 동일하게 교체됨
- [ ] TAB_CONFIG label 텍스트(`"Editor"`, `"Results"`, `"Console"`)가 변경되지 않음
- [ ] DOM 구조(Group/Panel/Separator/button)가 변경되지 않음
- [ ] `npm run build` 성공

## Verification

- `cd /home/ahwlsqja/Vibe-Loom && npm run build` exits with code 0
- `! grep -q "bg-gray-900" /home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx` — 하드코딩된 gray-900 제거
- `! grep -q "bg-gray-800" /home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx` — 하드코딩된 gray-800 제거
- `grep -q "bg-surface-base" /home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx` — 새 토큰 사용
- `grep -q "'Editor'" /home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx` — 탭 라벨 보존
- `grep -q "'Results'" /home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx` — 탭 라벨 보존
- `grep -q "'Console'" /home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx` — 탭 라벨 보존
- `grep -q "bg-surface-base" /home/ahwlsqja/Vibe-Loom/src/components/ide/EditorPanel.tsx`
- `grep -q "bg-surface-raised" /home/ahwlsqja/Vibe-Loom/src/components/ide/SidebarPanel.tsx`
- `grep -q "bg-surface-base" /home/ahwlsqja/Vibe-Loom/src/components/ide/ConsolePanel.tsx`

## Inputs

- `/home/ahwlsqja/Vibe-Loom/src/app/globals.css` — T01에서 정의된 @theme 디자인 토큰 (컬러 매핑 참조)
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx` — 86줄, 3-panel + 모바일 탭 레이아웃
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/EditorPanel.tsx` — 19줄, bg-gray-900 text-gray-100
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/SidebarPanel.tsx` — 20줄, border-gray-700 bg-gray-800
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/ConsolePanel.tsx` — 18줄, border-gray-700 bg-gray-900

## Expected Output

- `/home/ahwlsqja/Vibe-Loom/src/components/ide/IDELayout.tsx` — 하드코딩된 gray/amber → 디자인 토큰 기반 유틸리티
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/EditorPanel.tsx` — 동일 교체
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/SidebarPanel.tsx` — 동일 교체
- `/home/ahwlsqja/Vibe-Loom/src/components/ide/ConsolePanel.tsx` — 동일 교체
