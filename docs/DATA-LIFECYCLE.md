# Control Plane Data Lifecycle

## Backup and restore

Use PostgreSQL managed backups or encrypted `pg_dump` snapshots. Target initial objectives:

- RPO: 15 minutes for governance/audit data.
- RTO: 60 minutes for Control Plane service recovery.
- Active lock truth is **not** restored from backup as authoritative state; after restore every enrolled repository must reconcile locks from its provider.

Restore drill:

1. Provision an empty PostgreSQL instance.
2. Restore the latest backup.
3. Run all forward migrations.
4. Start Control Plane in enforcement-disabled staging mode.
5. Verify organization/repository/audit counts.
6. Reconcile GitHub/Azure provider health.
7. Re-fetch all active Git LFS locks.
8. Only then permit enforcement.

## Retention defaults

- audit events: 7 years unless organization policy overrides;
- webhook deliveries: 30 days metadata, redact secret-bearing payload fields;
- lock observations: 90 days;
- background job diagnostics: 30 days;
- expired policy exceptions: 1 year;
- application logs: 30 days in primary logging storage.

## Migration rules

- migrations are forward-only in production;
- schema changes must remain compatible during rolling deployment;
- destructive cleanup happens only in a later migration after old application versions are retired;
- every production migration is backed by a pre-deploy backup/restore point.

## Deletion

Organization deletion is an explicit privileged workflow with export/retention review. Audit retention obligations may require tombstoning rather than immediate physical deletion.
