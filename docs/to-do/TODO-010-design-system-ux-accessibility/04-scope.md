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
| REQ-001 | Own GitGat color, typography, spacing, radius, icon and motion tokens independently of SukiUI defaults. | Draft | D-001 |
| REQ-002 | Use consistent navigation, cards, rows, menus, dialogs, empty/loading/error states across features. | Draft | D-001 |
| REQ-003 | Support keyboard navigation, visible focus, screen-reader labels and non-color-only status. | Draft | D-001 |
| REQ-004 | Support dark theme first with a path to light/high-contrast variants. | Draft | D-001 |
| REQ-005 | Keep information density practical for Git while remaining understandable to non-experts. | Draft | D-001 |
| REQ-006 | Respect platform conventions for titlebar, dialogs, drag/drop and native file interactions. | Draft | D-001 |

## In scope

- Own GitGat color, typography, spacing, radius, icon and motion tokens independently of SukiUI defaults.
- Use consistent navigation, cards, rows, menus, dialogs, empty/loading/error states across features.
- Support keyboard navigation, visible focus, screen-reader labels and non-color-only status.
- Support dark theme first with a path to light/high-contrast variants.
- Keep information density practical for Git while remaining understandable to non-experts.
- Respect platform conventions for titlebar, dialogs, drag/drop and native file interactions.

## Out of scope

- Unrelated TODO capabilities.
- production changes without explicit authorization.

## Dependencies

**Depends on:** TODO-002, TODO-003  
**Blocked by:** Core product surfaces need real data/workflows for final polish  
**Blocks:** See INDEX/master plan  
**Related TODOs:** TODO-006, TODO-007, TODO-008, TODO-009

## UX boundaries

- Keep common workflows understandable to non-expert Git users.
- Advanced detail must remain available without overwhelming normal flows.

## Data/ownership boundaries

- Git remains source of truth for repository history/state.
- GitGat local state must not silently rewrite repository truth.

## Architecture impact

Use existing Clean Architecture boundaries; add ADRs for material cross-cutting decisions.

## Risks

- Over-styling can reduce information density/usability.
- SukiUI upgrades may affect control templates; GitGat wrapper components must isolate this.

## Scope approval

Not approved yet.
