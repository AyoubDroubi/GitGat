# Product Change Index

Global dashboard for GitGat tracked product changes.

## Health legend

- `GREEN`: unblocked, evidence/docs consistent for current stage.
- `YELLOW`: pending question/dependency or non-blocking documentation drift.
- `RED`: active blocker, material drift, missing gate evidence, or unresolved blocking failure.

| ID | Idea | Track | Owner | Area | Status | Health | Baseline | Open Qs | Blockers | Doc Drift | Last activity | Folder |
| --- | --- | --- | --- | --- | --- | --- | --- | ---: | --- | --- | --- | --- |
| TODO-001 | Desktop foundation & re-architecture | MAJOR | Ayoub | Architecture / Desktop Foundation | IN_REVIEW | YELLOW | v1-approved | 1 | Independent review/closeout | None | 2026-09-27 | [TODO-001](./TODO-001-foundation-rearchitecture/) |
| TODO-002 | Repository onboarding, clone & local catalog | STANDARD | Ayoub | Repositories | CAPTURED | YELLOW | v0-draft | 1 | TODO-001 | None | 2026-09-27 | [TODO-002](./TODO-002-repository-onboarding-catalog/) |
| TODO-003 | Files, changes, staging & commit workflow | MAJOR | Ayoub | Working Tree | CAPTURED | YELLOW | v0-draft | 1 | TODO-002 | None | 2026-09-27 | [TODO-003](./TODO-003-working-tree-changes-commit/) |
| TODO-004 | Fetch/pull/push, branches & history | MAJOR | Ayoub | Git Core | CAPTURED | YELLOW | v0-draft | 1 | TODO-002, TODO-003 | None | 2026-09-27 | [TODO-004](./TODO-004-sync-branches-history/) |
| TODO-005 | GitHub connection, identity & forge abstraction | MAJOR | Ayoub | GitHub / Security | CAPTURED | YELLOW | v0-draft | 1 | TODO-002 | None | 2026-09-27 | [TODO-005](./TODO-005-github-identity-auth-forge/) |
| TODO-006 | Pull requests, reviews, comments & merge | MAJOR | Ayoub | Collaboration | CAPTURED | YELLOW | v0-draft | 1 | TODO-003, TODO-004, TODO-005 | None | 2026-09-27 | [TODO-006](./TODO-006-pull-requests-reviews-comments/) |
| TODO-007 | Git LFS file locking | MAJOR | Ayoub | Assets / Collaboration | CAPTURED | YELLOW | v0-draft | 1 | TODO-002, TODO-003 | None | 2026-09-27 | [TODO-007](./TODO-007-git-lfs-file-locking/) |
| TODO-008 | Multi-repository workspaces & My Work | STANDARD | Ayoub | Workspace | CAPTURED | YELLOW | v0-draft | 1 | TODO-002, TODO-005, TODO-006 | None | 2026-09-27 | [TODO-008](./TODO-008-workspaces-my-work/) |
| TODO-009 | GitHub Actions, notifications & team activity | STANDARD | Ayoub | Collaboration / CI | CAPTURED | YELLOW | v0-draft | 1 | TODO-005, TODO-006 | None | 2026-09-27 | [TODO-009](./TODO-009-actions-notifications-activity/) |
| TODO-010 | GitGat design system, UX polish & accessibility | STANDARD | Ayoub | Design System / UX | CAPTURED | YELLOW | v0-draft | 1 | TODO-002, TODO-003 | None | 2026-09-27 | [TODO-010](./TODO-010-design-system-ux-accessibility/) |
| TODO-011 | Recovery, conflicts & Advanced Git | MAJOR | Ayoub | Advanced Git / Safety | CAPTURED | YELLOW | v0-draft | 1 | TODO-003, TODO-004, TODO-007 | None | 2026-09-27 | [TODO-011](./TODO-011-recovery-conflicts-advanced-git/) |
| TODO-012 | Quality, performance, security & cross-platform hardening | MAJOR | Ayoub | Quality / Release Readiness | CAPTURED | YELLOW | v0-draft | 1 | TODO-002..TODO-011 | None | 2026-09-27 | [TODO-012](./TODO-012-quality-cross-platform-hardening/) |
| TODO-013 | Windows executable & installer artifact | MAJOR | Ayoub | Packaging / Distribution | CAPTURED | YELLOW | v0-draft | 1 | TODO-010, TODO-012 | None | 2026-09-27 | [TODO-013](./TODO-013-windows-exe-installer-artifact/) |

## Snapshot

- Total: **13**
- Captured: **12**
- Discussion: **0**
- Approved: **0**
- Ready: **0**
- In progress: **0**
- Validating: **0**
- In review: **1**
- Changes requested: **0**
- Ready to merge: **0**
- Done: **0**
- On hold: **0**
- Rejected: **0**
- GREEN: **0**
- YELLOW: **13**
- RED: **0**

## Execution order

See [MASTER-PLAN.md](./MASTER-PLAN.md). Status governs what can actually be implemented; ordering alone is not start authorization.
