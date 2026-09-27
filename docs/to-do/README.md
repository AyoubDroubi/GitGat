# Product Change Tracking System

Mandatory end-to-end tracking area for the target project.

## Core rule

No product-facing change bypasses this system.

> Chat is input. Repository is memory.

Every change has a folder and an INDEX entry before implementation.

## Tracks

- `SMALL` — low-risk/localized; lighter planning allowed, audit evidence still required.
- `STANDARD` — default complete lifecycle.
- `MAJOR` — full lifecycle plus risk/migration/release/rollback and architecture evidence when applicable.

## Required item folder

`TODO-NNN-short-name/`

Required records:

1. `README.md`
2. `01-idea.md`
3. `02-discussion-log.md`
4. `03-decisions.md`
5. `04-scope.md`
6. `05-acceptance-criteria.md`
7. `06-implementation-plan.md`
8. `07-implementation-log.md`
9. `08-validation.md`
10. `09-review.md`
11. `10-release-closeout.md`
12. `11-traceability.md`
13. `12-risk-migration-release.md`
14. `13-operational-log.md`

For SMALL items, planning/risk files can explicitly be N/A when justified. For MAJOR items, risk/migration/release is mandatory.

## Product baseline

Use versioned baselines such as `v0-draft`, `v1-approved`, `v2-approved`.

Approved scope is never silently overwritten.

## Dependencies

Track `Depends on`, `Blocked by`, `Blocks`, and `Related TODOs`.

## Lifecycle

`CAPTURED → DISCUSSION → APPROVED → READY → IN_PROGRESS → VALIDATING → IN_REVIEW → READY_TO_MERGE → DONE`

Branch states:

`CHANGES_REQUESTED`, `ON_HOLD`, `REJECTED`

## Health

- `GREEN` — consistent, unblocked, evidence current.
- `YELLOW` — pending questions/dependencies or non-blocking drift.
- `RED` — blocked, missing gate evidence, material drift, or unresolved blocking failure.

## Traceability

Every material requirement gets a `REQ-` ID and maps through:

`Requirement → Decision → Acceptance Criterion → Implementation → Validation → Review → Release`

## Key rules

- Discussion is not approval.
- Approval is not implementation authorization.
- READY still requires explicit start authorization.
- New scope after approval creates/updates a baseline and requires re-readiness.
- Logs are append-only.
- Superseded decisions/baselines remain visible.
- Validation/review claims need evidence.
- INDEX and item README must stay synchronized.
- Product tracking does not replace ADRs or architecture documentation.


## Operational records

The target repository also contains `operations/`:

- `ACTIVITY-LEDGER.md`
- `DEPLOYMENT-REGISTRY.md`
- `ARTIFACT-REGISTRY.md`
- `ENVIRONMENTS.md`

Every meaningful state-changing action or material verification result is logged. APK production and deployments are never represented only by chat claims; they require operational evidence.
