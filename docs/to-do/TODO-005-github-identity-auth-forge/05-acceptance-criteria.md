# 05 — Acceptance Criteria

| ID | Requirement | Criterion | State |
| --- | --- | --- | --- |
| AC-001 | REQ-001 | User can connect/disconnect GitHub from Settings. | Draft |
| AC-002 | REQ-002 | No GitHub token appears in repo files, logs, SQLite or plaintext config. | Draft |
| AC-003 | REQ-003 | GitHub repositories map correctly from HTTPS and SSH remotes. | Draft |
| AC-004 | REQ-004 | PR/Actions features depend on forge interfaces, not direct UI HTTP calls. | Draft |
| AC-005 | REQ-005 | Revoked auth produces actionable reconnect state. | Draft |

## Regression criteria

- [ ] Existing repository data/history is not corrupted.
- [ ] UI remains responsive for expected operations.
- [ ] Errors are actionable and do not leak secrets.

## Gate

All criteria for the current approved baseline must be approved before READY.
