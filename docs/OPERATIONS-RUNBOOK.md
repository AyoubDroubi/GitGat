# GitGat Operations Runbook

## Provider outage

1. Mark affected provider health degraded.
2. Stop claiming stale lock observations are authoritative.
3. Protected operations that need lock verification fail closed.
4. Keep standalone/non-protected local Git usable.
5. Reconcile all enrolled repositories after recovery.

## Webhook backlog

1. Inspect failed/dead-letter background jobs.
2. Confirm provider health and credentials.
3. Replay by provider delivery ID; idempotency prevents duplicate processing.
4. Reconcile repository state after replay.

## LFS lock API outage

Do not infer ownership from cache. Show last verification time, mark it stale, and block protected mutations requiring authority.

## Database incident

Follow `DATA-LIFECYCLE.md`. After restore, provider lock reconciliation is mandatory before enforcement is re-enabled.

## Failed deployment

Roll back the Control Plane image independently from desktop clients. If a migration is forward-only, deploy the previous compatible application image; restore DB only when the migration itself damaged data.

## Credential compromise

1. Revoke/rotate provider app credential.
2. Rotate SSO gateway signing secret.
3. Invalidate provider sessions.
4. Audit provider connection and privileged actions.
5. Reconcile repository health before enforcement resumes.

## Emergency enforcement downgrade

An organization owner may move a repository from `enforce` to `warn` only through an audited privileged action. This never fabricates lock success and does not change provider lock state.
