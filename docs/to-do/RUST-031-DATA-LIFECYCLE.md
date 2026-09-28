# RUST-031 - Database Lifecycle, Backups, Migrations and Retention

Status: TODO

## Database
PostgreSQL for Control Plane durable state.

## Requirements
- versioned migrations;
- forward deployment strategy;
- rollback/restore plan;
- transactional writes;
- optimistic concurrency where relevant;
- organization isolation constraints;
- indexes for lock/audit queries.

## Backup
- automated backups;
- restore drill;
- documented RPO/RTO;
- encrypted storage;
- access controls.

## Retention
Define separately for:
- audit events;
- webhook deliveries;
- lock observations;
- operational logs;
- expired exceptions.

## Acceptance
A tested restore procedure can rebuild the Control Plane without inventing active lock truth; active locks are re-reconciled from providers after restore.
