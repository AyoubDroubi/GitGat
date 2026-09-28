# RUST-020 - Identity, Organizations, Teams and RBAC

Status: TODO

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
