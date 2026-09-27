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
