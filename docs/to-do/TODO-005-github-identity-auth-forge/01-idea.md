# 01 — Idea

## Original request

Connect GitGat to GitHub securely while keeping provider-specific behavior behind a forge abstraction.

## Problem

GitGat needs this capability to deliver a complete, approachable desktop Git workflow instead of exposing command-line complexity.

## Product value

GitHub connection, identity & forge abstraction moves GitGat toward a single modern workspace for repositories, collaboration and safe team workflows.

## Initial requirements

| ID | Requirement | Source | State |
| --- | --- | --- | --- |
| REQ-001 | Connect to GitHub using a user-authorized flow without storing secrets in SQLite. | Roadmap decision | Draft |
| REQ-002 | Store credentials only in OS secure storage or provider CLI secure storage. | Roadmap decision | Draft |
| REQ-003 | Resolve repository owner/name/host from remotes reliably. | Roadmap decision | Draft |
| REQ-004 | Expose provider capabilities through an IGitForge-style application boundary. | Roadmap decision | Draft |
| REQ-005 | Handle expired/revoked auth with reconnect UX, never silent failure. | Roadmap decision | Draft |

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
