# 01 — Idea

## Original request

Let users open, clone, remember, search and switch among local Git repositories without needing Git command knowledge.

## Problem

GitGat needs this capability to deliver a complete, approachable desktop Git workflow instead of exposing command-line complexity.

## Product value

Repository onboarding, clone & local catalog moves GitGat toward a single modern workspace for repositories, collaboration and safe team workflows.

## Initial requirements

| ID | Requirement | Source | State |
| --- | --- | --- | --- |
| REQ-001 | Open an existing local Git repository from a native folder picker. | Roadmap decision | Draft |
| REQ-002 | Clone a GitHub/local remote repository into a selected destination. | Roadmap decision | Draft |
| REQ-003 | Persist recent repositories and friendly aliases locally. | Roadmap decision | Draft |
| REQ-004 | Show repository identity, current branch, sync state and health at selection time. | Roadmap decision | Draft |
| REQ-005 | Recover gracefully when a repository folder is moved or deleted. | Roadmap decision | Draft |

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

STANDARD — scope/risk requires this governance depth.
