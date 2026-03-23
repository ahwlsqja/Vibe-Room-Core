# S01: Formalize Research into Actionable Roadmap

**Goal:** M005-RESEARCH.md의 완성된 리서치 결과를 M005-ROADMAP.md, 신규 요구사항(R017-R025), 전략 결정(D021-D023), 기존 요구사항 업데이트로 정형화한다.
**Demo:** M005-ROADMAP.md에 3-phase 기능 우선순위 매트릭스와 M006/M007/M008 마일스톤 제안이 포함되어 있고, REQUIREMENTS.md에 R017-R025가 등록되어 있고, DECISIONS.md에 D021-D023이 기록되어 있다.

## Must-Haves

- M005-ROADMAP.md 완성 (3-phase 매트릭스, 다음 마일스톤 제안, 요구사항 매핑, Don't Hand-Roll 레지스트리, Open Risks)
- R017-R025 새 요구사항 REQUIREMENTS.md에 등록
- D021-D023 전략 결정 DECISIONS.md에 기록
- R006 notes 업데이트 (최적화 제안 추가 필요성 근거)

## Verification

- `test -f .gsd/milestones/M005/M005-ROADMAP.md && echo "ROADMAP exists"` → "ROADMAP exists"
- `grep -c "^### Phase" .gsd/milestones/M005/M005-ROADMAP.md` → 3 (Phase 1, 2, 3)
- `grep -c "^### R0" .gsd/REQUIREMENTS.md` → 25 이상 (기존 16 + 신규 9)
- `grep -c "^| D0" .gsd/DECISIONS.md` → 14 이상 (기존 11 + 신규 3)
- `grep "R017" .gsd/REQUIREMENTS.md` → 존재
- `grep "D021" .gsd/DECISIONS.md` → 존재

## Tasks

- [x] **T01: Write M005-ROADMAP.md** `est:20m`
  - Why: M005의 핵심 산출물. 리서치 결과를 구조화된 기능 우선순위 매트릭스 + 다음 마일스톤 제안으로 정리
  - Files: `.gsd/milestones/M005/M005-ROADMAP.md`
  - Do: M005-RESEARCH.md의 킬러 기능 후보 매트릭스, Build Order, 후보 요구사항을 3-phase 우선순위로 정리. 각 phase에 대한 다음 마일스톤 제안(M006/M007/M008). Don't Hand-Roll 레지스트리. Open Risks 캐리포워드.
  - Verify: `test -f .gsd/milestones/M005/M005-ROADMAP.md && grep -c "Phase" .gsd/milestones/M005/M005-ROADMAP.md`
  - Done when: M005-ROADMAP.md가 존재하고 3개 Phase 섹션이 포함됨

- [x] **T02: Register New Requirements and Decisions** `est:15m`
  - Why: 리서치에서 도출된 후보 요구사항과 전략 결정을 프로젝트 레지스트리에 공식 등록
  - Files: `.gsd/REQUIREMENTS.md`, `.gsd/DECISIONS.md`
  - Do: gsd_update_requirement로 R006 notes 업데이트. 9개 신규 요구사항(R017-R025) 등록. gsd_save_decision으로 D021-D023 등록. R016 status → validated.
  - Verify: `grep "R017" .gsd/REQUIREMENTS.md && grep "D021" .gsd/DECISIONS.md`
  - Done when: REQUIREMENTS.md에 R017-R025 존재, DECISIONS.md에 D021-D023 존재, R006 notes 업데이트됨

## Observability / Diagnostics

**Runtime signals:** This is a documentation-only slice (no runtime components). The primary observable artifacts are static files.

**Inspection surfaces:**
- `M005-ROADMAP.md` — verify section structure with `grep -c "^### Phase" M005-ROADMAP.md` (expect 3)
- `REQUIREMENTS.md` — verify requirement count with `grep -c "^### R0" REQUIREMENTS.md` (expect ≥25)
- `DECISIONS.md` — verify decision count with `grep -c "^| D0" DECISIONS.md` (expect ≥14)

**Failure visibility:** If any required section is missing from M005-ROADMAP.md, `grep` will return 0 or a count below threshold. If a requirement/decision registration fails, the `gsd_update_requirement`/`gsd_save_decision` tool will return an error.

**Redaction constraints:** None — all artifacts are project documentation with no secrets.

## Verification (diagnostic / failure-path)

- `grep -c "Open Risks" .gsd/milestones/M005/M005-ROADMAP.md` → ≥1 (verifies risk section not accidentally omitted)
- `grep -c "Don't Hand-Roll" .gsd/milestones/M005/M005-ROADMAP.md` → ≥1 (verifies registry carried forward)
- `grep "M005 리서치" .gsd/REQUIREMENTS.md | wc -l` → ≥2 (R006 + R015 notes updated with research references)
- `grep "validated" .gsd/REQUIREMENTS.md | grep "R016"` → exists (R016 status update verified)

## Files Likely Touched

- `.gsd/milestones/M005/M005-ROADMAP.md`
- `.gsd/REQUIREMENTS.md`
- `.gsd/DECISIONS.md`
