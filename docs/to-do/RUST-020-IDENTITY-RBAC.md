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

Current branch: `feature/control-plane-rbac`

Implemented:
- remote JWKS JWT validation for OIDC-compatible providers;
- issuer + audience + required claim validation;
- OIDC decoder key refresh lifecycle;
- loopback-only development identity mode;
- startup refusal if development auth is used on a non-loopback bind;
- startup refusal for incomplete OIDC configuration;
- authenticated `/api/v1/me`;
- organization membership listing;
- organization access endpoint;
- explicit role/permission model;
- organization-scoped dashboard authorization;
- tenant-filtered dashboard SQL;
- admin web organization switcher.

Force-unlock execution remains disabled until the audit/approval gate is complete.
