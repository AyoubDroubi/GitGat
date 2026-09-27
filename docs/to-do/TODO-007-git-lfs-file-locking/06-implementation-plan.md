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

1. Add IGitLfsClient boundary and command adapter.
2. Detect LFS capability and remote lock support.
3. Build lock model/cache/query.
4. Add Files/Changes lock badges and context actions.
5. Build Locks workspace and My Locks view.
6. Add guided .gitattributes lockable configuration.
7. Test contention, stale locks and permission failures.

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

- Locking depends on remote/server LFS locking support.
- Stale caches can mislead users about asset ownership.

## Rollback

Revert implementation on main; preserve local repository/user data; document recovery for any stateful operation.

## Documentation impact

- Update relevant TODO evidence and architecture docs.

## Readiness result

Not ready — CAPTURED.
