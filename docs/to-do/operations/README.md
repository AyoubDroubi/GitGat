# Operational Audit Layer

This folder records **what actually happened to the project**, not only what was planned.

The rule is:

> Every meaningful state-changing operation and every material verification result must leave durable evidence.

Do not log harmless read-only inspection such as opening a file or listing a directory. Log actions/results that change project state or materially prove it.

## Global records

- [ACTIVITY-LEDGER.md](./ACTIVITY-LEDGER.md) — append-only chronological event stream.
- [DEPLOYMENT-REGISTRY.md](./DEPLOYMENT-REGISTRY.md) — every deployment attempt/result.
- [ARTIFACT-REGISTRY.md](./ARTIFACT-REGISTRY.md) — produced APK/AAB/IPA/packages/images/releases and other distributable artifacts.
- [ENVIRONMENTS.md](./ENVIRONMENTS.md) — known environments and their last verified deployed state.

## Mandatory event classes

Record at minimum:

- product/governance decision accepted, rejected, or superseded;
- TODO status/track/health/baseline transition;
- branch creation used for tracked work;
- material commit batch;
- PR opened, updated materially, approved, merged, closed, or reverted;
- build started/completed/failed when it produces meaningful evidence;
- test/analyze/lint verification result;
- artifact generated or generation failed;
- APK/AAB/IPA/package/release artifact created;
- deployment started/completed/failed;
- production/staging environment change;
- DB/schema migration applied/failed/rolled back;
- config/infrastructure change;
- release published;
- rollback/revert/hotfix;
- material operational failure or recovery;
- independent review completion.

## Event identity

Use stable IDs:

- `EVT-YYYYMMDD-NNN` — general activity.
- `DEP-YYYYMMDD-NNN` — deployment.
- `ART-YYYYMMDD-NNN` — artifact.

## Minimum event evidence

Each event should capture when applicable:

- timestamp with timezone;
- actor/source;
- event type;
- result: STARTED / SUCCEEDED / FAILED / CANCELLED / ROLLED_BACK;
- linked TODO(s);
- product baseline;
- branch;
- commit SHA;
- PR/release/workflow/build identifier;
- environment;
- artifact identifier/path/link;
- concise description;
- evidence;
- follow-up.

## Important rules

- Failed operations stay in history.
- Retry attempts get new event IDs.
- Never rewrite history to make it look clean.
- Do not claim a deploy/build/artifact exists without evidence.
- If exact time is unavailable, say so; do not invent it.
- Backfill history only from verifiable repository/tool evidence.
- A deployment and its produced artifact may have separate records linked by IDs.
