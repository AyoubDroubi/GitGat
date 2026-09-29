# RUST-020 - Identity, Organizations, Teams and RBAC

Status: IN PROGRESS

## Roles
- Organization Owner
- Organization Admin
- Repository Admin
- Team Lead
- Developer
- Auditor / Read-only

## Permissions
Separate permissions for:
- view repositories;
- manage enrollment;
- manage protected patterns;
- view active locks;
- request force unlock;
- approve force unlock;
- manage exceptions;
- view audit;
- manage provider connections;
- manage members/roles.

## Security rules
- least privilege;
- deny by default;
- organization isolation;
- repository-scoped permissions;
- approval action cannot be performed by the requester when separation-of-duty is enabled;
- every role/permission mutation audited.

## Authentication
Support organization login appropriate for the deployment model, with provider SSO/OIDC favored over local passwords.

## Acceptance
Every Control Plane API mutation is authorized by explicit permissions and tenant isolation tests.


## Implementation started

Foundation commit: `6529edb89aa0f5a9e49fa208473fd992bc3637b3`.

Implemented:
- users, organization memberships, teams and team memberships schema;
- repository-scoped role assignments;
- explicit role/permission domain with deny-by-absence semantics;
- least-privilege tests for Developer and Auditor;
- separation-of-duty helper and self-approval test;
- organization-scoped relational boundaries in PostgreSQL.

Remaining before DONE:
- OIDC/SSO authentication and trusted subject extraction;
- request authorization middleware;
- tenant isolation tests against live PostgreSQL;
- audited membership/role mutation APIs;
- repository permission resolution combining organization and repository scope.
