# 05 — Acceptance Criteria

| ID | Requirement | Criterion | State |
| --- | --- | --- | --- |
| AC-001 | REQ-001 | Selecting a valid .git repository opens it and populates the workspace. | Draft |
| AC-002 | REQ-002 | Clone shows progress/failure and opens the repository only after success. | Draft |
| AC-003 | REQ-003 | Repositories survive app restart using SQLite-backed local state. | Draft |
| AC-004 | REQ-004 | Switcher exposes branch and basic status without entering the repo. | Draft |
| AC-005 | REQ-005 | Missing paths show a recover/remove action instead of crashing. | Draft |

## Regression criteria

- [ ] Existing repository data/history is not corrupted.
- [ ] UI remains responsive for expected operations.
- [ ] Errors are actionable and do not leak secrets.

## Gate

All criteria for the current approved baseline must be approved before READY.
