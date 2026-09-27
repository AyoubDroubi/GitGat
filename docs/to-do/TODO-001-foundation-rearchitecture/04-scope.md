# 04 — Scope / Product Baseline

## Current baseline

`v1-approved`

## Baseline history

| Baseline | State | Date | Trigger | Approval evidence |
| --- | --- | --- | --- | --- |
| v1-approved | Approved/reconstructed | 2026-09-27 | Accepted architecture implemented | User selected and started approach |

## Requirements in current baseline

| ID | Requirement | State | Source/Decision |
| --- | --- | --- | --- |
| REQ-001 | Use .NET 10 as the application runtime and C# as the primary language. | Approved | D-001 |
| REQ-002 | Use Avalonia 12 + SukiUI 7 with a GitGat-owned design system. | Approved | D-001 |
| REQ-003 | Separate Domain, Application, Infrastructure and Desktop concerns. | Approved | D-001 |
| REQ-004 | Use installed System Git behind application interfaces; UI must not shell out directly. | Approved | D-001 |
| REQ-005 | Build the complete solution successfully in CI on Windows. | Approved | D-001 |

## In scope

- Use .NET 10 as the application runtime and C# as the primary language.
- Use Avalonia 12 + SukiUI 7 with a GitGat-owned design system.
- Separate Domain, Application, Infrastructure and Desktop concerns.
- Use installed System Git behind application interfaces; UI must not shell out directly.
- Build the complete solution successfully in CI on Windows.

## Out of scope

- Unrelated product capabilities tracked by other TODOs.
- production changes without explicit authorization.

## Dependencies

**Depends on:** None  
**Blocked by:** Independent review and governance closeout  
**Blocks:** See INDEX/master plan  
**Related TODOs:** TODO-012, TODO-013

## UX boundaries

- Default workflows must stay understandable to non-expert Git users.
- Advanced/risky Git operations must not dominate normal navigation.

## Data/ownership boundaries

- Git repository state remains source-of-truth in Git.
- GitGat local state must not silently rewrite repository history.

## Architecture impact

Use/extend Domain → Application → Infrastructure → Desktop boundaries; create ADR when a material cross-cutting decision appears.

## Risks

- Framework/package compatibility drift.
- Cross-platform behavior is not yet validated beyond build-level evidence.

## Scope approval

Reconstructed as approved from explicit implementation authorization; independent review still pending.
