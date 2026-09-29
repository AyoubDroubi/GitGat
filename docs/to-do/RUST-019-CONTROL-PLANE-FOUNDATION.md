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

Foundation commit: `2e27413f868d1ebce16c4ea3d67eea00266e6340`.

Implemented:
- independent Rust Control Plane service under `control-plane/`;
- PostgreSQL connection and embedded migrations;
- initial governance schema for organizations, repositories, lock policies and audit events;
- liveness/readiness endpoints;
- versioned `/api/v1` governance seed;
- structured API errors, request IDs and HTTP tracing;
- dedicated Control Plane quality workflow.

Remaining before DONE:
- authenticated administrator identity (RUST-020 dependency);
- repository enrollment mutation/API (RUST-021 dependency);
- admin web UI shell;
- PostgreSQL integration test proving migrations and readiness;
- green Control Plane quality evidence.
