# RUST-019 - GitGat Control Plane / Admin Portal Foundation

Status: TODO

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
