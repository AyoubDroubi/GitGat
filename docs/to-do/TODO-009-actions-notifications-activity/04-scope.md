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
| REQ-001 | List GitHub Actions workflow runs with status/conclusion and commit/branch context. | Draft | D-001 |
| REQ-002 | Inspect jobs, steps and failure logs without leaving GitGat. | Draft | D-001 |
| REQ-003 | Allow safe rerun/cancel/dispatch actions when GitHub permissions allow. | Draft | D-001 |
| REQ-004 | Show relevant notifications/mentions/review requests. | Draft | D-001 |
| REQ-005 | Provide concise repository/workspace team activity without becoming a noisy social feed. | Draft | D-001 |

## In scope

- List GitHub Actions workflow runs with status/conclusion and commit/branch context.
- Inspect jobs, steps and failure logs without leaving GitGat.
- Allow safe rerun/cancel/dispatch actions when GitHub permissions allow.
- Show relevant notifications/mentions/review requests.
- Provide concise repository/workspace team activity without becoming a noisy social feed.

## Out of scope

- Unrelated TODO capabilities.
- production changes without explicit authorization.

## Dependencies

**Depends on:** TODO-005, TODO-006  
**Blocked by:** GitHub forge integration not implemented  
**Blocks:** See INDEX/master plan  
**Related TODOs:** TODO-008

## UX boundaries

- Keep common workflows understandable to non-expert Git users.
- Advanced detail must remain available without overwhelming normal flows.

## Data/ownership boundaries

- Git remains source of truth for repository history/state.
- GitGat local state must not silently rewrite repository truth.

## Architecture impact

Use existing Clean Architecture boundaries; add ADRs for material cross-cutting decisions.

## Risks

- Actions logs can be large.
- Notification APIs may differ in freshness/permissions.
- Too much activity can make the home screen noisy.

## Scope approval

Not approved yet.
