# 12 — Risk / Migration / Release

**Track:** STANDARD  
**Required:** Maintain proportionate risk notes; escalate if risk grows.

## Risk register

| ID | Risk | Likelihood | Impact | Mitigation | Owner | Status |
| --- | --- | --- | --- | --- | --- | --- |
| RISK-001 | Actions logs can be large. | Medium | High | Validate early; preserve rollback/recovery paths; keep evidence. | Ayoub | Open |
| RISK-002 | Notification APIs may differ in freshness/permissions. | Medium | High | Validate early; preserve rollback/recovery paths; keep evidence. | Ayoub | Open |
| RISK-003 | Too much activity can make the home screen noisy. | Medium | High | Validate early; preserve rollback/recovery paths; keep evidence. | Ayoub | Open |

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
