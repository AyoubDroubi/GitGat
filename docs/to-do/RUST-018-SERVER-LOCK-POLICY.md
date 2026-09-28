# RUST-018 - Server-side Required Lock Policy

Status: IN PROGRESS

## Goal
Prevent protected-file lock policy from being bypassed by users who use another Git client or raw CLI.

## Why
Desktop guards alone are not sufficient because Git is distributed and local protections can be bypassed.

## GitHub
Build a required GitGat status/check that:
- examines PR changed files;
- determines which paths are lock-required;
- resolves relevant lock evidence/policy;
- validates no lock-policy violation exists;
- publishes PASS/FAIL with actionable details;
- can be made required by branch protection/rulesets.

## Azure DevOps
Build equivalent PR status/policy integration:
- evaluate changed paths;
- post GitGat policy status;
- configure required status on protected target branches;
- expose remediation.

## Policy result model
- PASS
- FAIL_LOCK_MISSING
- FAIL_LOCK_OWNED_BY_OTHER
- FAIL_POLICY_UNAVAILABLE
- EXEMPTED_WITH_APPROVAL
- NOT_APPLICABLE

## Anti-bypass
- fail closed for protected patterns;
- signed/traceable policy result;
- result bound to provider repository + PR + commit SHA;
- reevaluate when head SHA changes;
- invalidate stale approvals.

## Acceptance
A protected branch cannot merge a PR that violates GitGat lock policy, even if the author never used GitGat desktop.


## Implementation started

Added provider-neutral policy engine:
- `src/policy.rs`
- CLI gate: `gitgat --check-lock-policy --base <ref> --actor <identity>`

Current policy results:
- `PASS`
- `NOT_APPLICABLE`
- `FAIL_LOCK_MISSING`
- `FAIL_LOCK_OWNED_BY_OTHER`
- `FAIL_POLICY_UNAVAILABLE`

Added enforcement entry points:
- GitHub Actions: `.github/workflows/lock-policy.yml`
- Azure Pipelines template: `ci/azure/gitgat-lock-policy.yml`

The evaluator:
1. diffs the PR head against the supplied base ref;
2. identifies paths marked `lockable`;
3. queries the authoritative Git LFS lock service;
4. requires each protected changed file to have an active lock;
5. requires the lock owner to match the PR actor;
6. fails closed if the lock service cannot be queried.

## Remaining before DONE
- current branch quality + lock-policy workflow must be green;
- GitHub ruleset/branch protection must require the GitGat policy check;
- Azure DevOps required PR status/branch policy must be live-tested;
- bind final provider status to repository + PR + exact head SHA;
- add approved exception state after Control Plane audit/RBAC exist;
- record live anti-bypass evidence on both providers.
