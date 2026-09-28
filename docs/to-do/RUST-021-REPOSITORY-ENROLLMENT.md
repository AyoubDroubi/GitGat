# RUST-021 - Repository Enrollment, Provider Connections and Webhooks

Status: TODO

## Goal
Connect GitHub/Azure repositories to the Control Plane safely.

## Enrollment
- select provider;
- resolve organization/project/repository;
- verify admin permission;
- verify Git LFS capability;
- inspect .gitattributes;
- inspect protected target branches;
- choose policy mode: observe / warn / enforce;
- establish webhook/status integration;
- health check.

## Provider connections
- GitHub App / provider-native installation preferred for server access;
- Azure service connection / app registration appropriate for server-side operations;
- secrets stored only in secure server secret storage;
- rotation and revoke flows;
- connection health status.

## Webhooks/events
Handle:
- push;
- pull request created/updated/merged/closed;
- branch changes;
- relevant repository changes;
- policy configuration changes where provider exposes events.

## Reliability
- signature validation;
- delivery idempotency;
- replay protection;
- retry queue;
- dead-letter visibility;
- audit raw metadata without leaking secrets.

## Acceptance
An admin can enroll a repository and the Control Plane receives and reconciles provider events reliably.
