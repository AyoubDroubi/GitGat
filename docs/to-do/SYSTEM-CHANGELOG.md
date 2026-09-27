# Product Change Tracking System — Changelog

Append-only history of modifications to the tracking system itself.

## YYYY-MM-DD — Initial adoption

- Installed the Product Change Tracking System.
- Added global index.
- Added reusable item template.
- Added repository guardrails.

## 2026-09-24 — Governance v3

Added:

- SMALL / STANDARD / MAJOR change tracks.
- Versioned Product Baselines.
- dependency tracking: Depends on / Blocked by / Blocks / Related TODOs.
- requirement IDs and end-to-end Traceability Matrix.
- GREEN / YELLOW / RED item health.
- richer INDEX dashboard with owner, track, baseline, questions, blockers, documentation drift and last activity.
- MAJOR-only risk/migration/release/rollback record.
- explicit principle: Chat is input; repository is memory.


## 2026-09-24 — Governance v4

Added the Operational Audit Layer:

- global append-only Activity Ledger with EVT IDs;
- Deployment Registry with DEP IDs;
- Artifact Registry with ART IDs, including APK/AAB/IPA/build outputs;
- Environment Registry containing only verified deployed state;
- per-TODO `13-operational-log.md`;
- mandatory timestamps/timezones and operational evidence when available;
- failed attempts, retries and rollbacks remain visible;
- rule to log state-changing operations/material verification without flooding history with harmless read-only commands.

## 2026-09-27 — GitGat adoption

- Installed the complete Product Change Tracking System from the ME governance reference.
- Added the Operational Audit Layer and per-TODO operational logs.
- Added repository guardrails: normal development stays on `main`; `production` is reserved and cannot be changed without explicit user authorization.
- Reconstructed the already-merged desktop foundation as TODO-001 without inventing missing review evidence.
- Captured TODO-002 through TODO-013 as the ordered product roadmap.
- Added a master plan ending at verified Windows `GitGat.exe` and `GitGat-Setup-x64.exe` artifacts.
