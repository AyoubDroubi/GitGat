# GitGat Security Threat Model

## Trust boundaries

1. Desktop GitGat process and local Git repository.
2. GitHub/Azure DevOps provider APIs and Git LFS lock service.
3. GitGat Control Plane API/admin portal.
4. Trusted OIDC/SSO gateway in front of protected Control Plane APIs.
5. PostgreSQL and external secret storage.

The provider Git LFS server remains authoritative for active locks. Control Plane observations never create an independent lock truth.

## Primary threats and controls

| Threat | Control |
|---|---|
| Stolen desktop session | short-lived provider sessions; revoke support; protected mutations require live lock verification |
| Stolen Control Plane session | SSO gateway; signed identity headers; 5-minute replay window; RBAC; tenant isolation |
| Forged SSO headers | HMAC signature verified by Control Plane; fail closed when secret/signature missing |
| Compromised webhook | HMAC signature verification primitive; delivery idempotency; replay-safe storage |
| Cross-tenant access | organization-scoped queries; repository-to-organization resolution; integration tests |
| Policy bypass | exact commit-bound provider status; protected .gitattributes checks; fail-closed lock verification |
| Force-unlock abuse | explicit request; reason; separation of duty; live lock identity revalidation; audit |
| Audit tampering | append-oriented events plus chained event hashes; restricted mutation access |
| Secret leakage | provider credentials represented by secret references only; no provider PAT stored in Control Plane tables |
| Malicious paths | repository-relative pattern normalization; reject absolute/parent traversal patterns |
| Command injection | desktop process execution remains shell-free with explicit arguments |
| Replay | timestamped signed auth identity; webhook delivery IDs unique per connection |
| Database compromise | least-privilege DB role, encrypted transport/storage, backup access controls |

## Security invariants

- No automatic force unlock by default.
- No silent rewriting of protected branches.
- No provider PAT synchronization through Control Plane.
- No active-lock decision from an expired observation.
- Missing provider/lock verification fails closed for protected mutations.
- Production secrets are never committed to the repository.

## Remaining live evidence

Production certification still requires provider credential rotation tests, live webhook signature verification for each provider, independent security review, and dependency-audit evidence.
