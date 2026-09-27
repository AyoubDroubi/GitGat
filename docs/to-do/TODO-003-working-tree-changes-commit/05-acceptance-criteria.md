# 05 — Acceptance Criteria

| ID | Requirement | Criterion | State |
| --- | --- | --- | --- |
| AC-001 | REQ-001 | Working tree matches Git status for representative repositories. | Draft |
| AC-002 | REQ-002 | Diff view renders additions/removals without corrupting large/binary files. | Draft |
| AC-003 | REQ-003 | Stage/unstage immediately reflects index state. | Draft |
| AC-004 | REQ-004 | Commit succeeds only with valid staged changes and surfaces Git errors. | Draft |
| AC-005 | REQ-005 | Discard requires confirmation and never silently deletes untracked work. | Draft |
| AC-006 | REQ-006 | Long Git operations keep the desktop UI responsive. | Draft |

## Regression criteria

- [ ] Existing repository data/history is not corrupted.
- [ ] UI remains responsive for expected operations.
- [ ] Errors are actionable and do not leak secrets.

## Gate

All criteria for the current approved baseline must be approved before READY.
