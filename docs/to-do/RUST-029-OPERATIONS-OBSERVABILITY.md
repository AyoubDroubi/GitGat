# RUST-029 - Admin Operations, Observability and Support Tooling

Status: TODO

## Operational views
- provider connection health;
- webhook delivery health;
- reconciliation queue;
- failed jobs;
- policy evaluation latency/errors;
- database health;
- version/build info.

## Telemetry
- structured logs;
- correlation IDs;
- metrics;
- traces where useful;
- secret/token redaction;
- provider request IDs retained when safe.

## Support tools
- replay webhook;
- retry failed reconciliation;
- re-evaluate PR policy;
- refresh repository capabilities;
- export diagnostics;
- no hidden destructive repair actions.

## SLO targets
Define before production:
- API availability;
- webhook processing latency;
- policy check latency;
- reconciliation freshness.

## Acceptance
Operators can diagnose provider/policy failures without database access or raw server shell access.
