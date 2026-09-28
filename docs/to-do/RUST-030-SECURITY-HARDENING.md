# RUST-030 - Security Hardening and Threat Model

Status: TODO

## Threat model
Cover:
- stolen desktop session;
- stolen Control Plane session;
- compromised provider webhook;
- leaked provider app credential;
- cross-tenant access;
- malicious repository metadata/path;
- command injection;
- policy bypass;
- forged status check;
- replayed webhook;
- audit tampering;
- force-unlock abuse.

## Controls
- shell-free process execution remains invariant;
- strict argument handling;
- input/path validation;
- CSRF/XSS protections for portal;
- secure cookies/session lifecycle;
- least-privilege provider app scopes;
- encrypted secrets at rest;
- rotation;
- webhook signature validation;
- rate limiting;
- permission tests;
- dependency auditing;
- security headers;
- backup access controls.

## Acceptance
Threat model has no unresolved critical/high finding and automated security checks are in CI.
