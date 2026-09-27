# Agent Guardrail — Copy into target project instructions

Use/adapt the block below in `AGENTS.md`, `CLAUDE.md`, or equivalent repository instructions.

---

## Mandatory Product Change Tracking System

`docs/to-do/README.md` defines the required A-to-Z workflow.
`docs/to-do/INDEX.md` is the global status/health dashboard.

Every product change must have a `docs/to-do/TODO-NNN-short-name/` folder.

### Before any product code change

1. Read the governance README and relevant TODO folder.
2. Confirm the item exists in INDEX.
3. Confirm its track: `SMALL`, `STANDARD`, or `MAJOR`.
4. Confirm current status permits the requested action.
5. Confirm the current approved product baseline.
6. Check dependencies, blockers, open questions, health, and documentation drift.
7. Record any new meaningful user comment before coding.
8. Update decisions/scope/criteria/traceability if the comment changes behavior.
9. Never silently expand scope.

### Track enforcement

- `SMALL`: lightweight documentation is allowed, but discussion, decision/approval, criteria, implementation, validation, review, traceability, and closeout evidence remain mandatory.
- `STANDARD`: full normal lifecycle.
- `MAJOR`: full lifecycle plus explicit risk, migration/backward compatibility, rollout/release, rollback, and architecture evidence where applicable.

### Status enforcement

- `CAPTURED`, `DISCUSSION`, `APPROVED`: no implementation.
- `READY`: prepared; implementation begins only after explicit start authorization is logged.
- `IN_PROGRESS`: implementation allowed only within the approved baseline.
- `VALIDATING`: validation fixes only.
- `IN_REVIEW`: review-related fixes only.
- `CHANGES_REQUESTED`: documented requested fixes only.
- `READY_TO_MERGE`: no new scope; merge still requires applicable authorization.
- `DONE`: closed.
- `ON_HOLD` / `REJECTED`: no implementation.

### Mandatory tracking

- Append meaningful comments to `02-discussion-log.md`; do not dump raw chat transcripts.
- Record decisions in `03-decisions.md`; discussion is not approval.
- Maintain versioned requirements/scope in `04-scope.md`.
- Maintain dependencies/blockers/related TODOs in item README and INDEX when material.
- Every material requirement gets a `REQ-` ID and is mapped in `11-traceability.md`.
- Every implementation batch states which product baseline it implements.
- Validation claims require evidence in `08-validation.md`.
- Review findings/fixes require evidence in `09-review.md`.
- MAJOR items require `12-risk-migration-release.md`.
- Every status transition requires evidence.
- Keep item README and INDEX synchronized.
- If scope changes after approval, pause, create/update a baseline, and re-run readiness.
- Logs and superseded baselines/decisions are historical records; do not erase them.

### Operational audit

For every meaningful state-changing operation or material verification:

- append an `EVT-` row to `docs/to-do/operations/ACTIVITY-LEDGER.md`;
- append the linked TODO's `13-operational-log.md` when applicable;
- deployments also go to `DEPLOYMENT-REGISTRY.md`;
- produced/failed APKs, AABs, IPAs, packages, images, or other release artifacts go to `ARTIFACT-REGISTRY.md`;
- successful verified deployment state updates `ENVIRONMENTS.md`;
- record exact timestamp + timezone when available;
- record branch, commit, PR/workflow/build, environment, artifact/deployment IDs, result, and evidence when applicable;
- failed attempts and retries remain visible as separate events;
- never claim a build/deploy/artifact succeeded without evidence;
- do not log harmless read-only inspection merely to create noise.

### Health

Use `GREEN`, `YELLOW`, or `RED` based on blockers, evidence completeness, dependency state, and documentation/code drift. Health never replaces lifecycle status.

> Chat is input. Repository is memory.

Do not bypass this system because a change is small, easy, urgent, or already discussed in chat.

---
