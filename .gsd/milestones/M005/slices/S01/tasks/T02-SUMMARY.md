---
id: T02
parent: S01
milestone: M005
provides:
  - D021-D023 strategic decisions registered in DECISIONS.md
  - R017-R025 new requirements registered in REQUIREMENTS.md
  - R006 notes updated with M005 research findings (generic→actionable suggestions)
  - R015 notes updated with Phase 3 rationale and foundation grants appeal value
  - R016 status changed to validated (M004 completion evidence)
key_files:
  - .gsd/REQUIREMENTS.md
  - .gsd/DECISIONS.md
  - .gsd/milestones/M005/slices/S01/tasks/T02-PLAN.md (observability impact added)
  - .gsd/milestones/M005/slices/S01/S01-PLAN.md (diagnostic checks added, T02 marked done)
key_decisions: []
patterns_established:
  - Batch requirement/decision registration pattern: when T01 creates ROADMAP with full content tables, T02 verifies GSD registries already contain the registrations and applies observability gap fixes
observability_surfaces:
  - "grep -c '^### R0' .gsd/REQUIREMENTS.md → 25 (requirement count)"
  - "grep -c '^| D0' .gsd/DECISIONS.md → 23 (decision row count)"
  - "grep 'M005 리서치' .gsd/REQUIREMENTS.md | wc -l → 2 (R006 + R015 research notes)"
  - "grep 'validated' .gsd/REQUIREMENTS.md | grep R016 → exists"
duration: 5m
verification_result: passed
completed_at: 2026-03-24
blocker_discovered: false
---

# T02: Register New Requirements and Decisions

**Verified D021-D023 decisions, R017-R025 requirements, R006/R015 notes updates, and R016 validated status all present in DECISIONS.md and REQUIREMENTS.md; added observability gap fixes to T02-PLAN.md and S01-PLAN.md**

## What Happened

All four must-haves were already registered during the T01 ROADMAP creation pass, which wrote the full REQUIREMENTS.md and DECISIONS.md content in a single batch. This task verified each registration individually and addressed pre-flight observability gaps:

1. **D021-D023 in DECISIONS.md** — All 3 strategic decisions present: D021 (3-Phase 순서), D022 (제품 포지셔닝), D023 (재단 그랜츠 어필 포인트). Total decision rows: 23 (≥14 threshold met).
2. **R017-R025 in REQUIREMENTS.md** — All 9 new requirements registered with class, status, description, priority, proposed owner, and notes. Total R0 headings: 25 (16 existing + 9 new).
3. **R006 notes updated** — Contains "M005 리서치" reference explaining generic suggestions → actionable code modification suggestions needed (via R017).
4. **R015 notes updated** — Contains Phase 3(M008) rationale noting foundation grants top appeal value but high implementation complexity.
5. **R016 status validated** — Status shows "validated" with M004 completion evidence.

Pre-flight observability gaps fixed:
- Added `## Observability Impact` to T02-PLAN.md (signals, inspection surfaces, failure visibility)
- Added diagnostic/failure-path checks to S01-PLAN.md (M005 research reference count, R016 validated status)

## Verification

All 11 verification checks passed, including 6 slice-level and 5 task-level checks:
- ROADMAP exists with 3 Phase headings
- 25 R0 requirement headings (≥25 threshold)
- 23 D0 decision rows (≥14 threshold)
- R017, R025, D021, D023 individually present
- R006 notes contain M005 research reference
- R016 status is validated
- R015 notes contain Phase 3(M008) rationale
- Diagnostic checks: Open Risks ≥1, Don't Hand-Roll ≥1, M005 research refs = 2, R016 validated line present

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `test -f .gsd/milestones/M005/M005-ROADMAP.md && echo "ROADMAP exists"` | 0 | ✅ pass | <1ms |
| 2 | `grep -c "^### Phase" .gsd/milestones/M005/M005-ROADMAP.md` → 3 | 0 | ✅ pass | <1ms |
| 3 | `grep -c "^### R0" .gsd/REQUIREMENTS.md` → 25 | 0 | ✅ pass | <1ms |
| 4 | `grep -c "^| D0" .gsd/DECISIONS.md` → 23 | 0 | ✅ pass | <1ms |
| 5 | `grep "R017" .gsd/REQUIREMENTS.md` | 0 | ✅ pass | <1ms |
| 6 | `grep "R025" .gsd/REQUIREMENTS.md` | 0 | ✅ pass | <1ms |
| 7 | `grep "D021" .gsd/DECISIONS.md` | 0 | ✅ pass | <1ms |
| 8 | `grep "D023" .gsd/DECISIONS.md` | 0 | ✅ pass | <1ms |
| 9 | `grep "M005 리서치" .gsd/REQUIREMENTS.md` (R006 notes) | 0 | ✅ pass | <1ms |
| 10 | `grep "validated" .gsd/REQUIREMENTS.md \| grep "R016"` | 0 | ✅ pass | <1ms |
| 11 | `grep "Phase 3.*M008" .gsd/REQUIREMENTS.md` (R015 notes) | 0 | ✅ pass | <1ms |
| 12 | `grep -c "Open Risks" M005-ROADMAP.md` → 1 (diagnostic) | 0 | ✅ pass | <1ms |
| 13 | `grep -c "Don't Hand-Roll" M005-ROADMAP.md` → 1 (diagnostic) | 0 | ✅ pass | <1ms |
| 14 | `grep "M005 리서치" .gsd/REQUIREMENTS.md \| wc -l` → 2 (diagnostic) | 0 | ✅ pass | <1ms |

## Diagnostics

- Requirement count: `grep -c "^### R0" .gsd/REQUIREMENTS.md` (expect 25)
- Decision count: `grep -c "^| D0" .gsd/DECISIONS.md` (expect ≥14)
- Research note propagation: `grep "M005 리서치" .gsd/REQUIREMENTS.md | wc -l` (expect 2 — R006, R015)
- R016 validation: `grep "validated" .gsd/REQUIREMENTS.md | grep "R016"` (expect 1 match)

## Deviations

None — all registrations were already in place from T01's batch write. No additional edits needed to REQUIREMENTS.md or DECISIONS.md content. Only observability gap fixes were applied to plan files.

## Known Issues

None.

## Files Created/Modified

- `.gsd/REQUIREMENTS.md` — verified: 25 R0 headings (R017-R025 present), R006 notes with M005 research, R015 notes with Phase 3 rationale, R016 validated
- `.gsd/DECISIONS.md` — verified: D021-D023 strategic decisions present (23 total D0 rows)
- `.gsd/milestones/M005/slices/S01/tasks/T02-PLAN.md` — added `## Observability Impact` section
- `.gsd/milestones/M005/slices/S01/S01-PLAN.md` — added diagnostic/failure-path checks for R015 notes and R016 status; marked T02 as `[x]`
- `.gsd/milestones/M005/slices/S01/tasks/T02-SUMMARY.md` — created (this file)
