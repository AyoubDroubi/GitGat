# GitGat Repository Instructions

## Branch policy

- `main` is the active development and integration branch.
- All normal implementation, fixes, documentation, TODO updates, tests, and CI changes go directly to `main`.
- Do not create a development branch unless Ayoub explicitly requests one.
- `production` is reserved for a future stable/release decision.
- Do not merge, push, deploy, rebase, reset, or otherwise change `production` without explicit authorization from Ayoub in the current conversation.

## Mandatory Product Change Tracking

GitGat uses the repository-native Product Change Tracking System in `docs/to-do/`.

Before material product work:
1. Read `docs/to-do/README.md` and `docs/to-do/GOVERNANCE.md`.
2. Read `docs/to-do/INDEX.md`.
3. Read the relevant `TODO-NNN-*/` folder.
4. Confirm track, status, health, baseline, dependencies, blockers, and current gate.
5. Do not bypass lifecycle gates.
6. Do not silently expand scope.

Core rule: **Chat is input. Repository is memory.**

Maintain:
- discussion and decision evidence;
- versioned product baselines;
- REQ / AC / implementation / validation / review / release traceability;
- append-only implementation, validation, review, and operational history;
- global `INDEX.md`;
- `operations/ACTIVITY-LEDGER.md`;
- artifact records for generated desktop binaries/installers.

Never invent evidence for builds, tests, reviews, artifacts, deployments, or environments.
