# RUST-035 - Production Release Certification

Status: TODO

## Goal
Create one final release gate proving GitGat is ready for organization use.

## Required green evidence
- all mandatory TODOs DONE;
- Windows quality green;
- macOS quality green;
- Linux quality green;
- GitHub live E2E green;
- Azure DevOps live E2E green;
- multi-identity LFS locking green;
- required provider lock-policy enforcement green;
- Control Plane staging smoke/E2E green;
- security review green;
- dependency audit green;
- backup restore drill green;
- migrations validated;
- installer/package artifacts verified;
- zero open P0/P1 issues.

## Rollout
1. internal dogfood;
2. controlled pilot team;
3. monitored organization rollout;
4. general availability.

## Rollback
Maintain independent rollback paths for:
- desktop application;
- Control Plane service;
- database migration;
- provider policy enforcement.

## Final release record
Record:
- release version/tag;
- commit SHA;
- workflow run IDs;
- artifact checksums;
- database migration version;
- deployed environment versions;
- known limitations;
- approved deferrals;
- rollback instructions.

## Acceptance
GitGat can be released with evidence that desktop Git, locking, GitHub, Azure DevOps, server enforcement, governance, auditing and recovery work end-to-end.
