# M005: Monad Ecosystem UX Research — Research

**Date:** 2026-03-24

## Summary

Monad 생태계는 2025년 11월 메인넷 런칭 이후 급속히 성장하고 있다. Paradigm 주도의 $244M 자금과 함께 300+ 프로젝트가 생태계 디렉토리에 등록되었고, Uniswap·Curve·Aave 등 주요 DeFi 프로토콜이 이미 배포되었다. 그러나 **개발자 도구 영역에서 Monad의 병렬 실행 특성을 활용하는 전용 웹 IDE는 존재하지 않는다**. Remix·Hardhat·Foundry 같은 범용 EVM 도구만 사용되고 있으며, Tenderly가 가장 가까운 경쟁자이지만 Enterprise급 가격과 범용 EVM 포커스로 Vibe-Loom의 틈새가 명확하다.

Vibe-Loom/Vibe-Room의 가장 강력한 차별화는 **monad-core Rust 엔진 기반 실제 병렬 실행 시뮬레이션**이다. 이 기능은 현재 시장에 존재하지 않는다. 단, 이 가치를 사용자에게 전달하려면 현재의 MVP 수준 UX를 대폭 개선해야 한다. 온보딩 플로우 부재, 제한된 컨트랙트 템플릿(4개 하드코딩), 사용자 워크스페이스 미지원, 배포 후 모니터링 부재가 주요 마찰점이다.

**핵심 권고**: 병렬 실행 최적화 가이드(실행 가능한 제안)를 중심으로 킬러 기능을 강화하고, 온보딩·템플릿·워크스페이스를 추가하여 "처음 방문 → 배포 → 반복 사용"까지의 사용자 루프를 완성하라. 재단 그랜츠 신청에서 NINE FORK 준수 + 병렬 실행 최적화는 핵심 어필 포인트다.

## Recommendation

### 전략: "Monad 병렬 실행 전문 IDE"로 포지셔닝 강화

1. **먼저 증명할 것**: 병렬 실행 최적화 제안 기능 (Vibe Score → 구체적 코드 수정 제안)
2. **그 다음**: 온보딩 + 컨트랙트 템플릿 갤러리로 신규 사용자 유입
3. **마지막**: 커뮤니티 기능 (공유, 포크, 리더보드)으로 리텐션

이 순서는 다음 논리를 따른다:
- 병렬 실행 최적화는 다른 도구가 제공하지 못하는 유일한 가치 → 재단 그랜츠 + 개발자 관심 확보
- 온보딩/템플릿은 유입 funnel 완성 → MAU 증가
- 커뮤니티 기능은 네트워크 효과 → 장기 성장

## Implementation Landscape

### Monad 생태계 현황

**메인넷 (2025-11-24 런칭)**:
- 10,000 TPS, 400ms 블록, 800ms 파이널리티
- Chain ID: 143 (메인넷), 10143 (테스트넷)
- MONAD_NINE 업그레이드 (2026년 2월) — 효율성/예측성/Ethereum 호환성 개선
- Monad AI Blueprint — AI+블록체인 융합 지원 프로그램

**개발자 도구 현황**:
| 도구 | 용도 | Monad 지원 | 웹 IDE? |
|------|------|-----------|---------|
| Monad Foundry | 컴파일/테스트/배포 (Foundry 커스텀 포크) | ✅ 네이티브 | ❌ CLI |
| Hardhat | 컴파일/테스트/배포 | ✅ RPC 설정만 | ❌ CLI |
| Remix IDE | 웹 기반 Solidity IDE | ⚠️ 범용 EVM | ✅ |
| Tenderly | 디버깅/시뮬레이션/모니터링 | ✅ Day-1 통합 | ✅ (대시보드) |
| Cookbook.dev | 템플릿 + AI 채팅 + 원클릭 배포 | ⚠️ 범용 EVM | ✅ |
| MonadVision / Monadscan | 블록 탐색기 | ✅ | ✅ |

**핵심 인사이트**: 모나드 개발자는 현재 Foundry(CLI) 또는 Remix(웹)를 주로 사용한다. Monad Foundry가 공식 추천 도구이나, CLI 기반이라 진입 장벽이 있다. 웹 기반이면서 Monad 병렬 실행 특화 기능을 제공하는 도구는 **없다**.

### 경쟁 분석

**직접 경쟁자: 없음** — Monad 전용 웹 IDE + 병렬 실행 시뮬레이션은 시장에 존재하지 않음

**간접 경쟁자**:

| 경쟁자 | 강점 | 약점 (vs Vibe-Loom) |
|--------|------|---------------------|
| **Remix IDE** | 범용 EVM IDE, 낮은 진입장벽, 플러그인 생태계 | Monad 병렬 실행 무시, 보안 분석 없음, 배포 후 모니터링 없음 |
| **Tenderly** | 트랜잭션 시뮬레이션, 디버깅, 모니터링, Monad Day-1 지원 | Enterprise 가격, 병렬 실행 최적화 점수 없음, IDE 아님 |
| **Cookbook.dev** | AI 채팅, 원클릭 배포, 템플릿 | 범용 EVM, 보안 분석 없음, 병렬 실행 인사이트 없음 |
| **Monad Foundry** | Monad 네이티브 EVM, 스테이킹 프리컴파일, 트레이스 디코딩 | CLI 전용, AI 없음, 웹 UI 없음 |

**Vibe-Loom 차별화 요소**:
1. 🎯 **실제 EVM 병렬 실행 시뮬레이션** — Block-STM OCC 기반, 다른 도구에 없는 유일한 기능
2. 🤖 **AI 에러 분석 + 자동 수정 루프** — monad-docs RAG 컨텍스트 + 실행 트레이스
3. 💰 **무료 배포 (Paymaster)** — 3회 무료로 진입장벽 제거
4. 🏗️ **NINE FORK 준수** — MIP-3/4/5 프리컴파일 지원

### Pain Points 발견

1. **Monad 병렬 실행 최적화 가이드 부재**: 개발자들이 "왜 내 컨트랙트에서 충돌이 발생하는지" 이해할 도구가 없음. Vibe Score가 이 gap을 정확히 메운다.
2. **테스트넷 faucet 접근성**: Discord Full Access 역할 필요, 12시간 대기, 0.05 MON만 지급. 개발자에게 frustrating.
3. **RPC 불안정**: 테스트넷 공용 RPC가 혼잡. 개발자들이 대안 RPC를 직접 찾아야 함.
4. **배포→검증→모니터링 파편화**: Foundry로 배포, Sourcify/Monadscan으로 검증, Tenderly로 모니터링 — 3개 도구를 오가야 함. Vibe-Loom이 통합할 기회.
5. **NINE FORK 프리컴파일 학습 곡선**: MIP-3 (메모리 최적화), MIP-4 (reserve 가스), MIP-5 (CLZ 옵코드)를 이해하고 활용하는 교육 자료 부족.

### 킬러 기능 후보 매트릭스

| 기능 | 사용자 가치 | 구현 복잡도 | 차별화 | 재단 어필 | 우선순위 |
|------|------------|-----------|--------|----------|---------|
| **병렬 실행 최적화 제안** | ⭐⭐⭐⭐⭐ | 중 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | **P0** |
| **컨트랙트 템플릿 갤러리** | ⭐⭐⭐⭐ | 저 | ⭐⭐⭐ | ⭐⭐⭐ | **P1** |
| **온보딩 가이드 투어** | ⭐⭐⭐⭐ | 저 | ⭐⭐ | ⭐⭐ | **P1** |
| **배포 후 컨트랙트 모니터링** | ⭐⭐⭐⭐ | 고 | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | **P2** |
| **원클릭 컨트랙트 검증** | ⭐⭐⭐ | 중 | ⭐⭐ | ⭐⭐ | **P2** |
| **가스 최적화 제안 (via_ir)** | ⭐⭐⭐ | 중 | ⭐⭐⭐ | ⭐⭐⭐ | **P2** |
| **멀티 컨트랙트 워크스페이스** | ⭐⭐⭐ | 고 | ⭐⭐ | ⭐⭐ | **P3** |
| **커뮤니티 공유/포크** | ⭐⭐⭐ | 고 | ⭐⭐⭐ | ⭐⭐ | **P3** |
| **Foundry 프로젝트 임포트** | ⭐⭐⭐ | 중 | ⭐⭐⭐ | ⭐⭐ | **P3** |
| **실시간 Monad 블록 리플레이 대시보드** | ⭐⭐⭐⭐ | 고 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | **P3** |

### Key Files (현재 코드베이스)

**monad-core (Rust 엔진)**:
- `crates/cli/src/main.rs` — JSON stdin/stdout CLI 브릿지. NestJS에서 subprocess로 호출. 현재 병렬 실행 결과만 반환하며, **최적화 제안 로직 추가 위치**
- `crates/scheduler/src/parallel_executor.rs` — Block-STM 병렬 실행기. `ParallelExecutionResult.incarnations`로 충돌/재실행 추적
- `crates/mv-state/src/read_write_sets.rs` — Read/Write Set 추적. 충돌 원인 분석의 핵심 데이터
- `crates/nine-fork/` — NINE FORK (MIP-3/4/5) 프리컴파일 구현

**Vibe-Loom (Next.js 프론트엔드)**:
- `src/lib/api-client.ts` — 5개 API 엔드포인트 (contract-source, compile, deploy, vibe-score, analysis/error). **템플릿 API 추가 필요**
- `src/app/page.tsx` — 메인 페이지, 394줄 단일 파일. CONTRACT_OPTIONS 4개 하드코딩. **리팩토링 대상**
- `src/components/ide/VibeScoreDashboard.tsx` — Vibe Score 시각화. 현재 점수+통계만 표시. **최적화 제안 카드 추가 위치**
- `src/components/ide/IDELayout.tsx` — 3패널 레이아웃 (데스크톱) + 탭 (모바일). 온보딩 오버레이 추가 가능
- `src/lib/auth-context.tsx` — GitHub OAuth 인증. 기본 구조 완성

### UX 개선 기회

1. **온보딩 부재**: 첫 방문 사용자를 위한 가이드 투어 없음. "이 도구가 뭘 하는지" 설명이 없어 이탈 위험
2. **컨트랙트 템플릿 제한**: 4개 하드코딩 (FailingContract, FixedContract, PectraTest, ParallelConflict). Monad 생태계에서 실제 필요한 템플릿 (ERC20, ERC721, DEX, Staking, etc.) 없음
3. **사용자 워크스페이스**: 작성한 코드를 저장/불러오기 불가. 세션 간 데이터 유실
4. **Vibe Score 액션 부재**: 점수만 보여주고 "어떻게 개선하는지" 구체적 행동을 안내하지 않음. "병렬 실행에서 storage slot 0x3a에서 충돌 → mapping을 분리하세요" 수준의 실행 가능한 제안 필요
5. **배포 피드백 루프**: 배포 완료 후 "다음 무엇을 할지" 안내 없음. 컨트랙트 검증, 모니터링, 인터랙션으로 연결해야 함
6. **에러 메시지 품질**: API 에러 시 기술적 메시지만 표시. 사용자 친화적 안내 필요

### Build Order

1. **Phase 1 — 핵심 차별화 강화**: 병렬 실행 최적화 제안 기능
   - CLI에서 read/write set 분석 데이터 추가 반환
   - NestJS에서 충돌 원인 분석 로직 추가
   - 프론트엔드 VibeScoreDashboard에 "actionable suggestions" 카드 추가
   - **왜 먼저?** 이것이 Vibe-Loom의 유일한 차별화. 다른 모든 기능은 Remix/Tenderly가 할 수 있지만, 이것은 할 수 없다

2. **Phase 2 — 사용자 유입 최적화**: 온보딩 + 템플릿
   - 첫 방문 온보딩 투어 (5단계: 편집→컴파일→배포→분석→최적화)
   - Monad 생태계 맞춤 컨트랙트 템플릿 갤러리 (10-15개)
   - **왜 두 번째?** 차별화 기능이 있어야 유입된 사용자가 남음

3. **Phase 3 — 리텐션 + 생태계 확장**: 모니터링, 커뮤니티, 고급 기능
   - 배포 후 컨트랙트 모니터링 (이벤트, 트랜잭션 추적)
   - 컨트랙트 공유/포크 소셜 기능
   - 실시간 Monad 블록 리플레이 대시보드

### Verification Approach

- **Phase 1 검증**: Vibe Score API가 충돌 원인 + 구체적 코드 수정 제안을 반환하는지 E2E 테스트
- **Phase 2 검증**: 새 사용자가 5분 이내에 온보딩→첫 배포 완료하는 시나리오 Playwright 테스트
- **Phase 3 검증**: 배포된 컨트랙트의 실시간 이벤트가 대시보드에 1초 이내 표시되는지 확인

## Don't Hand-Roll

| Problem | Existing Solution | Why Use It |
|---------|------------------|------------|
| 온보딩 투어 UI | react-joyride 또는 driver.js | 검증된 step-by-step 가이드 라이브러리. 커스텀 구현 대비 접근성·모바일 대응 우수 |
| 컨트랙트 템플릿 관리 | OpenZeppelin Contracts Wizard API | 표준 ERC20/ERC721/ERC1155 템플릿을 커스터마이즈 가능. 보안 검증된 코드 |
| 가스 최적화 분석 | Foundry의 `forge snapshot` 패턴 | via_ir 최적화 효과를 CLI에서 비교 가능 |
| 실시간 이벤트 모니터링 | Envio HyperIndex / Goldsky Streams | Monad 네이티브 인덱서. 직접 구현보다 안정적 |
| 컨트랙트 검증 | Sourcify API / Monadscan API | 표준화된 검증 프로토콜. 직접 구현 불필요 |

## Constraints

- **monad-core는 별도 레포**: Core 레포에 프론트엔드/백엔드 코드가 들어가면 안 됨 (KNOWLEDGE.md 규칙)
- **CLI subprocess 연동**: Rust 엔진은 JSON stdin/stdout으로만 통신. NAPI/FFI 아님 (D002 결정)
- **3회 무료 배포 모델**: Paymaster 구조 변경은 운영 모델에 직접 영향 (D004 결정)
- **Monad 테스트넷 불안정성**: 배포 테스트는 30-90초 소요, 타임아웃 빈번 (KNOWLEDGE.md). 모든 네트워크 의존 기능에 방어적 설계 필수
- **NINE FORK 준수 필수**: MIP-3/4/5 프리컴파일은 monad-core에 이미 구현. 새 기능이 이를 위반하면 안 됨

## Common Pitfalls

- **"점수만 보여주기" 함정** — Vibe Score가 숫자만 반환하면 사용자에게 가치 없음. 반드시 "왜 이 점수인지" + "어떻게 개선하는지" 실행 가능한 제안이 함께 와야 함. 현재 `suggestions` 필드가 있지만 내용이 generic함
- **템플릿 과다 추가** — 100개 템플릿보다 10개 잘 큐레이션된 Monad 최적화 템플릿이 낫다. 각 템플릿에 Vibe Score + 최적화 설명이 포함되어야 차별화
- **테스트넷 의존 데모** — 테스트넷이 불안정할 때 데모가 실패하면 치명적. 로컬 시뮬레이션 모드(monad-core 엔진 직접 실행) 폴백 필요
- **EVM 호환성 과신** — Monad는 EVM 호환이지만 병렬 실행 환경에서 state 접근 패턴이 다를 수 있음. 이 차이를 문서화하고 에러 분석에 반영해야 함
- **커뮤니티 기능 조급증** — 사용자 기반 없이 소셜 기능을 만들면 텅 빈 플랫폼. Phase 1-2가 사용자를 확보한 후에 Phase 3 진행

## Open Risks

- **재단 그랜츠 승인 불확실**: Monad Foundation의 그랜츠 프로그램 구체적 조건 미확인. AI Blueprint 프로그램이 Vibe-Room에 맞는지 추가 조사 필요
- **Monad Foundry와의 경쟁/협력**: Monad 공식 Foundry 포크가 점점 강력해지면 CLI 사용자를 빼앗길 수 있음. 반대로 Foundry 프로젝트 임포트 기능으로 시너지 가능
- **Tenderly Monad 지원 확대**: Tenderly가 병렬 실행 디버깅을 추가하면 Vibe-Loom의 차별화가 약화됨. 선점이 중요
- **테스트넷→메인넷 전환**: 현재 Vibe-Loom은 테스트넷 대상. 메인넷 배포는 실제 MON 비용 발생 → Paymaster 경제 모델 재검토 필요
- **병렬 실행 최적화의 실용성**: 대부분의 단순 컨트랙트는 충돌이 없을 수 있음. 복잡한 DeFi 프로토콜 타겟팅이 필요할 수 있으나, 이는 초보자 타겟과 충돌

## Candidate Requirements (리서치 기반 제안)

> ⚠️ 아래는 리서치에서 도출된 **후보 요구사항**이다. 자동으로 스코프에 포함되지 않으며, 로드맵 플래너가 평가 후 결정한다.

### 높은 우선순위 후보

| 후보 | 설명 | 근거 |
|------|------|------|
| **CR-01: 병렬 실행 최적화 제안** | Vibe Score에 충돌 원인 분석 + 구체적 코드 수정 제안 추가 | 유일한 차별화 기능. 현재는 숫자만 반환 |
| **CR-02: 컨트랙트 템플릿 갤러리** | Monad 최적화 10-15개 표준 템플릿 (ERC20, ERC721, DEX 등) + 각 Vibe Score | 현재 4개 하드코딩. 신규 사용자 유입 bottleneck |
| **CR-03: 온보딩 가이드 투어** | 첫 방문 시 5단계 가이드 (편집→컴파일→배포→분석→최적화) | 현재 "이 도구가 뭘 하는지" 설명 없음 |

### 중간 우선순위 후보

| 후보 | 설명 | 근거 |
|------|------|------|
| **CR-04: 사용자 워크스페이스** | 컨트랙트 코드 저장/불러오기 (PostgreSQL 백엔드) | 세션 간 데이터 유실 방지 |
| **CR-05: 원클릭 컨트랙트 검증** | Sourcify/Monadscan API 연동 자동 검증 | 배포→검증 플로우 통합 |
| **CR-06: Monad 가스 최적화 제안** | via_ir, Cancun EVM 설정 등 Monad 특화 최적화 가이드 | Monad Foundry Starter Kit의 최적화 설정을 웹 UI로 |

### 낮은 우선순위 후보

| 후보 | 설명 | 근거 |
|------|------|------|
| **CR-07: 배포 후 컨트랙트 모니터링** | 이벤트/트랜잭션 실시간 대시보드 | Tenderly 대비 차별화 어려움, 높은 구현 복잡도 |
| **CR-08: 커뮤니티 공유/포크** | 컨트랙트 공유, 다른 사용자 코드 포크 | 사용자 기반 확보 후 의미 있음 |
| **CR-09: Monad 블록 리플레이 대시보드** | 실제 메인넷 블록을 병렬 실행 시뮬레이션 + 시각화 | R015(deferred)와 연관. 재단 어필 최상위이나 구현 복잡 |

### 기존 요구사항 관찰

- **R006 (Vibe Score)**: 현재 "active"이지만 제안(suggestions)이 generic함. CR-01로 구체화 필요
- **R011 (엔진 트레이스 기반 실패 분석)**: active이나 프론트엔드에서 트레이스 시각화 미구현. 높은 가치
- **R015 (실 체인 블록 리플레이)**: deferred이나 재단 그랜츠 어필을 위해 재평가 가치 있음
- **R016 (UI 재디자인)**: M004에서 진행 중. M005 리서치 결과가 M004의 디자인 방향에 input 제공

## Monad 커뮤니티 트렌드

- **Monad Devs Twitter**: 106.8K 팔로워, 높은 개발자 커뮤니티 engagement
- **Monad Blitz 해커톤**: 전 세계에서 1일 해커톤 개최 (인도, 중국 등). 빌더 커뮤니티 활발
- **Monad AI Blueprint**: AI+블록체인 융합 프로그램. Vibe-Room의 AI 에러 분석이 이 방향에 정확히 맞음
- **Aave DAO Monad 배포 (2026-03-05)**: 주요 DeFi 프로토콜 확장 → 복잡한 컨트랙트 배포 수요 증가 → 병렬 실행 최적화 니즈 증가
- **테스트넷 3.9M+ 월렛**: 매우 높은 개발자/사용자 관심

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| Next.js / React | react-best-practices | installed (available_skills) |
| Frontend Design | frontend-design | installed (available_skills) |
| Code Optimization | code-optimizer | installed (available_skills) |
| Accessibility | accessibility | installed (available_skills) |
| Solidity / Smart Contracts | — | none found (domain-specific) |
| Monad / EVM | — | none found (too niche) |

## Sources

- Monad 메인넷 2025-11-24 런칭, 10K TPS, 400ms 블록 (source: [Monad Developer Docs](https://docs.monad.xyz/))
- Monad Foundry — Foundry 커스텀 포크 with Monad-native EVM (source: [Monad Deploy Guide](https://docs.monad.xyz/guides/deploy-smart-contract/foundry))
- Tenderly Monad Day-1 통합 — Virtual TestNets, 디버깅, 모니터링 (source: [Tenderly Blog](https://blog.tenderly.co/tenderly-supports-monad-from-day-one/))
- 300+ 프로젝트 에코시스템 디렉토리 (source: [Backpack Monad Ecosystem](https://learn.backpack.exchange/articles/monad-ecosystem))
- 테스트넷 3.9M 월렛, 5-6K 컨트랙트 배포/시간 (source: [Mitosis Monad Analysis](https://university.mitosis.org/monad-testnet-analysis/))
- MONAD_NINE 업그레이드 Feb 2026 (source: [CoinMarketCap Monad Updates](https://coinmarketcap.com/cmc-ai/monad/latest-updates/))
- Monad AI Blueprint 프로그램 2025-11-18 발표 (source: [CoinMarketCap Monad Updates](https://coinmarketcap.com/cmc-ai/monad/latest-updates/))
- Aave DAO Monad 배포 투표 2026-03-05 (source: [CoinMarketCap Monad Updates](https://coinmarketcap.com/cmc-ai/monad/latest-updates/))
- Smart contract 보안 도구 2026 현황 — Hardhat/Foundry 리드, Tenderly+Defender 모니터링 (source: [Nadcab Smart Contract Tools](https://www.nadcab.com/blog/smart-contracts-security-tools))
- Monad Foundry Starter Kit — via_ir 최적화, Cancun EVM 설정 (source: [GitHub monad-foundry-starter](https://github.com/obinnafranklinduru/monad-foundry-starter))
