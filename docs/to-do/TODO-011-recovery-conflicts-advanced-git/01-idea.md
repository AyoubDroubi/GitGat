# 01 — Idea

## Original request

Add conflict handling, stash/recovery and advanced Git operations only after the safe daily workflow is stable.

## Problem

GitGat needs this capability to complete a safe, modern desktop Git experience without forcing users into command-line Git.

## Product value

Recovery, conflicts & Advanced Git closes a required part of the end-to-end GitGat product roadmap.

## Initial requirements

| ID | Requirement | Source | State |
| --- | --- | --- | --- |
| REQ-001 | Detect merge/rebase/cherry-pick/revert conflict states and guide resolution. | Roadmap decision | Draft |
| REQ-002 | Provide stash create/list/apply/pop/drop with clear recovery semantics. | Roadmap decision | Draft |
| REQ-003 | Expose rebase, cherry-pick and reset under Advanced with previews/confirmation. | Roadmap decision | Draft |
| REQ-004 | Support worktree management for parallel branch folders. | Roadmap decision | Draft |
| REQ-005 | Expose reflog/recovery tools for lost commits/stashes where practical. | Roadmap decision | Draft |
| REQ-006 | Maintain an operation journal for risky compound operations and interruptions. | Roadmap decision | Draft |

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
