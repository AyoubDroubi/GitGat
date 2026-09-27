# 01 — Idea

## Original request

Provide a safe visual working-tree experience for file changes, diffs, staging, discard and commits.

## Problem

GitGat needs this capability to deliver a complete, approachable desktop Git workflow instead of exposing command-line complexity.

## Product value

Files, changes, staging & commit workflow moves GitGat toward a single modern workspace for repositories, collaboration and safe team workflows.

## Initial requirements

| ID | Requirement | Source | State |
| --- | --- | --- | --- |
| REQ-001 | Display modified, added, deleted, renamed and untracked files. | Roadmap decision | Draft |
| REQ-002 | Show readable text diffs and appropriate binary/image states. | Roadmap decision | Draft |
| REQ-003 | Stage/unstage individual files and all changes. | Roadmap decision | Draft |
| REQ-004 | Create commits with title/body and validation. | Roadmap decision | Draft |
| REQ-005 | Discard destructive changes only behind explicit confirmation/recovery-safe behavior. | Roadmap decision | Draft |
| REQ-006 | Keep Git operations off the UI thread and refresh state atomically. | Roadmap decision | Draft |

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
