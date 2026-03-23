---
estimated_steps: 5
estimated_files: 2
---

# T02: Register New Requirements and Decisions

**Slice:** S01 — Formalize Research into Actionable Roadmap
**Milestone:** M005

## Description

리서치에서 도출된 9개 신규 요구사항(R017-R025)을 REQUIREMENTS.md에 등록하고, 3개 전략 결정(D021-D023)을 DECISIONS.md에 기록하고, 기존 요구사항(R006, R016)의 notes/status를 업데이트한다.

## Steps

1. gsd_save_decision으로 D021 (3-Phase 순서), D022 (제품 포지셔닝), D023 (재단 그랜츠 어필 포인트) 등록
2. gsd_update_requirement로 R006 notes 업데이트 — "M005 리서치 결과: generic suggestions → 구체적 코드 수정 제안 필요 (R017로 구체화)"
3. gsd_update_requirement로 R016 status → validated (M004 완료 증거 참조)
4. R015 notes 업데이트 — "M005 리서치: 재단 그랜츠 어필 최상위 가치이나 구현 복잡도 높아 Phase 3 유지"
5. REQUIREMENTS.md에 R017-R025 신규 요구사항 추가 (수동 편집 — gsd tool이 개별 등록만 지원하므로)

## Must-Haves

- [ ] D021-D023 DECISIONS.md에 등록
- [ ] R006 notes 업데이트
- [ ] R016 status validated
- [ ] R017-R025 REQUIREMENTS.md에 등록

## Verification

- `grep "D021" .gsd/DECISIONS.md` → 존재
- `grep "D023" .gsd/DECISIONS.md` → 존재
- `grep "R017" .gsd/REQUIREMENTS.md` → 존재
- `grep "R025" .gsd/REQUIREMENTS.md` → 존재
- `grep -c "^### R0" .gsd/REQUIREMENTS.md` → 25

## Inputs

- `.gsd/milestones/M005/M005-ROADMAP.md` — T01에서 작성한 로드맵 (요구사항/결정 내용 참조)
- `.gsd/REQUIREMENTS.md` — 기존 요구사항 레지스트리 (R001-R016)
- `.gsd/DECISIONS.md` — 기존 결정 레지스트리 (D001-D011)

## Observability Impact

**What signals change:**
- `REQUIREMENTS.md` gains 9 new requirement headings (R017-R025), R006 notes updated with M005 research reference, R015 notes updated with Phase 3 rationale, R016 status changed to validated
- `DECISIONS.md` gains 3 new decision rows (D021-D023) in the decisions table

**How a future agent inspects this task:**
- `grep -c "^### R0" .gsd/REQUIREMENTS.md` — expect ≥25 (16 existing + 9 new)
- `grep -c "^| D0" .gsd/DECISIONS.md` — expect ≥14 (11 existing + 3 new)
- `grep "M005 리서치" .gsd/REQUIREMENTS.md` — expect matches in R006, R015 notes
- `grep "validated" .gsd/REQUIREMENTS.md` — expect R016 in results

**Failure state visibility:**
- If requirement registration fails, `grep -c "^### R0" .gsd/REQUIREMENTS.md` returns <25
- If decision registration fails, `grep -c "^| D0" .gsd/DECISIONS.md` returns <14
- Missing notes updates detectable via `grep "M005 리서치" .gsd/REQUIREMENTS.md` returning fewer matches than expected

## Expected Output

- `.gsd/REQUIREMENTS.md` — R017-R025 추가, R006/R015 notes 업데이트, R016 status validated
- `.gsd/DECISIONS.md` — D021-D023 추가
