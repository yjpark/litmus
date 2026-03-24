# Fixture Candidates

Staging area for proposed fixtures before they are promoted to the main `fixtures/` directory.

## Review Workflow

1. **Create**: Add a new candidate as `fixtures/candidates/{id}/` with `setup.sh` and `command.sh`
2. **Test locally**: Run `FIXTURE_WORK_DIR=$(mktemp -d) && bash fixtures/candidates/{id}/setup.sh && cd $FIXTURE_WORK_DIR && bash fixtures/candidates/{id}/command.sh`
3. **Parse**: Verify ANSI output parses cleanly: `bash command.sh | cargo run -p litmus-capture -- parse-ansi --id {id}`
4. **Assess**: Fill out `REVIEW.md` for the candidate (copy the template below)
5. **Promote or discard**: Move passing candidates to `fixtures/{id}/` or remove them

## Quality Criteria

Every fixture must pass all five criteria:

| # | Criterion | Description |
|---|-----------|-------------|
| 1 | **Color variety** | Uses ≥4 distinct ANSI colors (ideally covers standard + bright) |
| 2 | **Instant recognition** | Output is immediately recognizable as a common terminal scenario |
| 3 | **Fits 80x24** | Output stays within 80 columns × 24 rows |
| 4 | **Deterministic** | Same output every run (no dates, PIDs, system-dependent data) |
| 5 | **Self-contained** | No external dependencies beyond standard tools (bash, git, etc.) |

## REVIEW.md Template

Copy this into `fixtures/candidates/{id}/REVIEW.md` for each candidate:

```markdown
# Review: {fixture-id}

**Description**: What this fixture demonstrates
**Command**: The primary command shown

## Quality Criteria

- [ ] Color variety (≥4 distinct ANSI colors)
- [ ] Instant recognition
- [ ] Fits 80x24
- [ ] Deterministic
- [ ] Self-contained

## Color Coverage

List the ANSI colors used:
- fg: (which colors?)
- bg: (which colors?)
- 256/truecolor: (if any)

## Notes

Any observations, edge cases, or concerns.

## Decision

- [ ] Promote to fixtures/
- [ ] Needs changes (describe below)
- [ ] Discard (reason below)
```
