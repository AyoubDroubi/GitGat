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
| REQ-001 | Create named workspaces containing one or more repositories. | Draft | D-001 |
| REQ-002 | Switch among repositories/workspaces quickly with recent/favorite behavior. | Draft | D-001 |
| REQ-003 | Aggregate the user's open PRs, requested reviews, mentions and relevant work across repositories. | Draft | D-001 |
| REQ-004 | Show workspace-level health/counts without hiding per-repository truth. | Draft | D-001 |
| REQ-005 | Persist workspace membership and aliases locally without changing Git remotes. | Draft | D-001 |

## In scope

- Create named workspaces containing one or more repositories.
- Switch among repositories/workspaces quickly with recent/favorite behavior.
- Aggregate the user's open PRs, requested reviews, mentions and relevant work across repositories.
- Show workspace-level health/counts without hiding per-repository truth.
- Persist workspace membership and aliases locally without changing Git remotes.

## Out of scope

- Unrelated TODO capabilities.
- production changes without explicit authorization.

## Dependencies

**Depends on:** TODO-002, TODO-005, TODO-006  
**Blocked by:** Repository catalog and collaboration APIs not implemented  
**Blocks:** See INDEX/master plan  
**Related TODOs:** TODO-007, TODO-009

## UX boundaries

- Keep common workflows understandable to non-expert Git users.
- Advanced detail must remain available without overwhelming normal flows.

## Data/ownership boundaries

- Git remains source of truth for repository history/state.
- GitGat local state must not silently rewrite repository truth.

## Architecture impact

Use existing Clean Architecture boundaries; add ADRs for material cross-cutting decisions.

## Risks

- Cross-repo refresh may generate excessive provider calls.
- Stale aggregate counts can reduce trust if freshness is unclear.

## Scope approval

Not approved yet.
