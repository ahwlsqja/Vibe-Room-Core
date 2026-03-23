# M006: Parallel Execution Optimization Suggestions — Vibe Score를 "점수"에서 "처방전"으로

**Gathered:** 2026-03-24
**Status:** Ready for planning

## Project Description

Vibe Score가 현재 숫자(73점)와 generic suggestion("Consider using per-address storage patterns")만 반환한다. M005 리서치에서 이를 "점수만 보여주기" 함정으로 진단. 개발자는 "그래서 뭘 어쩌라고?"에 대한 답이 필요하다.

M006은 3개 레포(monad-core, Vibe-Room-Backend, Vibe-Loom)를 걸쳐 Vibe Score 파이프라인을 확장한다:
1. Rust CLI가 R/W set 충돌 데이터를 JSON 출력에 추가
2. NestJS가 solc storage layout으로 slot→변수명 디코딩 + 구체적 코드 수정 제안 생성
3. Vibe-Loom이 함수×변수명 매트릭스 히트맵과 구조화된 suggestion 카드로 시각화

## Why This Milestone

- M005 리서치 핵심 발견: "점수만 보여주기"는 함정. Vibe Score의 차별화를 완성하려면 "왜 이 점수인지" + "어떻게 고치는지"까지 반환해야 한다.
- 직접 경쟁자 없음 (D022): Remix/Tenderly/Cookbook.dev 모두 범용 EVM. 모나드 병렬 실행 최적화 제안은 시장에 없는 기능.
- 재단 그랜츠 핵심 어필 포인트 (D023): NINE FORK 준수 + 병렬 실행 최적화.
- 3-Phase 순서 (D021): Phase 1 차별화 → Phase 2 유입 → Phase 3 리텐션. 이 기능이 먼저 있어야 유입된 사용자가 남는다.

## User-Visible Outcome

### When this milestone is complete, the user can:

- Vibe-Loom에서 컨트랙트를 분석하면, 어떤 **변수**(예: `balances` mapping)에서 어떤 **함수들**(예: `transfer()`와 `approve()`) 사이에 충돌이 발생하는지 **매트릭스 히트맵**으로 볼 수 있다
- 각 충돌에 대해 **구체적 코드 수정 제안**(예: "balances mapping을 sender/receiver별로 분리하라")을 받을 수 있다
- 기존 Vibe Score 기능(점수 게이지, 충돌/재실행/가스효율 stat)이 그대로 동작하면서 추가 정보가 표시된다

### Entry point / environment

- Entry point: https://vibe-loom.xyz — Vibe Score 분석 버튼
- Environment: browser (production)
- Live dependencies: NestJS 백엔드 (Railway) → monad-core CLI (subprocess) → Vibe-Loom (Vercel)

## Completion Class

- Contract complete means: Rust CLI가 충돌 데이터를 JSON으로 반환하는 것이 단위 테스트로 검증됨. NestJS storage layout 디코딩이 테스트로 검증됨.
- Integration complete means: NestJS가 CLI 충돌 데이터를 받아 변수명으로 디코딩하고 suggestion을 생성해서 API로 반환하는 전체 파이프라인이 동작.
- Operational complete means: Vibe-Loom 프론트엔드에서 실제 컨트랙트로 히트맵과 suggestion 카드가 렌더링됨.

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- ParallelConflict 테스트 컨트랙트로 전체 파이프라인(컴파일→블록구성→엔진실행→충돌분석→변수명디코딩→히트맵렌더링)이 동작
- 충돌 없는 컨트랙트(FixedContract 등)에서도 깨지지 않고 "충돌 없음" 상태를 정상 표시
- 기존 Vibe Score 기능(점수, stat grid, generic suggestion)이 하위 호환

## Risks and Unknowns

- **ReadSet 보존 경로 복잡도** — `parallel_executor.rs`의 `collect_results()`가 현재 `(ExecutionResult, WriteSet)`만 반환. ReadSet은 coordinator에서 validation 후 폐기됨. ReadSet도 보존하려면 scheduler/coordinator 수정 필요.
- **Storage layout 디코딩 정확도** — solc `storageLayout` 출력이 mapping/dynamic array의 base slot만 제공. `keccak256(key, baseSlot)` 역매핑은 runtime slot → base slot 매칭 로직이 필요하고, 정확도가 100%는 아닐 수 있음. proxy 패턴이나 assembly 사용 시 layout이 불완전할 수 있음.
- **3개 레포 동시 수정** — Core(Rust), Backend(NestJS), Frontend(Next.js) 모두 수정해야 하며, KNOWLEDGE.md 규칙에 따라 Core 레포에 frontend/backend 코드가 들어가면 안 됨. 각 레포 별도 커밋/푸시.
- **CLI 출력 크기 증가** — R/W set 데이터가 추가되면 JSON 페이로드가 커짐. 200 tx 블록이면 상당한 크기. 필요시 요약만 출력하는 옵션 고려.

## Existing Codebase / Prior Art

- `crates/mv-state/src/read_write_sets.rs` (443줄) — ReadSet/WriteSet 구현. `LocationKey` (Storage/Balance/Nonce/CodeHash), `ReadOrigin`, `WriteValue` 타입 정의. `.iter()` 메서드로 순회 가능.
- `crates/mv-state/src/types.rs` — `LocationKey`, `WriteValue`, `ReadOrigin`, `MvReadResult` 등 핵심 타입.
- `crates/scheduler/src/parallel_executor.rs` — `execute_block_parallel()` → `ParallelExecutionResult { tx_results, beneficiary_tracker, incarnations }`. ReadSet은 현재 coordinator 내부에서 소비되고 외부로 노출 안 됨.
- `crates/scheduler/src/coordinator.rs` — `finish_execution()`이 ReadSet 저장, `take_read_set()`이 validation 시 꺼냄. validation 후 ReadSet은 폐기됨.
- `crates/cli/src/main.rs` (245줄) — JSON stdin→parallel execution→JSON stdout. `CliOutput { results, incarnations, stats }`.
- `Vibe-Room-Backend/src/engine/engine.service.ts` — CLI subprocess 호출. `CliOutput` 인터페이스.
- `Vibe-Room-Backend/src/vibe-score/vibe-score.service.ts` — 컴파일→블록구성→엔진→점수계산 파이프라인. `calculateScore()`가 generic suggestion 생성.
- `Vibe-Room-Backend/src/contracts/compile.service.ts` — Hardhat 기반 컴파일. storage layout 추출은 현재 미구현이나 solc 옵션 추가로 가능.
- `Vibe-Loom/src/components/ide/VibeScoreDashboard.tsx` — 원형 게이지 + stat grid + suggestion 카드. `suggestions: string[]`로 받음.
- `Vibe-Loom/src/lib/api-client.ts` — `VibeScoreResult` 인터페이스. `getVibeScore()` API 호출.

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R017 — 병렬 실행 최적화 제안: Vibe Score에 충돌 원인 분석(storage slot→변수명 수준) + 구체적 코드 수정 제안 추가
- R018 — R/W Set 충돌 시각화: 함수×변수명 매트릭스 히트맵으로 충돌 표시
- R006 — Vibe Score 강화: generic suggestion → actionable suggestion (R017이 이를 구현)

## Scope

### In Scope

- monad-core CLI JSON 출력에 R/W set 충돌 데이터 추가 (`conflict_details` 필드)
- ReadSet을 parallel execution 결과에 보존하도록 scheduler/coordinator 수정
- NestJS CompileService에서 solc `storageLayout` 추출
- NestJS에서 slot→변수명/mapping 디코딩 로직 (일반 변수, mapping base slot, dynamic array base slot)
- NestJS VibeScoreService에서 충돌 데이터 → actionable suggestion 변환
- VibeScoreResultDto 확장 (구조화된 충돌 정보 + actionable suggestions)
- Vibe-Loom VibeScoreDashboard에 함수×변수명 매트릭스 히트맵 추가
- Suggestion 카드를 단순 문자열에서 구조화된 카드로 업그레이드
- 기존 기능 하위 호환 유지
- Rust 단위 테스트 + E2E 파이프라인 검증

### Out of Scope / Non-Goals

- Solidity AST 분석을 통한 정확한 변수→코드 라인 매핑 (복잡도 높음, Phase 2+)
- proxy 패턴의 storage layout 분석 (delegatecall 기반은 layout이 다름)
- CLI 출력 크기 최적화 (gzip 등) — 현재 규모에서 불필요
- 프론트엔드 디자인 리뉴얼 — M004 디자인 토큰 시스템 내에서 작업

## Technical Constraints

- Core 레포(monad-core)에는 Rust crates + Cargo.toml만 들어감. Frontend/Backend 코드 절대 포함 금지 (KNOWLEDGE.md 규칙)
- Vibe-Loom은 M004 디자인 토큰 시스템(oklch, 3-tier surface) 준수
- NestJS 백엔드는 Railway 배포 — ENGINE_BINARY_PATH 환경변수로 CLI 바이너리 경로 설정
- solc storage layout은 `--storage-layout` 플래그 또는 `outputSelection` 설정으로 추출

## Integration Points

- monad-core CLI ↔ NestJS EngineService — JSON stdin/stdout. CLI 출력 스키마에 `conflict_details` 추가 시 EngineService의 `CliOutput` 인터페이스 동기화 필요
- NestJS VibeScoreService ↔ CompileService — storage layout 데이터 전달 경로 추가
- NestJS API ↔ Vibe-Loom API client — `VibeScoreResult` 타입에 구조화된 충돌 데이터 추가

## Open Questions

- solc `storageLayout`에서 mapping의 base slot만 나오는데, runtime에서 실제 접근된 slot과 매칭하려면 `keccak256(key, baseSlot)` 역연산이 필요. key 값을 모르면 정확한 매칭이 안 될 수 있음 — base slot이 같은 범위에 있는지로 heuristic 매칭하는 방식이 현실적
- Vibe Score API 응답이 커지면 프론트엔드 로딩에 영향? — 현재 ParallelConflict 수준(5-10 tx)이면 KB 단위라 무시 가능
