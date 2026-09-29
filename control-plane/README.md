# GitGat Control Plane

The Control Plane is GitGat's organization governance layer. It is **not** the authoritative file-lock server. Active lock truth remains in the Git LFS service attached to GitHub or Azure DevOps.

## Current foundation

- Rust + Axum API
- PostgreSQL + SQLx migrations
- liveness: `GET /health/live`
- readiness: `GET /health/ready`
- service metadata: `GET /api/v1/meta`
- initial persistence model for organizations, identities, repositories, provider connections, policies, lock observations, exceptions, force-unlock requests, approvals, audit, webhooks and provider health

RBAC enforcement, provider webhooks, force-unlock execution and policy mutations are intentionally **not enabled yet**. Their TODO dependency gates must be completed first.

## Local development

1. Start PostgreSQL:
   `docker compose -f control-plane/docker-compose.yml up -d postgres`
2. Export:
   `GITGAT_DATABASE_URL=postgres://gitgat:gitgat@127.0.0.1:5432/gitgat`
3. Run:
   `cargo run --manifest-path control-plane/api/Cargo.toml`

The API applies migrations at startup.

## Safety invariants

- provider Git LFS remains authoritative for active locks;
- no desktop-user PATs are stored here;
- a force unlock must eventually re-verify the live provider lock before execution;
- all sensitive governance mutations will require RBAC and durable audit before being enabled.


## Authentication and tenant boundary

Administrative APIs are protected by the Control Plane authentication middleware.

Production:
- configure `GITGAT_OIDC_ISSUER`;
- configure `GITGAT_OIDC_AUDIENCE`;
- configure `GITGAT_OIDC_JWKS_URL`;
- bearer JWTs are verified against remote JWKS with issuer, audience and required claim checks.

Local development may set `GITGAT_DEV_AUTH_SUBJECT`. Development auth is rejected if the API binds to a non-loopback address, so it cannot be used as a public deployment shortcut.

Organization data is scoped through `organization_memberships`. The current role model is:
- owner
- admin
- repository_admin
- team_lead
- developer
- auditor

The dashboard is organization-scoped. There is intentionally no global unauthenticated governance dashboard.
