# 05 — Acceptance Criteria

| ID | Requirement | Criterion | State |
| --- | --- | --- | --- |
| AC-001 | REQ-001 | Missing Git LFS shows setup guidance without breaking normal Git. | Draft |
| AC-002 | REQ-002 | Locks view matches git lfs locks output. | Draft |
| AC-003 | REQ-003 | Successful lock/unlock refreshes state immediately. | Draft |
| AC-004 | REQ-004 | Locked files clearly show who owns the lock. | Draft |
| AC-005 | REQ-005 | Force unlock is never the default action and clearly names the affected lock. | Draft |
| AC-006 | REQ-006 | Common binary patterns can be marked lockable through guided configuration. | Draft |

## Regression criteria

- [ ] Existing repository data/history is not corrupted.
- [ ] UI remains responsive for expected operations.
- [ ] Errors are actionable and do not leak secrets.

## Gate

All criteria for the current approved baseline must be approved before READY.
