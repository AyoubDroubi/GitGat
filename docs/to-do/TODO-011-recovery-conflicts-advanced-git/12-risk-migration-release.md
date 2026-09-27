# 12 — Risk / Migration / Release

**Track:** MAJOR  
**Required:** Yes — MAJOR item

## Risk register

| ID | Risk | Likelihood | Impact | Mitigation | Owner | Status |
| --- | --- | --- | --- | --- | --- | --- |
| RISK-001 | History-changing operations can permanently lose work. | Medium | High | Validate early; preserve rollback/recovery paths; keep evidence. | Ayoub | Open |
| RISK-002 | Platform/process interruptions can leave repositories mid-operation. | Medium | High | Validate early; preserve rollback/recovery paths; keep evidence. | Ayoub | Open |
| RISK-003 | A false sense of recovery safety is dangerous. | Medium | High | Validate early; preserve rollback/recovery paths; keep evidence. | Ayoub | Open |

## Data / migration impact

- Schema change: TBD where applicable.
- Backfill: None unless discovered.
- Data compatibility: preserve existing repositories/settings or migrate explicitly.
- Rollback compatibility: must be defined before READY.

## Release / rollout plan

- Development target: main.
- production decision: explicitly deferred; do not modify production under this TODO without new authorization.
- Success: AC + validation + independent review + traceability pass.
- Stop: data-loss risk, secret leakage, unrecoverable Git mutation, or blocking review finding.

## Rollback plan

Revert product changes on main and preserve local repository/user data. Record any stateful recovery procedure before release.

## Architecture / ADR references

- ADR required: evaluate during readiness.
- References: docs/architecture.md
