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
| REQ-001 | List open/draft/merged PRs relevant to the repository. | Draft | D-001 |
| REQ-002 | Show PR metadata, commits and changed files. | Draft | D-001 |
| REQ-003 | Read and create conversation and inline review comments. | Draft | D-001 |
| REQ-004 | Approve, request changes or submit general review comments. | Draft | D-001 |
| REQ-005 | Create/edit PRs with base/head, title/body, reviewers and draft state. | Draft | D-001 |
| REQ-006 | Merge only when provider state and repository rules permit it. | Draft | D-001 |

## In scope

- List open/draft/merged PRs relevant to the repository.
- Show PR metadata, commits and changed files.
- Read and create conversation and inline review comments.
- Approve, request changes or submit general review comments.
- Create/edit PRs with base/head, title/body, reviewers and draft state.
- Merge only when provider state and repository rules permit it.

## Out of scope

- Unrelated product capabilities tracked by other TODOs.
- production changes without explicit authorization.

## Dependencies

**Depends on:** TODO-003, TODO-004, TODO-005  
**Blocked by:** GitHub connection and core Git workflows not implemented  
**Blocks:** See INDEX/master plan  
**Related TODOs:** TODO-008, TODO-009

## UX boundaries

- Default workflows must stay understandable to non-expert Git users.
- Advanced/risky Git operations must not dominate normal navigation.

## Data/ownership boundaries

- Git repository state remains source-of-truth in Git.
- GitGat local state must not silently rewrite repository history.

## Architecture impact

Use/extend Domain → Application → Infrastructure → Desktop boundaries; create ADR when a material cross-cutting decision appears.

## Risks

- Review APIs have subtle line/position semantics.
- Stale mergeability/review state can cause confusing actions.

## Scope approval

Not approved for implementation yet.
