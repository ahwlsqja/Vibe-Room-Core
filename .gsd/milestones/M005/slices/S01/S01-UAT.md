# S01: Formalize Research into Actionable Roadmap — UAT

**Milestone:** M005
**Written:** 2026-03-24

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: This is a documentation-only slice with no runtime components. All deliverables are static markdown files whose content can be verified by file existence checks, grep counts, and structural inspection.

## Preconditions

- Working directory is the M005 worktree or main repo with `.gsd/` directory accessible
- M005-RESEARCH.md exists (input artifact — was completed before this slice)
- `.gsd/REQUIREMENTS.md` and `.gsd/DECISIONS.md` exist

## Smoke Test

```bash
test -f .gsd/milestones/M005/M005-ROADMAP.md && \
  grep -q "Phase 1" .gsd/milestones/M005/M005-ROADMAP.md && \
  grep -q "R017" .gsd/REQUIREMENTS.md && \
  grep -q "D021" .gsd/DECISIONS.md && \
  echo "SMOKE PASS" || echo "SMOKE FAIL"
```

Expected: `SMOKE PASS`

## Test Cases

### 1. M005-ROADMAP.md Structure Completeness

1. Run `grep -c "^### Phase" .gsd/milestones/M005/M005-ROADMAP.md`
2. **Expected:** 3 (Phase 1 핵심 차별화 강화, Phase 2 사용자 유입 최적화, Phase 3 리텐션 + 생태계 확장)
3. Run `grep "^### Phase" .gsd/milestones/M005/M005-ROADMAP.md`
4. **Expected:** Three lines containing "Phase 1", "Phase 2", "Phase 3" respectively

### 2. Milestone Proposals Present (M006/M007/M008)

1. Run `grep -c "^### M00[678]" .gsd/milestones/M005/M005-ROADMAP.md`
2. **Expected:** 3 (one heading each for M006, M007, M008)
3. For each milestone, verify scope, covered requirements, and estimated effort are listed:
   ```bash
   grep -A5 "^### M006" .gsd/milestones/M005/M005-ROADMAP.md | grep -q "Covers:"
   grep -A5 "^### M007" .gsd/milestones/M005/M005-ROADMAP.md | grep -q "Covers:"
   grep -A5 "^### M008" .gsd/milestones/M005/M005-ROADMAP.md | grep -q "Covers:"
   ```
4. **Expected:** All three grep commands exit 0

### 3. New Requirements Registration (R017-R025)

1. Run `grep -c "^### R0" .gsd/REQUIREMENTS.md`
2. **Expected:** 25 (16 existing + 9 new)
3. Verify each new requirement individually:
   ```bash
   for r in R017 R018 R019 R020 R021 R022 R023 R024 R025; do
     grep -q "^### $r" .gsd/REQUIREMENTS.md && echo "$r: OK" || echo "$r: MISSING"
   done
   ```
4. **Expected:** All 9 print "OK"
5. Spot-check R017 contains "differentiator" class and "P0" priority context:
   ```bash
   grep -A5 "^### R017" .gsd/REQUIREMENTS.md | grep -q "differentiator"
   ```
6. **Expected:** Exit 0

### 4. Strategic Decisions Registration (D021-D023)

1. Run `grep -c "^| D0" .gsd/DECISIONS.md`
2. **Expected:** ≥14 (11 existing + 3 new)
3. Verify each new decision:
   ```bash
   for d in D021 D022 D023; do
     grep -q "$d" .gsd/DECISIONS.md && echo "$d: OK" || echo "$d: MISSING"
   done
   ```
4. **Expected:** All 3 print "OK"

### 5. R006 Notes Updated with Research Finding

1. Run `grep -A10 "^### R006" .gsd/REQUIREMENTS.md | grep "M005"`
2. **Expected:** Contains reference to M005 research finding about generic suggestions → actionable code modification suggestions needed
3. Run `grep -A10 "^### R006" .gsd/REQUIREMENTS.md | grep "R017"`
4. **Expected:** Contains cross-reference to R017 (병렬 실행 최적화 제안)

### 6. R015 Notes Updated with Phase 3 Rationale

1. Run `grep -A10 "^### R015" .gsd/REQUIREMENTS.md | grep "M005"`
2. **Expected:** Contains M005 research reference
3. Run `grep -A10 "^### R015" .gsd/REQUIREMENTS.md | grep "Phase 3"`
4. **Expected:** Contains Phase 3(M008) re-evaluation note

### 7. R016 Status Changed to Validated

1. Run `grep -A3 "^### R016" .gsd/REQUIREMENTS.md | grep "Status:"`
2. **Expected:** Contains "validated"

### 8. Don't Hand-Roll Registry Present

1. Run `grep -c "Don't Hand-Roll" .gsd/milestones/M005/M005-ROADMAP.md`
2. **Expected:** ≥1
3. Run `grep -A20 "Don't Hand-Roll" .gsd/milestones/M005/M005-ROADMAP.md | grep -c "|"` (table rows)
4. **Expected:** ≥5 (header + 5 entries)

### 9. Open Risks Carried Forward

1. Run `grep -c "Open Risks" .gsd/milestones/M005/M005-ROADMAP.md`
2. **Expected:** ≥1
3. Run `grep -A30 "Open Risks" .gsd/milestones/M005/M005-ROADMAP.md | grep -c "^\d\."` or count numbered items
4. **Expected:** ≥5 risks listed

## Edge Cases

### Requirement ID Gaps

1. Run `grep "^### R0" .gsd/REQUIREMENTS.md | sort`
2. **Expected:** Continuous from R001 to R025 with no gaps (R001-R016 existing, R017-R025 new)
3. No duplicate IDs present: `grep "^### R0" .gsd/REQUIREMENTS.md | sort | uniq -d`
4. **Expected:** No output (no duplicates)

### Decision ID Continuity

1. Run `grep "D02[123]" .gsd/DECISIONS.md | wc -l`
2. **Expected:** ≥3 (D021, D022, D023 each appear at least once)

### Cross-Reference Consistency

1. Verify R017-R025 in ROADMAP match R017-R025 in REQUIREMENTS.md:
   ```bash
   for r in R017 R018 R019 R020 R021 R022 R023 R024 R025; do
     grep -q "$r" .gsd/milestones/M005/M005-ROADMAP.md && \
     grep -q "$r" .gsd/REQUIREMENTS.md && \
     echo "$r: consistent" || echo "$r: MISMATCH"
   done
   ```
2. **Expected:** All 9 print "consistent"

## Failure Signals

- `grep -c "^### Phase" M005-ROADMAP.md` returns < 3 → missing phase sections
- `grep -c "^### R0" REQUIREMENTS.md` returns < 25 → requirements not fully registered
- `grep "M005 리서치" REQUIREMENTS.md | wc -l` returns < 2 → R006 or R015 notes not updated
- Any R017-R025 missing from REQUIREMENTS.md → incomplete registration
- Any D021-D023 missing from DECISIONS.md → incomplete registration
- R016 status not "validated" → missed status update

## Not Proven By This UAT

- **Content quality of the roadmap** — this UAT verifies structural completeness (sections exist, counts match) but does not evaluate whether the strategic recommendations are sound. That requires human review.
- **Requirement feasibility** — R017-R025 are registered as proposals with "unmapped" validation. Whether they're achievable with current architecture is unproven.
- **Research data accuracy** — the UAT verifies the research was formalized into artifacts, not that the underlying data is correct or current.
- **Runtime behavior** — no runtime components exist in this slice. There's nothing to run or deploy.

## Notes for Tester

- This is a pure documentation milestone. All tests are `grep`/`test` commands — no servers, builds, or browsers needed.
- The DECISIONS.md count (23 rows) seems high relative to 11+3=14 expected — this is because existing decision entries may span multiple table rows. Verify D021-D023 presence specifically rather than relying on exact row counts.
- R016's section is under "## Out of Scope" in REQUIREMENTS.md (its original category from M004), but its status field reads "validated." This is intentional — it was completed and validated in M004.
- The Korean text in requirements and roadmap is intentional — this is a Korean-language project with mixed Korean/English documentation.
