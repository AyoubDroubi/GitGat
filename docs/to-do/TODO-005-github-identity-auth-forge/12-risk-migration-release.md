# 12 — Risk / Migration / Release

**Track:** MAJOR  
**Required:** Yes — MAJOR item

## Risk register

| ID | Risk | Likelihood | Impact | Mitigation | Owner | Status |
| --- | --- | --- | --- | --- | --- | --- |
| RISK-001 | OAuth/device-flow configuration may need a registered GitHub app. | Medium | High | Validate early; preserve rollback/recovery paths; record failures. | Ayoub | Open |
| RISK-002 | Credential leakage through logs/errors is unacceptable. | Medium | High | Validate early; preserve rollback/recovery paths; record failures. | Ayoub | Open |

## Data / migration impact

- Schema change: TBD where applicable.
- Backfill: None unless discovered during implementation.
- Data compatibility: Existing Git repositories must never be mutated outside explicit user actions.
- Rollback compatibility: Product changes must be revertible without damaging repository history.

## Release / rollout plan

- Development target: main.
- Production decisions: intentionally deferred; production branch is not modified by this TODO without explicit user authorization.
- Success criteria: acceptance criteria + validation + independent review + traceability pass.
- Stop conditions: data-loss risk, auth/secret leakage, unrecoverable Git mutation, or unresolved blocking review finding.

## Rollback plan

Revert the implementation commit(s) on main and preserve repository/user data. Destructive Git operations require their own recovery strategy before shipping.

## Architecture / ADR references

- ADR required: Evaluate during readiness.
- References: docs/architecture.md
