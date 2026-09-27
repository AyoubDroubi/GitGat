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
| REQ-001 | Detect merge/rebase/cherry-pick/revert conflict states and guide resolution. | Draft | D-001 |
| REQ-002 | Provide stash create/list/apply/pop/drop with clear recovery semantics. | Draft | D-001 |
| REQ-003 | Expose rebase, cherry-pick and reset under Advanced with previews/confirmation. | Draft | D-001 |
| REQ-004 | Support worktree management for parallel branch folders. | Draft | D-001 |
| REQ-005 | Expose reflog/recovery tools for lost commits/stashes where practical. | Draft | D-001 |
| REQ-006 | Maintain an operation journal for risky compound operations and interruptions. | Draft | D-001 |

## In scope

- Detect merge/rebase/cherry-pick/revert conflict states and guide resolution.
- Provide stash create/list/apply/pop/drop with clear recovery semantics.
- Expose rebase, cherry-pick and reset under Advanced with previews/confirmation.
- Support worktree management for parallel branch folders.
- Expose reflog/recovery tools for lost commits/stashes where practical.
- Maintain an operation journal for risky compound operations and interruptions.

## Out of scope

- Unrelated TODO capabilities.
- production changes without explicit authorization.

## Dependencies

**Depends on:** TODO-003, TODO-004, TODO-007  
**Blocked by:** Core working tree/branch/lock workflows not implemented  
**Blocks:** See INDEX/master plan  
**Related TODOs:** TODO-012

## UX boundaries

- Keep common workflows understandable to non-expert Git users.
- Advanced detail must remain available without overwhelming normal flows.

## Data/ownership boundaries

- Git remains source of truth for repository history/state.
- GitGat local state must not silently rewrite repository truth.

## Architecture impact

Use existing Clean Architecture boundaries; add ADRs for material cross-cutting decisions.

## Risks

- History-changing operations can permanently lose work.
- Platform/process interruptions can leave repositories mid-operation.
- A false sense of recovery safety is dangerous.

## Scope approval

Not approved yet.
