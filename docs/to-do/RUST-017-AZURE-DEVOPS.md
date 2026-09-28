# RUST-017 - Azure DevOps Repos / PRs / Pipelines

Status: IN PROGRESS

## Goal
Provide first-class Azure DevOps workflow parity for the GitGat product scope.

## Authentication
- Microsoft Entra through Azure CLI initially;
- no PAT persistence in GitGat desktop;
- explicit account/expiry status;
- reconnect flow;
- later native OAuth can replace CLI without changing domain contract.

## Azure Repos
- detect dev.azure.com;
- detect legacy visualstudio.com;
- detect SSH remote and warn for LFS locking transport limitations;
- parse organization/project/repository;
- list PRs;
- create PR;
- edit PR;
- comments;
- approve/vote;
- complete PR;
- delete source branch where requested;
- surface branch-policy failures.

## Azure Pipelines
- list runs;
- display status/result;
- rerun selected pipeline;
- fetch useful logs;
- link to provider page;
- handle missing permissions.

## Live validation
Use an authenticated Azure DevOps test project and disposable branches. Never use production branch for E2E.

## Acceptance
A user can perform the normal GitGat PR + CI flow against Azure DevOps without leaving GitGat except provider login.
