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

1. Implement PR query/detail DTOs and forge methods.
2. Build PR list/detail navigation.
3. Build changed-files review UI.
4. Build comments and review submission.
5. Build create/edit PR workflow.
6. Build guarded merge flow.
7. Add caching/refresh and error-state tests.

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

- Review APIs have subtle line/position semantics.
- Stale mergeability/review state can cause confusing actions.

## Rollback

Revert implementation on main; preserve local repository/user data; document recovery for any stateful operation.

## Documentation impact

- Update relevant TODO evidence and architecture docs.

## Readiness result

Not ready — CAPTURED.
