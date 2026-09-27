# 01 — Idea

## Original request

Produce a repeatable Windows x64 GitGat application artifact: self-contained GitGat.exe plus a user-installable setup .exe, with checksums and artifact evidence.

## Problem

GitGat needs this capability to complete a safe, modern desktop Git experience without forcing users into command-line Git.

## Product value

Windows executable & installer artifact closes a required part of the end-to-end GitGat product roadmap.

## Initial requirements

| ID | Requirement | Source | State |
| --- | --- | --- | --- |
| REQ-001 | Publish GitGat for win-x64 in Release mode as self-contained so end users do not need a separate .NET installation. | Roadmap decision | Draft |
| REQ-002 | Evaluate single-file publishing and use it when runtime/native dependency behavior is verified. | Roadmap decision | Draft |
| REQ-003 | Produce a traditional Windows installer .exe using free tooling (preferred evaluation: Inno Setup or NSIS). | Roadmap decision | Draft |
| REQ-004 | Set application name/version/icon and clean install/uninstall behavior. | Roadmap decision | Draft |
| REQ-005 | Generate SHA-256/checksum metadata and register every successful/failed artifact attempt. | Roadmap decision | Draft |
| REQ-006 | Provide a repeatable build command/workflow from main; production branch remains untouched until separately authorized. | Roadmap decision | Draft |
| REQ-007 | Smoke-test the published app/installer on a clean supported Windows environment. | Roadmap decision | Draft |

## Initial proposal

Follow the ordered implementation plan after governance readiness/start authorization.

## Constraints

- Normal work targets main.
- production remains untouched without explicit user authorization.
- No silent destructive behavior or plaintext secrets.
- Evidence is required for build/review/artifact claims.

## Non-goals

- Any unrelated capability tracked by another TODO.

## Initial track recommendation

MAJOR — chosen based on scope/risk.

## References

- https://docs.avaloniaui.net/xpf/deployment/windows
- https://docs.avaloniaui.net/tools/parcel/packaging-for-windows
