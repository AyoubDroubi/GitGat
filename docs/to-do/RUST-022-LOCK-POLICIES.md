# RUST-022 - Lock Policy and Protected Pattern Management

Status: TODO

## Goal
Manage organization/repository locking rules centrally while keeping .gitattributes compatible.

## Policy capabilities
- include patterns;
- exclude patterns;
- required-lock flag;
- repository defaults;
- organization templates;
- branch scope;
- path scope;
- maximum lock age warning;
- force-unlock approval requirements;
- exception rules.

## Sync model
Policy changes must produce an explicit desired-vs-observed state:
- desired policy in Control Plane;
- observed .gitattributes/provider state;
- drift status;
- optional generated PR to reconcile repository policy.

Never silently rewrite protected branches from the portal.

## UI
- policy editor;
- pattern tester;
- impact preview;
- drift preview;
- generated .gitattributes diff;
- apply via PR;
- audit history.

## Acceptance
Admins can define and safely roll out lock policy with preview, review and auditable repository reconciliation.
