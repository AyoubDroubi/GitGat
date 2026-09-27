# Product Change Tracking System

A reusable, repository-native governance system for tracking the complete life of a product change.

This is not only a backlog. It is a **Product Change Ledger + Engineering Governance + Audit Trail**.

It records the full journey:

`Idea → Discussion → Decision → Approval → Scope → Acceptance Criteria → Implementation Plan → Start Authorization → Implementation → Validation → Review → Fixes/Re-review → Ready to Merge → Merge/Release → Done`

The system is designed to be copied into any software repository and used by humans, AI coding agents, or both.

> **Chat is input. The repository is memory.**
>
> Conversations can start or refine a change, but every meaningful requirement, decision, scope change, approval, implementation step, validation result, review comment, and closeout fact must become durable repository evidence.

---

## Core rule

No product-facing change should be implemented before it is registered and tracked.

Examples:

- new features;
- UX changes;
- widgets;
- integrations;
- navigation behavior;
- cross-feature connections;
- product-facing data-model changes;
- significant behavior changes;
- follow-up changes discovered during implementation or review.

Small, obvious, urgent, or chat-approved changes are not exceptions. They may use a lighter **SMALL** track, but they still must be tracked.

---

## Change tracks

The system has three tracks so governance depth matches change risk.

### SMALL

Use for low-risk, localized changes with no meaningful architecture/data/migration impact.

Examples:

- copy or label behavior;
- a small interaction refinement;
- a localized UI improvement;
- a contained bug fix that changes product behavior.

Minimum governance:

- idea;
- meaningful discussion/comments;
- explicit decision/approval;
- testable acceptance criteria;
- implementation log;
- validation;
- review;
- closeout.

Scope/implementation-plan files may be marked **Not applicable — SMALL track** when the change is truly self-contained, but the files remain in the folder so the audit shape is consistent.

### STANDARD

Default track for normal product work.

Use the complete lifecycle and all standard records.

### MAJOR

Use when the change has elevated risk or breadth.

Examples:

- architecture changes;
- auth/identity changes;
- significant data-model changes;
- migrations/backfills;
- cross-system integrations;
- release-sensitive behavior;
- large cross-feature workflows.

MAJOR requires the complete STANDARD track plus:

- explicit risk analysis;
- migration/backward-compatibility plan;
- rollout/release strategy;
- rollback plan;
- architecture/ADR references when applicable;
- stronger traceability and review evidence.

Track changes themselves are decisions and must be logged.

---

## One folder per change

Each change gets a permanent folder:

`docs/to-do/TODO-NNN-short-name/`

Recommended files:

1. `README.md` — live status, track, health, dependencies, baseline.
2. `01-idea.md` — original idea/problem/value.
3. `02-discussion-log.md` — append-only meaningful comments.
4. `03-decisions.md` — explicit decisions and approval evidence.
5. `04-scope.md` — versioned requirements/scope baseline, dependencies, boundaries.
6. `05-acceptance-criteria.md` — testable completion conditions.
7. `06-implementation-plan.md` — implementation plan.
8. `07-implementation-log.md` — append-only implementation history.
9. `08-validation.md` — builds/tests/manual checks and evidence.
10. `09-review.md` — review rounds, comments, fixes, re-review.
11. `10-release-closeout.md` — PR/merge/release evidence and final closeout.
12. `11-traceability.md` — requirement-to-decision-to-code-to-test-to-review mapping.
13. `12-risk-migration-release.md` — MAJOR risk/migration/rollout record; optional/N/A otherwise.

Do not collapse the history into one large document if traceability matters.

---

## Versioned product baseline

Every tracked item has a **Product Baseline**.

Examples:

- `v0-draft` — discussion is still moving;
- `v1-approved` — first approved scope;
- `v2-approved` — approved scope changed later.

The current baseline is shown in the item README and version history is stored in `04-scope.md`.

Rules:

- approved scope is never silently overwritten;
- a material requirement/scope change creates a new baseline;
- implementation log entries state which baseline they implement;
- acceptance criteria and traceability must match the current approved baseline;
- superseded baselines remain visible in history.

---

## Dependencies

Every item records:

- **Depends on** — work that must exist first;
- **Blocked by** — active blocker preventing progress;
- **Blocks** — work waiting on this item;
- **Related TODOs** — connected but not blocking items.

Dependency changes are meaningful updates and must be reflected in the item README and global index when they affect readiness/health.

---

## Traceability

Every material requirement gets an ID such as `REQ-001`.

Use `11-traceability.md` to map:

`Requirement → Decision → Acceptance Criterion → Implementation → Validation → Review → Release`

Example:

`REQ-003 → D-002 → AC-005 → IL-004 → V-011 → R-002 → release evidence`

This makes it possible to answer whether every approved requirement was actually implemented and verified.

For SMALL changes, the matrix may be short. For MAJOR changes, it is mandatory and complete.

---

## Status lifecycle

| Status | Meaning | Product code allowed? |
| --- | --- | ---: |
| `CAPTURED` | Idea is recorded. | No |
| `DISCUSSION` | Behavior/scope is being discussed. | No |
| `APPROVED` | Product direction is explicitly accepted. | No |
| `READY` | Scope, criteria, plan, dependencies, and required risk work are complete. | No, until explicit start authorization |
| `IN_PROGRESS` | Implementation was explicitly started. | Yes, within approved baseline |
| `VALIDATING` | Implementation is being verified. | Validation fixes only |
| `IN_REVIEW` | Independent review is running. | Review fixes only |
| `CHANGES_REQUESTED` | Review found required issues. | Documented fixes only |
| `READY_TO_MERGE` | Criteria, validation, traceability, and review all pass. | No new scope |
| `DONE` | Authorized merge/release/closeout is complete. | Closed |
| `ON_HOLD` | Intentionally paused. | No |
| `REJECTED` | Explicitly declined. | No |

---

## Health model

The global index and item README use an objective health signal.

### GREEN

- current lifecycle gate is internally consistent;
- no active blockers;
- required documents/evidence for the current stage are present;
- no known documentation drift.

### YELLOW

- open product questions remain;
- dependencies are pending;
- evidence/docs need synchronization;
- item is waiting on a normal external decision;
- non-blocking drift exists.

### RED

- active blocker prevents progress;
- code/scope/evidence materially disagree;
- required gate evidence is missing;
- unapproved scope was implemented;
- validation/review has unresolved blocking failures.

Health is not a substitute for status. It explains whether the item is healthy **within** its current status.

---

## A-to-Z flow

### 1. Capture

When a new idea or material product change arrives:

- create a TODO folder;
- choose an initial track;
- add it to the global index;
- capture original intent in `01-idea.md`;
- register initial requirements with `REQ-` IDs when possible;
- append the meaningful comment to `02-discussion-log.md`;
- set status to `CAPTURED` or `DISCUSSION`;
- do not implement.

### 2. Discuss

Record meaningful comments, not raw transcripts.

Meaningful comments include:

- new requirements;
- rejected directions;
- UX preferences;
- naming choices;
- scope changes;
- edge cases;
- data/architecture constraints;
- priority changes;
- dependency changes;
- explicit approval/rejection;
- track changes.

Each discussion entry should record:

- date/time when available;
- actor/source;
- concise comment summary;
- impact;
- affected requirement/baseline;
- unresolved question or linked decision.

### 3. Decide

When discussion produces a decision:

- assign a decision ID such as `D-001`;
- record the decision;
- record alternatives considered;
- record the reason;
- link relevant discussion and requirement IDs;
- record approval evidence;
- update scope/criteria/traceability as affected.

Discussion is not approval.

### 4. Approve

Move to `APPROVED` only after explicit product approval.

Create/update the approved product baseline.

Approval accepts the direction. It still does not authorize coding.

### 5. Prepare

Before `READY`:

- resolve/defer product questions;
- verify dependencies/blockers;
- finish versioned scope;
- define testable acceptance criteria;
- finish implementation plan when required by track;
- complete risk/migration/release analysis when required;
- identify ADR/architecture needs;
- update traceability.

### 6. Ready

Move to `READY` only when the readiness checklist passes for the selected track.

Do not silently infer readiness from a chat conversation.

### 7. Start

Move to `IN_PROGRESS` only after explicit start authorization.

Log the authorization and current baseline before product code changes.

### 8. Implement

For each material change batch:

- append to implementation log;
- state the baseline implemented;
- list touched areas/files;
- record commit/PR evidence when available;
- update traceability;
- record deviations and newly discovered issues.

If product scope changes, pause and return to discussion/decision/readiness.

### 9. Validate

Record every meaningful check:

- build;
- tests;
- lint/analyze;
- migrations;
- API checks;
- manual scenarios;
- device/platform checks;
- regression checks.

Failures remain in history. Record fixes and re-runs as new entries and map evidence into traceability.

### 10. Review

Run independent review and record:

- reviewer;
- baseline/diff/scope reviewed;
- findings;
- severity;
- required fixes;
- fix evidence;
- re-validation;
- re-review result.

If required findings exist, use `CHANGES_REQUESTED`.

### 11. Ready to merge

Use `READY_TO_MERGE` only when:

- approved acceptance criteria pass;
- validation passes;
- review approves;
- traceability has no unexplained requirement gaps;
- no blocking issues remain;
- docs/evidence are synchronized;
- scope has not drifted;
- MAJOR release/rollback gates are ready when applicable.

### 12. Merge/release/close

After applicable authorization:

- record PR;
- record merge commit;
- record target branch;
- record release/build/deploy evidence;
- close traceability rows;
- record follow-ups;
- update local README;
- update global INDEX;
- close as `DONE`.

---

## Non-negotiable integrity rules

- Discussion, implementation, validation, and review logs are append-only histories.
- Correct history by appending; do not erase failures or earlier decisions.
- Superseded decisions and baselines remain visible and link to replacements.
- Every status transition needs evidence.
- Every material requirement is traceable.
- Every implementation claim should have evidence when available.
- No silent scope creep.
- A new requirement after approval can force a new baseline and a return to discussion/readiness.
- Product tracking does not replace ADRs, architecture docs, tests, or code review.
- Repository records capture **meaningful decisions and evidence**, not raw conversation dumps.

---

## Operational audit layer

Product tracking answers **why/what** changed. The operational layer records **what actually happened**.

Target projects maintain:

- `docs/to-do/operations/ACTIVITY-LEDGER.md`
- `docs/to-do/operations/DEPLOYMENT-REGISTRY.md`
- `docs/to-do/operations/ARTIFACT-REGISTRY.md`
- `docs/to-do/operations/ENVIRONMENTS.md`
- per-TODO `13-operational-log.md`

Every meaningful state-changing operation or material verification result must be recorded with timestamp/timezone and evidence when available.

Examples include decisions/status transitions, branches, material commits, PR actions, builds, tests, APK/AAB/IPA generation, deployments, releases, migrations, infrastructure/config changes, failures, retries, rollbacks, and review completion.

Read-only inspection is not logged merely for noise. The rule is to log **state changes and material proof**, not every command.

Deployments and artifacts receive stable IDs and dedicated registry rows. Failed attempts remain visible. Current environment state is updated only from verified deployment evidence.



This folder includes:

- [ADOPTION.md](./ADOPTION.md) — how to install it in another repository.
- [AGENT-GUARDRAIL.md](./AGENT-GUARDRAIL.md) — instructions for AGENTS/CLAUDE/project rules.
- [BOOTSTRAP-PROMPT.md](./BOOTSTRAP-PROMPT.md) — reusable installation prompt.
- [reference/](./reference/) — canonical target-project files/templates.

The target project may rename `docs/to-do`, but the lifecycle, track, audit, baseline, dependency, traceability, and health principles should remain intact.
