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
| REQ-001 | Detect Git LFS availability and repository LFS configuration. | Draft | D-001 |
| REQ-002 | List current locks with path, owner and lock identifier. | Draft | D-001 |
| REQ-003 | Lock/unlock files from file context actions. | Draft | D-001 |
| REQ-004 | Show lock state and owner directly in Files/Changes. | Draft | D-001 |
| REQ-005 | Support force unlock only behind explicit elevated confirmation. | Draft | D-001 |
| REQ-006 | Guide users to configure lockable patterns without hand-editing when practical. | Draft | D-001 |

## In scope

- Detect Git LFS availability and repository LFS configuration.
- List current locks with path, owner and lock identifier.
- Lock/unlock files from file context actions.
- Show lock state and owner directly in Files/Changes.
- Support force unlock only behind explicit elevated confirmation.
- Guide users to configure lockable patterns without hand-editing when practical.

## Out of scope

- Unrelated product capabilities tracked by other TODOs.
- production changes without explicit authorization.

## Dependencies

**Depends on:** TODO-002, TODO-003  
**Blocked by:** Repository/file workspace not implemented  
**Blocks:** See INDEX/master plan  
**Related TODOs:** TODO-008, TODO-011

## UX boundaries

- Default workflows must stay understandable to non-expert Git users.
- Advanced/risky Git operations must not dominate normal navigation.

## Data/ownership boundaries

- Git repository state remains source-of-truth in Git.
- GitGat local state must not silently rewrite repository history.

## Architecture impact

Use/extend Domain → Application → Infrastructure → Desktop boundaries; create ADR when a material cross-cutting decision appears.

## Risks

- Locking depends on remote/server LFS locking support.
- Stale caches can mislead users about asset ownership.

## Scope approval

Not approved for implementation yet.
