# 06 — Implementation Plan

**Track:** STANDARD  
**Target baseline:** future v1-approved

## Preconditions

- Approved scope/baseline.
- Approved acceptance criteria.
- Dependencies ready.
- Required architecture decisions complete.
- Explicit start authorization recorded before product code changes.

## Ordered steps

1. Introduce persistent repository entity/catalog with EF Core SQLite.
2. Build native Open Repository flow.
3. Build clone service behind application boundary.
4. Build repository switcher with recent/search/favorites.
5. Add moved/missing repository recovery.
6. Add tests and validation.

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

- Large repo discovery/status calls could block UI.
- Clone credentials and host errors require clear user-safe messages.

## Rollback

Revert implementation on main; preserve local repository/user data; document recovery for any stateful operation.

## Documentation impact

- Update relevant TODO evidence and architecture docs.

## Readiness result

Not ready — CAPTURED.
