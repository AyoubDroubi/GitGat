# 06 — Implementation Plan

**Track:** MAJOR  
**Target baseline:** future v1-approved

## Preconditions

- Scope and acceptance criteria approved.
- Dependencies complete/ready.
- Architecture/risk work complete where required.
- Explicit start authorization recorded.

## Ordered steps

1. Finalize semantic version/application metadata/icon.
2. Add win-x64 self-contained publish profile/command.
3. Evaluate single-file publish against Avalonia/native dependencies.
4. Choose/configure free installer tool (Inno Setup or NSIS).
5. Add packaging script/workflow that consumes main commit/version.
6. Generate GitGat.exe and GitGat-Setup-x64.exe.
7. Compute SHA-256 and register ART evidence.
8. Smoke-test on clean Windows VM/machine.
9. Close packaging TODO only after independent review and artifact verification.

## Areas/files expected to change

- Domain/Application contracts as required.
- Infrastructure adapters/platform/provider code.
- Desktop Views/ViewModels/design system.
- Automated tests, CI and docs/to-do evidence.

## Data/migration impact

Evaluate and document before READY; never infer safe migration behavior.

## Test plan

- Unit tests for deterministic logic/parsing.
- Integration tests at Git/provider/process boundaries where practical.
- Release build CI.
- Manual scenarios for high-value/destructive workflows.

## Risks

- Unsigned installers may trigger Windows reputation warnings; code signing is a separate future distribution decision.
- Single-file/native dependency behavior must be validated, not assumed.
- Installer scripts can leave stale files/settings if upgrade/uninstall rules are wrong.

## Rollback

Revert implementation commits on main while preserving user/repository data; define feature-specific recovery before shipping.

## Documentation impact

Update TODO evidence and architecture/user docs affected by implementation.

## Readiness result

Not ready — CAPTURED.

## References

- https://docs.avaloniaui.net/xpf/deployment/windows
- https://docs.avaloniaui.net/tools/parcel/packaging-for-windows
