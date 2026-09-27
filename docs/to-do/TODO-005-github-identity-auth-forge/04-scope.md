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
| REQ-001 | Connect to GitHub using a user-authorized flow without storing secrets in SQLite. | Draft | D-001 |
| REQ-002 | Store credentials only in OS secure storage or provider CLI secure storage. | Draft | D-001 |
| REQ-003 | Resolve repository owner/name/host from remotes reliably. | Draft | D-001 |
| REQ-004 | Expose provider capabilities through an IGitForge-style application boundary. | Draft | D-001 |
| REQ-005 | Handle expired/revoked auth with reconnect UX, never silent failure. | Draft | D-001 |

## In scope

- Connect to GitHub using a user-authorized flow without storing secrets in SQLite.
- Store credentials only in OS secure storage or provider CLI secure storage.
- Resolve repository owner/name/host from remotes reliably.
- Expose provider capabilities through an IGitForge-style application boundary.
- Handle expired/revoked auth with reconnect UX, never silent failure.

## Out of scope

- Unrelated product capabilities tracked by other TODOs.
- production changes without explicit authorization.

## Dependencies

**Depends on:** TODO-002  
**Blocked by:** Repository identity/catalog not implemented  
**Blocks:** See INDEX/master plan  
**Related TODOs:** TODO-006, TODO-008, TODO-009

## UX boundaries

- Default workflows must stay understandable to non-expert Git users.
- Advanced/risky Git operations must not dominate normal navigation.

## Data/ownership boundaries

- Git repository state remains source-of-truth in Git.
- GitGat local state must not silently rewrite repository history.

## Architecture impact

Use/extend Domain → Application → Infrastructure → Desktop boundaries; create ADR when a material cross-cutting decision appears.

## Risks

- OAuth/device-flow configuration may need a registered GitHub app.
- Credential leakage through logs/errors is unacceptable.

## Scope approval

Not approved for implementation yet.
