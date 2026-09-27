# 04 — Scope / Product Baseline

## Current baseline

`v0-draft`

## Baseline history

| Baseline | State | Date | Trigger | Approval evidence |
| --- | --- | --- | --- | --- |
| v0-draft | Draft | 2026-09-27 | Roadmap capture | Not yet approved for implementation |

## Requirements in current baseline

| ID | Requirement | State | Source/Decision |
| --- | --- | --- | --- |
| REQ-001 | Publish GitGat for win-x64 in Release mode as self-contained so end users do not need a separate .NET installation. | Draft | D-001 |
| REQ-002 | Evaluate single-file publishing and use it when runtime/native dependency behavior is verified. | Draft | D-001 |
| REQ-003 | Produce a traditional Windows installer .exe using free tooling (preferred evaluation: Inno Setup or NSIS). | Draft | D-001 |
| REQ-004 | Set application name/version/icon and clean install/uninstall behavior. | Draft | D-001 |
| REQ-005 | Generate SHA-256/checksum metadata and register every successful/failed artifact attempt. | Draft | D-001 |
| REQ-006 | Provide a repeatable build command/workflow from main; production branch remains untouched until separately authorized. | Draft | D-001 |
| REQ-007 | Smoke-test the published app/installer on a clean supported Windows environment. | Draft | D-001 |

## In scope

- Publish GitGat for win-x64 in Release mode as self-contained so end users do not need a separate .NET installation.
- Evaluate single-file publishing and use it when runtime/native dependency behavior is verified.
- Produce a traditional Windows installer .exe using free tooling (preferred evaluation: Inno Setup or NSIS).
- Set application name/version/icon and clean install/uninstall behavior.
- Generate SHA-256/checksum metadata and register every successful/failed artifact attempt.
- Provide a repeatable build command/workflow from main; production branch remains untouched until separately authorized.
- Smoke-test the published app/installer on a clean supported Windows environment.

## Out of scope

- Unrelated TODO capabilities.
- production changes without explicit authorization.

## Dependencies

**Depends on:** TODO-010, TODO-012  
**Blocked by:** Release-readiness validation/hardening incomplete  
**Blocks:** See INDEX/master plan  
**Related TODOs:** None

## UX boundaries

- Keep common workflows understandable to non-expert Git users.
- Advanced detail must remain available without overwhelming normal flows.

## Data/ownership boundaries

- Git remains source of truth for repository history/state.
- GitGat local state must not silently rewrite repository truth.

## Architecture impact

Use existing Clean Architecture boundaries; add ADRs for material cross-cutting decisions.

## Risks

- Unsigned installers may trigger Windows reputation warnings; code signing is a separate future distribution decision.
- Single-file/native dependency behavior must be validated, not assumed.
- Installer scripts can leave stale files/settings if upgrade/uninstall rules are wrong.

## Scope approval

Not approved yet.
