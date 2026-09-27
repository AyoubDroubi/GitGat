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
| REQ-001 | Open an existing local Git repository from a native folder picker. | Draft | D-001 |
| REQ-002 | Clone a GitHub/local remote repository into a selected destination. | Draft | D-001 |
| REQ-003 | Persist recent repositories and friendly aliases locally. | Draft | D-001 |
| REQ-004 | Show repository identity, current branch, sync state and health at selection time. | Draft | D-001 |
| REQ-005 | Recover gracefully when a repository folder is moved or deleted. | Draft | D-001 |

## In scope

- Open an existing local Git repository from a native folder picker.
- Clone a GitHub/local remote repository into a selected destination.
- Persist recent repositories and friendly aliases locally.
- Show repository identity, current branch, sync state and health at selection time.
- Recover gracefully when a repository folder is moved or deleted.

## Out of scope

- Unrelated product capabilities tracked by other TODOs.
- production changes without explicit authorization.

## Dependencies

**Depends on:** TODO-001  
**Blocked by:** TODO-001 review/closeout  
**Blocks:** See INDEX/master plan  
**Related TODOs:** TODO-003, TODO-008

## UX boundaries

- Default workflows must stay understandable to non-expert Git users.
- Advanced/risky Git operations must not dominate normal navigation.

## Data/ownership boundaries

- Git repository state remains source-of-truth in Git.
- GitGat local state must not silently rewrite repository history.

## Architecture impact

Use/extend Domain → Application → Infrastructure → Desktop boundaries; create ADR when a material cross-cutting decision appears.

## Risks

- Large repo discovery/status calls could block UI.
- Clone credentials and host errors require clear user-safe messages.

## Scope approval

Not approved for implementation yet.
