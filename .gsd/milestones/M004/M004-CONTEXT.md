# M004: Vibe-Loom UI Redesign — Refined Technical Aesthetic

**Gathered:** 2026-03-23
**Status:** Ready for planning

## Project Description

Vibe-Loom 프론트엔드를 impeccable 디자인 원칙 기반으로 전면 재설계한다. 현재 전형적인 "AI가 만든" 다크모드 IDE 스타일(gray-900 + amber accent + gradient text)에서 탈피하여, Bloomberg Terminal 수준의 정보 밀도와 세련된 타이포그래피/컬러/모션을 갖춘 Refined Technical 심미성을 구현한다.

## Why This Milestone

현재 UI는 기능적으로 동작하지만 시각적으로 generic하다. Tailwind 기본값 + 회색 배경 + amber 액센트 조합은 모든 "AI IDE 프로토타입"에서 볼 수 있는 패턴이다. 모나드 생태계에서 차별화되려면 사용자가 첫 로드에서 "이건 다르다"고 느낄 수 있는 수준의 디자인 퀄리티가 필요하다.

impeccable (pbakaus/impeccable, 12.6k stars)의 7개 도메인별 레퍼런스(typography, color, spatial, motion, interaction, responsive, UX writing) + 안티패턴 가이드를 적용한다.

## User-Visible Outcome

### When this milestone is complete, the user can:

- 브라우저에서 vibe-loom.xyz 접속 시 시각적으로 확연히 다른 Refined Technical UI를 경험한다
- 데스크톱에서 정보 밀도가 높고 시각적 위계가 명확한 IDE 레이아웃을 사용한다
- 모바일에서 단순 축소가 아닌 컨텍스트에 맞게 재구성된 인터페이스를 사용한다
- 모든 인터랙션이 의도적이고 반응감 있는 모션으로 피드백을 준다
- 기존 모든 기능(컴파일, 배포, 바이브 스코어, AI 분석, 컨트랙트 인터랙션)이 동일하게 동작한다

### Entry point / environment

- Entry point: https://vibe-loom.xyz (Next.js frontend)
- Environment: browser (Chrome, Safari, Firefox, mobile)
- Live dependencies involved: Railway backend API, Monad testnet

## Completion Class

- Contract complete means: 모든 컴포넌트가 새 디자인 시스템으로 렌더링되고, 기존 E2E 테스트 22개가 여전히 통과
- Integration complete means: 라이브 서비스에서 full flow (로드→편집→컴파일→배포→AI 분석) 동작
- Operational complete means: none (인프라 변경 없음)

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- 기존 22개 E2E 테스트가 새 UI에서 전부 통과 (기능 회귀 없음)
- 새 디자인 시스템이 전체 컴포넌트에 일관적으로 적용됨 — 타이포그래피, 컬러, 스페이싱
- 데스크톱 + 모바일(375×812) 양쪽에서 레이아웃이 의도대로 렌더링됨
- 로드 시 entry animation, 인터랙션 시 모션 피드백이 존재함

## Risks and Unknowns

- **Monaco Editor 스타일링 한계** — Monaco는 자체 테마 시스템을 사용하므로 외부 CSS로 완전 커스터마이징이 어려울 수 있음. 커스텀 Monaco 테마로 대응.
- **E2E 테스트 셀렉터 깨짐** — DOM 구조 변경 시 기존 Playwright 셀렉터가 실패할 수 있음. 기능 회귀 테스트는 마지막 슬라이스에서 수행.
- **Tailwind v4 + 커스텀 디자인 토큰 호환성** — 새 디자인 토큰을 Tailwind v4와 통합하는 과정에서 예상치 못한 이슈 가능.

## Existing Codebase / Prior Art

- `src/app/page.tsx` (394 lines) — 메인 IDE 페이지. 컨트랙트 셀렉터, 에디터, 사이드바, 콘솔 오케스트레이션
- `src/components/ide/IDELayout.tsx` (86 lines) — 3-panel resizable + 모바일 탭 레이아웃
- `src/components/ide/*.tsx` (13 files, ~1400 lines) — 에디터, 사이드바, 콘솔, 바이브 스코어 등
- `src/app/globals.css` (29 lines) — 최소 글로벌 스타일
- `src/app/layout.tsx` — HTML 셸, 메타데이터

## Relevant Requirements

- R009 — Next.js 프론트엔드 (UI 전환). 이 마일스톤은 기능 변경 없이 시각적 개선만 수행
- R016 — 프론트엔드 UI 재디자인은 out-of-scope으로 선언되었으나, 사용자가 명시적으로 요청했으므로 이 마일스톤에서 수행

## Scope

### In Scope

- 디자인 시스템 구축: 타이포그래피 스케일, 컬러 팔레트, 스페이싱 시스템, 모션 토큰
- 전체 컴포넌트 시각적 리팩토링 (13 컴포넌트 + page.tsx)
- Monaco Editor 커스텀 테마
- 페이지 로드 애니메이션, 인터랙션 모션
- 모바일 레이아웃 재설계 (적응형, 축소가 아닌)
- UX 카피 정리 (한/영 혼용 해결)
- 기존 E2E 테스트 호환성 유지

### Out of Scope / Non-Goals

- 새로운 기능 추가 (컴파일, 배포, AI 분석 등은 동일)
- 백엔드 API 변경
- 새로운 페이지/라우트 추가
- 랜딩 페이지/마케팅 페이지

## Technical Constraints

- Next.js 15 + React 19 + Tailwind CSS v4
- 기존 react-resizable-panels 유지 (데스크톱 리사이즈)
- Monaco Editor React 패키지 유지 (@monaco-editor/react)
- 기존 API 클라이언트/훅 인터페이스 동일

## Integration Points

- Railway 백엔드 — API 호출 인터페이스 변경 없음
- Monad testnet — 배포 플로우 변경 없음
- GitHub OAuth — 로그인 UI 스타일만 변경
- WalletConnect — 모달 스타일만 변경

## Open Questions

- Google Fonts 사용 vs self-hosted fonts — 성능 영향 확인 필요. next/font 사용 예정.
- 다크모드 전용 vs 라이트모드 옵션 — Refined Technical 방향이므로 다크모드 단일이지만, "AI 슬롭" 다크모드가 아닌 의도적인 다크 팔레트로 재구성
