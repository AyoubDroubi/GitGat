# RUST-018 - Server-side Required Lock Policy

Status: TODO

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
