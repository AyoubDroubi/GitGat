# 06 — Implementation Plan

**Track:** MAJOR  
**Target baseline:** future v1-approved

## Preconditions

- Approved scope/baseline.
- Approved acceptance criteria.
- Dependencies ready.
- Required architecture decisions complete.
- Explicit start authorization recorded before product code changes.

## Ordered steps

1. Move status parsing to porcelain v2 typed models.
2. Add file/change query layer and refresh coordinator.
3. Implement diff service and diff UI.
4. Implement stage/unstage commands.
5. Implement commit composer and commit execution.
6. Implement guarded discard/recovery behavior.
7. Add regression tests and large-repo checks.

## Areas/files expected to change

- Domain/Application contracts where needed.
- Infrastructure adapters for Git/provider/platform behavior.
- Desktop ViewModels/Views/design system.
- Tests and docs/to-do evidence.

## Data/migration impact

Evaluate SQLite/settings changes during readiness; Git repository history must remain compatible.

## Test plan

- Unit tests for parsing/domain/application behavior.
- Integration tests around Git/process/provider boundaries where practical.
- Release build in CI.
- Manual desktop scenarios for destructive/high-value workflows.

## Risks

- Incorrect diff/status parsing can lose user trust.
- Destructive operations can cause data loss if guardrails fail.

## Rollback

Revert implementation on main; preserve local repository/user data; document recovery for any stateful operation.

## Documentation impact

- Update relevant TODO evidence and architecture docs.

## Readiness result

Not ready — CAPTURED.
