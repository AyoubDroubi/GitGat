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
| REQ-001 | Fetch, pull and push with ahead/behind visibility. | Draft | D-001 |
| REQ-002 | Create, switch, rename and safely delete branches. | Draft | D-001 |
| REQ-003 | Display local/remote branches and upstream relationships. | Draft | D-001 |
| REQ-004 | Show paged commit history and commit details. | Draft | D-001 |
| REQ-005 | Detect divergence/conflicts and avoid unsafe implicit force pushes. | Draft | D-001 |
| REQ-006 | Keep advanced rebase/reset/cherry-pick out of normal workflow. | Draft | D-001 |

## In scope

- Fetch, pull and push with ahead/behind visibility.
- Create, switch, rename and safely delete branches.
- Display local/remote branches and upstream relationships.
- Show paged commit history and commit details.
- Detect divergence/conflicts and avoid unsafe implicit force pushes.
- Keep advanced rebase/reset/cherry-pick out of normal workflow.

## Out of scope

- Unrelated product capabilities tracked by other TODOs.
- production changes without explicit authorization.

## Dependencies

**Depends on:** TODO-002, TODO-003  
**Blocked by:** Working tree workflow not implemented  
**Blocks:** See INDEX/master plan  
**Related TODOs:** TODO-006, TODO-011

## UX boundaries

- Default workflows must stay understandable to non-expert Git users.
- Advanced/risky Git operations must not dominate normal navigation.

## Data/ownership boundaries

- Git repository state remains source-of-truth in Git.
- GitGat local state must not silently rewrite repository history.

## Architecture impact

Use/extend Domain → Application → Infrastructure → Desktop boundaries; create ADR when a material cross-cutting decision appears.

## Risks

- Force/rewrite semantics can overwrite teammate work.
- Branch switching with uncommitted changes needs explicit recovery paths.

## Scope approval

Not approved for implementation yet.
