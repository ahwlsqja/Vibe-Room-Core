---
estimated_steps: 4
estimated_files: 1
---

# T01: Write M005-ROADMAP.md

**Slice:** S01 — Formalize Research into Actionable Roadmap
**Milestone:** M005

## Description

M005-RESEARCH.md의 완성된 리서치 결과를 M005-ROADMAP.md로 정형화한다. 3-phase 기능 우선순위 매트릭스, 다음 마일스톤 제안(M006/M007/M008), 신규 요구사항 요약, 기존 요구사항 업데이트 내역, 전략 결정 요약, Don't Hand-Roll 레지스트리, Open Risks를 포함한다.

## Steps

1. M005-RESEARCH.md의 킬러 기능 후보 매트릭스와 Build Order 섹션을 3-phase 구조로 재편성
2. 각 Phase에 대한 구체적 다음 마일스톤 제안 작성 (M006: Phase 1, M007: Phase 2, M008: Phase 3)
3. 신규 요구사항(R017-R025) 요약 테이블, 기존 요구사항 변경사항, 전략 결정(D021-D023) 테이블 추가
4. Don't Hand-Roll 레지스트리와 Open Risks 캐리포워드

## Must-Haves

- [ ] 3-phase 기능 우선순위 매트릭스 (Phase 1: 차별화, Phase 2: 유입, Phase 3: 리텐션)
- [ ] M006/M007/M008 마일스톤 제안 (스코프, 커버 요구사항, 예상 슬라이스 수)
- [ ] 신규 요구사항 요약 (R017-R025)
- [ ] 전략 결정 요약 (D021-D023)

## Verification

- `test -f .gsd/milestones/M005/M005-ROADMAP.md` → 파일 존재
- `grep -c "Phase" .gsd/milestones/M005/M005-ROADMAP.md` → 3 이상

## Inputs

- `.gsd/milestones/M005/M005-RESEARCH.md` — 완성된 리서치 결과 (킬러 기능 매트릭스, Build Order, 후보 요구사항, Pain Points)
- `.gsd/milestones/M005/M005-CONTEXT.md` — 리서치 질문 및 스코프

## Expected Output

- `.gsd/milestones/M005/M005-ROADMAP.md` — 3-phase 기능 우선순위 매트릭스 + 다음 마일스톤 제안 + 요구사항/결정 요약

## Observability Impact

**Signals changed:** M005-ROADMAP.md creation makes the roadmap inspectable via `grep` for Phase sections, requirement IDs, and decision IDs.

**How to inspect:** `grep -c "^### Phase" M005-ROADMAP.md` should return 3. `grep "R017\|R025" M005-ROADMAP.md` should return matching lines. `grep "Don't Hand-Roll\|Open Risks" M005-ROADMAP.md` should each return ≥1 match.

**Failure state visible:** If the file is missing or incomplete, `test -f` fails and `grep -c` returns 0 or below threshold.
