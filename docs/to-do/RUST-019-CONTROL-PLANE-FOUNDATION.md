# RUST-019 - GitGat Control Plane / Admin Portal Foundation

Status: IN PROGRESS

## Goal
Create the organization governance layer without becoming a second lock authority.

## Recommended architecture
- API/service: Rust service;
- database: PostgreSQL;
- admin web UI: dedicated web client;
- provider integration workers/webhooks;
- background jobs for stale lock reconciliation;
- deployment separate from desktop releases.

The exact web framework can be finalized before implementation, but API/domain boundaries must remain framework-neutral.

## Initial modules
- Organizations
- Teams
- Users
- Repositories
- Provider Connections
- Active Locks
- Lock Policies
- Exceptions
- Force Unlock Requests
- Audit Log
- Provider Health
- Settings

## Core entities
- Organization
- Membership
- Team
- TeamMembership
- RepositoryRegistration
- ProviderConnection
- LockPolicy
- LockObservation
- PolicyException
- ForceUnlockRequest
- Approval
- AuditEvent
- WebhookDelivery
- ProviderHealthSnapshot

## Non-goals
- storing a competing authoritative lock state;
- storing desktop-user GitHub/Azure PATs;
- automatically force-unlocking without policy.

## API foundation
- versioned API;
- pagination/filtering;
- idempotency for mutations;
- optimistic concurrency;
- structured errors;
- audit context;
- health/readiness endpoints.

## Acceptance
The portal can authenticate an admin, enroll a repository and show governance state without changing provider lock truth.


## Implementation started

Current branch: `feature/control-plane-foundation`

Implemented foundation:
- Rust Axum API service;
- PostgreSQL 17 + SQLx migrations;
- liveness and database-backed readiness endpoints;
- service metadata endpoint declaring `provider_git_lfs` as active lock authority;
- database-backed dashboard summary endpoint;
- foundation schema for organizations, users, memberships, providers, repositories, policies, lock observations, exceptions, force-unlock requests/approvals, audit, webhooks and provider health;
- local Docker PostgreSQL development environment;
- React 19.3 + Vite 8.3 admin web foundation;
- dashboard shell connected to API summary/health/meta endpoints;
- dedicated CI gate for Rust quality, PostgreSQL migration/API smoke, and web build.

Intentionally not enabled yet:
- authentication;
- RBAC mutations;
- provider webhook processing;
- force-unlock execution;
- policy mutation endpoints.

Those remain gated by RUST-020/RUST-021/RUST-025.
