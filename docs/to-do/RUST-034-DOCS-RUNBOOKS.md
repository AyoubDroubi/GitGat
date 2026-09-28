# RUST-034 - Documentation, Onboarding and Runbooks

Status: TODO

## Developer documentation
- architecture;
- provider abstraction;
- Git/LFS locking lifecycle;
- Control Plane domain model;
- local setup;
- tests;
- migrations;
- adding another provider;
- release process.

## Desktop user documentation
- open/clone repository;
- provider login;
- protecting file patterns;
- lock/edit/stage/commit/push;
- PR workflow;
- unlock;
- stale lock handling;
- offline behavior;
- conflict/recovery;
- error messages and remediation.

## Admin documentation
- Control Plane setup;
- organization/team setup;
- provider connection;
- repository enrollment;
- policy configuration;
- branch/status enforcement;
- RBAC;
- force unlock approval;
- exceptions;
- audit search/export;
- stale-lock escalation.

## Operations runbooks
- GitHub outage;
- Azure DevOps outage;
- LFS lock API outage;
- webhook backlog;
- stuck policy evaluation;
- provider credential compromise;
- failed deployment;
- database restore;
- migration failure;
- emergency disable/enforcement downgrade procedure.

## Acceptance
A new developer, end user, repository admin and platform operator can complete their primary workflow from documentation alone.
