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
| REQ-001 | Establish unit tests for domain/application parsing and invariants. | Draft | D-001 |
| REQ-002 | Establish integration tests around disposable Git repositories and process adapters. | Draft | D-001 |
| REQ-003 | Run Release builds on supported target OSes/runtimes in CI where practical. | Draft | D-001 |
| REQ-004 | Validate large-repository responsiveness and cancellation behavior. | Draft | D-001 |
| REQ-005 | Review secrets/logging/auth/file-system and destructive-operation security. | Draft | D-001 |
| REQ-006 | Run full product review against TODO acceptance criteria and close blocking defects. | Draft | D-001 |
| REQ-007 | Define a supported-platform matrix with evidence rather than assumptions. | Draft | D-001 |

## In scope

- Establish unit tests for domain/application parsing and invariants.
- Establish integration tests around disposable Git repositories and process adapters.
- Run Release builds on supported target OSes/runtimes in CI where practical.
- Validate large-repository responsiveness and cancellation behavior.
- Review secrets/logging/auth/file-system and destructive-operation security.
- Run full product review against TODO acceptance criteria and close blocking defects.
- Define a supported-platform matrix with evidence rather than assumptions.

## Out of scope

- Unrelated TODO capabilities.
- production changes without explicit authorization.

## Dependencies

**Depends on:** TODO-002 through TODO-011  
**Blocked by:** Feature implementation incomplete  
**Blocks:** See INDEX/master plan  
**Related TODOs:** TODO-013

## UX boundaries

- Keep common workflows understandable to non-expert Git users.
- Advanced detail must remain available without overwhelming normal flows.

## Data/ownership boundaries

- Git remains source of truth for repository history/state.
- GitGat local state must not silently rewrite repository truth.

## Architecture impact

Use existing Clean Architecture boundaries; add ADRs for material cross-cutting decisions.

## Risks

- Cross-platform runtime differences can surface late.
- Large repositories can reveal architectural performance bottlenecks.
- Insufficient destructive-operation tests can hide data-loss bugs.

## Scope approval

Not approved yet.
