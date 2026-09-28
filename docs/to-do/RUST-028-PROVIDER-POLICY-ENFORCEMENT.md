# RUST-028 - Provider Branch Protection / Status Enforcement

Status: TODO

## Goal
Operationalize RUST-018 across enrolled repositories.

## GitHub
- install/configure GitGat check;
- publish commit-bound result;
- document required ruleset/status setup;
- verify head SHA before decision;
- support re-run.

## Azure DevOps
- publish PR status;
- configure required branch policy;
- verify source commit;
- surface provider policy configuration drift.

## Admin UI
- enforcement status by repository;
- missing-required-check warning;
- fix/setup instructions;
- observed vs desired state.

## Acceptance
Control Plane detects if a repository is not actually enforcing required GitGat policy and marks it unhealthy.
