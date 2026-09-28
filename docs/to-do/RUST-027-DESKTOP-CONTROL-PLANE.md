# RUST-027 - Desktop Enrollment, Policy Sync and Offline Behavior

Status: TODO

## Goal
Connect GitGat desktop to organization governance without breaking standalone Git usage.

## Modes
- Standalone repository
- Organization-managed repository

## Managed repository behavior
- detect enrollment;
- show organization/policy status;
- fetch policy;
- compare local .gitattributes against desired state;
- show drift;
- surface exceptions;
- link force-unlock request;
- show provider/Control Plane health.

## Offline
- local Git remains usable for non-protected files;
- cached policy clearly marked stale;
- protected mutations requiring authoritative verification fail closed;
- no fake success;
- reconcile when back online.

## Security
- use short-lived authenticated session;
- no provider PAT synchronization through Control Plane;
- device/session revoke support.

## Acceptance
A developer understands whether a repo is managed, which rules apply and why an operation is blocked.
