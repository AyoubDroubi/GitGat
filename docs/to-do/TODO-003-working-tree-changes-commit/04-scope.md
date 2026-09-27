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
| REQ-001 | Display modified, added, deleted, renamed and untracked files. | Draft | D-001 |
| REQ-002 | Show readable text diffs and appropriate binary/image states. | Draft | D-001 |
| REQ-003 | Stage/unstage individual files and all changes. | Draft | D-001 |
| REQ-004 | Create commits with title/body and validation. | Draft | D-001 |
| REQ-005 | Discard destructive changes only behind explicit confirmation/recovery-safe behavior. | Draft | D-001 |
| REQ-006 | Keep Git operations off the UI thread and refresh state atomically. | Draft | D-001 |

## In scope

- Display modified, added, deleted, renamed and untracked files.
- Show readable text diffs and appropriate binary/image states.
- Stage/unstage individual files and all changes.
- Create commits with title/body and validation.
- Discard destructive changes only behind explicit confirmation/recovery-safe behavior.
- Keep Git operations off the UI thread and refresh state atomically.

## Out of scope

- Unrelated product capabilities tracked by other TODOs.
- production changes without explicit authorization.

## Dependencies

**Depends on:** TODO-002  
**Blocked by:** Repository workspace not implemented  
**Blocks:** See INDEX/master plan  
**Related TODOs:** TODO-004, TODO-007, TODO-011

## UX boundaries

- Default workflows must stay understandable to non-expert Git users.
- Advanced/risky Git operations must not dominate normal navigation.

## Data/ownership boundaries

- Git repository state remains source-of-truth in Git.
- GitGat local state must not silently rewrite repository history.

## Architecture impact

Use/extend Domain → Application → Infrastructure → Desktop boundaries; create ADR when a material cross-cutting decision appears.

## Risks

- Incorrect diff/status parsing can lose user trust.
- Destructive operations can cause data loss if guardrails fail.

## Scope approval

Not approved for implementation yet.
