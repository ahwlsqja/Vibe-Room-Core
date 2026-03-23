---
id: M005
provides:
  - M005-RESEARCH.md — 모나드 생태계 현황, 경쟁 분석, pain point, 킬러 기능 후보, UX 개선 기회 종합 리서치 (237줄, 출처 포함)
  - M005-ROADMAP.md — 3-phase 기능 우선순위 매트릭스 + M006/M007/M008 마일스톤 제안 (슬라이스 수준 스코프)
  - 9 new requirements (R017-R025) registered in REQUIREMENTS.md with class, priority, proposed milestone owner
  - 3 strategic decisions (D021-D023) registered in DECISIONS.md
  - R006 notes enriched with "점수만 보여주기" gap analysis pointing to R017
  - R015 notes enriched with Phase 3 re-evaluation rationale and Envio/Goldsky recommendation
  - R016 status confirmed as validated (M004 evidence)
  - Don't Hand-Roll registry (5 entries) and Open Risks (5 items) documented for downstream milestones
key_decisions:
  - "D021: 기능 개발 3-Phase 순서 — Phase 1 차별화 → Phase 2 유입 → Phase 3 리텐션"
  - "D022: 제품 포지셔닝 — Monad 병렬 실행 전문 IDE"
  - "D023: 재단 그랜츠 어필 — NINE FORK 준수 + 병렬 실행 최적화"
patterns_established:
  - "Research-to-roadmap pipeline: CONTEXT(질문) → RESEARCH(데이터 수집) → ROADMAP(우선순위 매트릭스 + 마일스톤 제안) → REQUIREMENTS/DECISIONS(정식 등록). 리서치 마일스톤의 표준 산출물 체인."
  - "Batch registration with post-verification: 단일 write pass에서 ROADMAP + REQUIREMENTS + DECISIONS를 동시 작성하고, 별도 태스크에서 각 등록을 개별 검증. 중복 작업 방지 + 누락 검출."
  - "Don't Hand-Roll registry: 리서치에서 발견한 외부 라이브러리/API를 명시적 레지스트리로 관리하여 구현 단계에서 재발명 방지."
observability_surfaces:
  - "grep -c '^### Phase' M005-ROADMAP.md → 3 (phase count)"
  - "grep -c '^### R0' REQUIREMENTS.md → 25 (total requirements)"
  - "grep -c '^| D0' DECISIONS.md → 23 (total decision rows)"
  - "grep -c 'M005 리서치' REQUIREMENTS.md → 2 (R006 + R015 enrichment)"
requirement_outcomes:
  - id: R006
    from_status: active
    to_status: active
    proof: "Notes enriched with M005 리서치 findings — '점수만 보여주기' gap identified, R017 linkage added. Status unchanged; notes update only."
  - id: R015
    from_status: deferred
    to_status: deferred
    proof: "Notes enriched with M005 리서치 findings — Phase 3 재평가, Envio/Goldsky 활용 검토. Status unchanged; deferred maintained due to high complexity."
  - id: R016
    from_status: out_of_scope
    to_status: validated
    proof: "M004 completion evidence: 24 design tokens, 13 components migrated, 21/22 E2E + 57 unit tests. Status transition recorded during M005 S01."
  - id: R017
    from_status: new
    to_status: active
    proof: "Created from M005-RESEARCH.md CR-01 finding. Registered in REQUIREMENTS.md with class=differentiator, priority=P0, proposed owner=M006."
  - id: R018
    from_status: new
    to_status: active
    proof: "Created from M005-RESEARCH.md CR-01 extension. Registered in REQUIREMENTS.md with class=differentiator, priority=P0, proposed owner=M006."
  - id: R019
    from_status: new
    to_status: active
    proof: "Created from M005-RESEARCH.md CR-02 finding. Registered in REQUIREMENTS.md with class=primary-user-loop, priority=P1, proposed owner=M007."
  - id: R020
    from_status: new
    to_status: active
    proof: "Created from M005-RESEARCH.md CR-03 finding. Registered in REQUIREMENTS.md with class=primary-user-loop, priority=P1, proposed owner=M007."
  - id: R021
    from_status: new
    to_status: active
    proof: "Created from M005-RESEARCH.md CR-05 finding. Registered in REQUIREMENTS.md with class=integration, priority=P2, proposed owner=M007."
  - id: R022
    from_status: new
    to_status: active
    proof: "Created from M005-RESEARCH.md CR-06 finding. Registered in REQUIREMENTS.md with class=quality-attribute, priority=P2, proposed owner=M007."
  - id: R023
    from_status: new
    to_status: active
    proof: "Created from M005-RESEARCH.md CR-07 finding. Registered in REQUIREMENTS.md with class=differentiator, priority=P2, proposed owner=M008."
  - id: R024
    from_status: new
    to_status: active
    proof: "Created from M005-RESEARCH.md CR-04 finding. Registered in REQUIREMENTS.md with class=continuity, priority=P3, proposed owner=M008."
  - id: R025
    from_status: new
    to_status: active
    proof: "Created from M005-RESEARCH.md CR-08 finding. Registered in REQUIREMENTS.md with class=primary-user-loop, priority=P3, proposed owner=M008."
duration: 13m
verification_result: passed
completed_at: 2026-03-24
---

# M005: Monad Ecosystem UX Research — 사용자가 진짜 필요한 것 찾기

**Transformed Monad ecosystem research into a data-driven 3-phase product roadmap (M006 차별화 → M007 유입 → M008 리텐션), registering 9 new requirements (R017-R025), 3 strategic decisions (D021-D023), and enriching existing requirements (R006, R015, R016) with research-derived insights.**

## What Happened

M005 was a pure research milestone — no code was written. Its purpose was to answer six key questions about the Monad developer ecosystem and translate findings into actionable development priorities for Vibe-Loom/Vibe-Room.

**Research phase** (pre-S01) surveyed the Monad ecosystem landscape: 300+ projects post-mainnet (2025-11), Paradigm-led $244M funding, and a developer tooling gap — no web IDE exists that leverages Monad's parallel execution characteristics. Remix, Hardhat, Foundry, and Tenderly all serve generic EVM workflows. Cookbook.dev offers templates and AI chat but lacks parallel execution analysis. The research identified Vibe-Loom's monad-core Rust engine as a unique differentiator: real parallel execution simulation with Block-STM OCC is something no competing tool provides.

**S01 (Formalize Research into Actionable Roadmap)** took the raw research findings and structured them into project artifacts:

1. **M005-ROADMAP.md** — A 3-phase feature priority matrix organizing 9 candidate features by strategic sequencing:
   - **Phase 1 (M006)**: 병렬 실행 최적화 — P0 features R017 (actionable optimization suggestions) and R018 (R/W set conflict visualization). This is the core differentiator no competitor offers.
   - **Phase 2 (M007)**: 사용자 유입 — P1/P2 features R019-R022 (template gallery, onboarding tour, contract verification, gas optimization). Grows the user base to experience Phase 1 features.
   - **Phase 3 (M008+)**: 리텐션 — P2/P3 features R023-R025 + R015 re-evaluation (monitoring, workspace, community, block replay). Network effects require an established user base.

2. **9 new requirements (R017-R025)** registered in REQUIREMENTS.md with class, status, priority, description, and proposed milestone owner. Each traces back to a specific research finding (CR-01 through CR-08).

3. **3 strategic decisions (D021-D023)** registered in DECISIONS.md:
   - D021: 3-Phase development sequencing (차별화 → 유입 → 리텐션)
   - D022: Product positioning as "Monad 병렬 실행 전문 IDE"
   - D023: Foundation grants appeal via NINE FORK compliance + parallel optimization

4. **Existing requirement updates**: R006 notes enriched with the "점수만 보여주기" gap (needs actionable suggestions per R017); R015 notes enriched with Phase 3 re-evaluation rationale; R016 status confirmed as validated from M004.

5. **Don't Hand-Roll registry** (5 entries) and **Open Risks** (5 items) documented for downstream milestone planning.

## Cross-Slice Verification

M005 had a single slice (S01). All 5 success criteria from the milestone roadmap were verified:

| # | Success Criterion | Evidence | Result |
|---|-------------------|----------|--------|
| 1 | M005-RESEARCH.md에 6개 리서치 질문 모두 답변 + 출처 | 237줄, 11 sections covering all 6 questions. `## Implementation Landscape` (Q1+Q2), `## Common Pitfalls` (Q3), `## Candidate Requirements` (Q4), `## Recommendation` (Q5), `## Monad 커뮤니티 트렌드` (Q6), `## Sources` | ✅ |
| 2 | M005-ROADMAP.md에 3-phase 매트릭스 + 다음 마일스톤 제안 | `grep -c '^### Phase' → 3`, `grep -c '^### M00[678]' → 3` | ✅ |
| 3 | 리서치 도출 요구사항(R017+) 등록 | R017-R025 all found in REQUIREMENTS.md. `grep -c '^### R0' → 25` | ✅ |
| 4 | 전략적 결정(D021+) 기록 | D021-D023 all found in DECISIONS.md. `grep -c '^| D0' → 23` | ✅ |
| 5 | 기존 요구사항(R006, R015, R016) 업데이트 | R006+R015: `grep 'M005 리서치' REQUIREMENTS.md` → 2 matches. R016: `grep 'validated' → found` | ✅ |

**Definition of Done**: All 5 criteria met. Single slice S01 marked `[x]`. S01-SUMMARY.md exists with 14/14 internal verification checks passed. T01 and T02 both have passing VERIFY.json files.

## Requirement Changes

- **R006**: active → active (notes enriched) — M005 리서치 identified "점수만 보여주기" gap. Current generic suggestions insufficient; R017 linkage added for actionable code modification proposals.
- **R015**: deferred → deferred (notes enriched) — Highest foundation grant appeal value (⭐⭐⭐⭐⭐) but high implementation complexity. Phase 3 (M008) re-evaluation scheduled. Envio HyperIndex / Goldsky Streams recommended.
- **R016**: out_of_scope → validated — M004 delivered: 24 design tokens, 13 components migrated, 21/22 E2E + 57 unit tests.
- **R017-R025**: new → active — 9 research-derived requirements created with class, priority (P0-P3), and proposed milestone owners (M006/M007/M008).

## Forward Intelligence

### What the next milestone should know
- **M006 is the highest-priority next milestone** — Phase 1 (병렬 실행 최적화) validates the core differentiator hypothesis. The proposal in M005-ROADMAP.md names specific key files: `crates/cli/src/main.rs`, `crates/mv-state/src/read_write_sets.rs`, NestJS vibe-score module, `VibeScoreDashboard.tsx`. Estimated 3-4 slices, ~6-8h.
- **The Don't Hand-Roll registry must be consulted before implementation** — react-joyride (onboarding), OpenZeppelin Contracts Wizard (templates), Foundry forge snapshot (gas), Envio/Goldsky (monitoring), Sourcify API (verification). These prevent re-inventing existing solutions.
- **R017/R018 feasibility depends on monad-core's current R/W set exposure** — Check what `crates/mv-state/src/read_write_sets.rs` currently exposes (storage slot granularity, conflict attribution depth) before planning M006 slices.
- **All 9 new requirements (R017-R025) have "unmapped" validation** — Each needs concrete validation criteria when its owning milestone is created.

### What's fragile
- **Research data freshness** (~3-6 month shelf life) — Monad ecosystem evolving rapidly. Tenderly's Monad support expansion (Open Risk #3) could erode the "no competitor" positioning. If M006 planning is delayed past mid-2026, re-scan the competitive landscape.
- **Foundation grants assumptions** — D023 assumes Monad AI Blueprint alignment, but specific grant conditions are unconfirmed (Open Risk #1). Don't over-index development priorities on grant appeal without validation.
- **Phase sequencing rigidity** — D021 is explicitly marked revisable. If early user feedback strongly requests onboarding (Phase 2) before optimization features (Phase 1), be prepared to swap.

### Authoritative diagnostics
- `grep -c '^### R0' .gsd/REQUIREMENTS.md` → 25 — confirms total requirement count including M005 additions
- `grep -c '^| D0' .gsd/DECISIONS.md` → 23 — confirms total decision count including M005 additions
- M005-ROADMAP.md Phase sections are the source of truth for feature sequencing and milestone proposals
- M005-RESEARCH.md `## Sources` section is the reference for all research data provenance

### What assumptions changed
- **Original assumption:** Vibe-Loom's main gap was backend integration (M002 focus) → **Research revealed:** The real gap is translating Vibe Score numbers into actionable optimization suggestions. The score alone is a "number trap" — users need to know *why* and *how to fix*.
- **Original assumption:** Template gallery was a nice-to-have → **Research revealed:** Current 4 hardcoded templates are a significant new-user bottleneck. Expanding to 10-15 Monad-optimized templates is P1 priority.
- **Original assumption:** Community features would drive growth → **Research revealed:** Community features (R025) are meaningless without an established user base. They belong in Phase 3, not earlier.

## Files Created/Modified

- `.gsd/milestones/M005/M005-RESEARCH.md` — 237-line comprehensive ecosystem research with 11 sections covering all 6 research questions
- `.gsd/milestones/M005/M005-ROADMAP.md` — 3-phase feature priority matrix, M006/M007/M008 proposals, R017-R025 summary, D021-D023 summary, Don't Hand-Roll registry, Open Risks
- `.gsd/milestones/M005/M005-CONTEXT.md` — Research scope and questions definition
- `.gsd/milestones/M005/slices/S01/S01-SUMMARY.md` — Slice summary with 14/14 verification checks
- `.gsd/milestones/M005/slices/S01/tasks/T01-SUMMARY.md` — ROADMAP write and verification
- `.gsd/milestones/M005/slices/S01/tasks/T02-SUMMARY.md` — Requirements/decisions registration verification
- `.gsd/REQUIREMENTS.md` — R017-R025 added, R006/R015 notes enriched, R016 validated
- `.gsd/DECISIONS.md` — D021-D023 strategic decisions registered
- `.gsd/PROJECT.md` — Milestone sequence updated (M005 marked complete)
