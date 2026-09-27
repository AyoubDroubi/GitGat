# 01 — Idea

## Original request

Establish GitGat as a .NET 10 desktop application using Avalonia 12, SukiUI 7, CommunityToolkit.Mvvm, Clean Architecture, System Git and CI validation.

## Problem

GitGat needs this capability to deliver a complete, approachable desktop Git workflow instead of exposing command-line complexity.

## Product value

Desktop foundation & re-architecture moves GitGat toward a single modern workspace for repositories, collaboration and safe team workflows.

## Initial requirements

| ID | Requirement | Source | State |
| --- | --- | --- | --- |
| REQ-001 | Use .NET 10 as the application runtime and C# as the primary language. | Roadmap decision | Approved |
| REQ-002 | Use Avalonia 12 + SukiUI 7 with a GitGat-owned design system. | Roadmap decision | Approved |
| REQ-003 | Separate Domain, Application, Infrastructure and Desktop concerns. | Roadmap decision | Approved |
| REQ-004 | Use installed System Git behind application interfaces; UI must not shell out directly. | Roadmap decision | Approved |
| REQ-005 | Build the complete solution successfully in CI on Windows. | Roadmap decision | Approved |

## Initial proposal

Follow the ordered plan in `06-implementation-plan.md`, using application boundaries rather than direct UI-to-Git/provider coupling.

## Constraints

- Development work stays on main unless the user explicitly changes branch policy.
- production is not modified without explicit authorization.
- No silent destructive Git behavior.
- No plaintext secrets.
- Keep UI responsive during process/network work.

## Non-goals

- Production release/deployment behavior unless explicitly part of the TODO and authorized later.

## Initial track recommendation

MAJOR — scope/risk requires this governance depth.
