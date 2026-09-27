# 05 — Acceptance Criteria

| ID | Requirement | Criterion | State |
| --- | --- | --- | --- |
| AC-001 | REQ-001 | Core logic has deterministic automated coverage for success/failure cases. | Draft |
| AC-002 | REQ-002 | Disposable-repo tests validate status/stage/commit/branch/sync/locks where feasible. | Draft |
| AC-003 | REQ-003 | CI produces successful Release builds for the declared platform matrix. | Draft |
| AC-004 | REQ-004 | Representative large repos do not freeze the UI and long operations can be cancelled safely. | Draft |
| AC-005 | REQ-005 | Security review has no unresolved high-severity secret/data-loss findings. | Draft |
| AC-006 | REQ-006 | All release-bound TODOs have review/validation/traceability evidence or explicit deferral. | Draft |
| AC-007 | REQ-007 | README/release docs state only verified supported platforms. | Draft |

## Regression criteria

- [ ] No repository/user data corruption.
- [ ] UI stays responsive during expected operations.
- [ ] Failures are actionable and do not leak secrets.

## Gate

Criteria remain Draft until scope/baseline approval.
