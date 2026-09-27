# 01 — Idea

## Original request

Bring the complete GitHub pull-request loop into GitGat: list, inspect, comment, review, approve/request changes, create and merge.

## Problem

GitGat needs this capability to deliver a complete, approachable desktop Git workflow instead of exposing command-line complexity.

## Product value

Pull requests, reviews, comments & merge moves GitGat toward a single modern workspace for repositories, collaboration and safe team workflows.

## Initial requirements

| ID | Requirement | Source | State |
| --- | --- | --- | --- |
| REQ-001 | List open/draft/merged PRs relevant to the repository. | Roadmap decision | Draft |
| REQ-002 | Show PR metadata, commits and changed files. | Roadmap decision | Draft |
| REQ-003 | Read and create conversation and inline review comments. | Roadmap decision | Draft |
| REQ-004 | Approve, request changes or submit general review comments. | Roadmap decision | Draft |
| REQ-005 | Create/edit PRs with base/head, title/body, reviewers and draft state. | Roadmap decision | Draft |
| REQ-006 | Merge only when provider state and repository rules permit it. | Roadmap decision | Draft |

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
