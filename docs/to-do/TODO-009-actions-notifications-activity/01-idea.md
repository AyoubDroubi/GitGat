# 01 — Idea

## Original request

Surface CI runs, jobs/logs, notifications and useful team activity directly in GitGat.

## Problem

GitGat needs this capability to complete a safe, modern desktop Git experience without forcing users into command-line Git.

## Product value

GitHub Actions, notifications & team activity closes a required part of the end-to-end GitGat product roadmap.

## Initial requirements

| ID | Requirement | Source | State |
| --- | --- | --- | --- |
| REQ-001 | List GitHub Actions workflow runs with status/conclusion and commit/branch context. | Roadmap decision | Draft |
| REQ-002 | Inspect jobs, steps and failure logs without leaving GitGat. | Roadmap decision | Draft |
| REQ-003 | Allow safe rerun/cancel/dispatch actions when GitHub permissions allow. | Roadmap decision | Draft |
| REQ-004 | Show relevant notifications/mentions/review requests. | Roadmap decision | Draft |
| REQ-005 | Provide concise repository/workspace team activity without becoming a noisy social feed. | Roadmap decision | Draft |

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

STANDARD — chosen based on scope/risk.
