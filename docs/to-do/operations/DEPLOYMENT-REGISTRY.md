# Deployment Registry

Every deployment attempt is recorded, successful or not.

| Deployment ID | Timestamp | Result | Environment | Service/App | TODO | Baseline | Branch | Commit | Source/Mechanism | Artifact | Verification | Rollback | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |

## Rules

- Record failed deployments too.
- Retry = new Deployment ID.
- Record the actual deployment mechanism; do not assume GitHub, Railway, CI/CD, or another provider.
- Production deployment must identify the exact commit/version when evidence is available.
- Record post-deploy verification separately or in the Verification field.
