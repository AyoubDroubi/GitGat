# RUST-015 - Cross-platform Multi-Identity Locking E2E

Status: IN PROGRESS

## Goal
Prove the locking experience works between two real identities, not merely two clones sharing the same credentials.

## Required matrix
- Windows
- macOS
- Linux
- GitHub
- Azure DevOps

## Canonical flow
User A clones -> locks protected asset -> file becomes writable for A.
User B clones/refreshes -> sees lock owner A -> file remains read-only -> Stage/Commit/Push are denied.
User A pushes/merges/unlocks.
User B refreshes/pulls -> acquires lock -> file becomes writable -> can Stage/Commit/Push.

## Scenarios
- fresh clone;
- existing clone;
- stale cached locks;
- expired auth;
- network unavailable;
- lock server unavailable;
- lock owner changed;
- file renamed while locked;
- branch switch while lock exists;
- unlock before merge;
- unlock after merge;
- interrupted push;
- teammate attempts manual chmod/read-only bypass;
- teammate manually stages outside GitGat.

## Acceptance
The second identity cannot accidentally complete a protected-file workflow while another identity owns the lock.

## Evidence
Store workflow run IDs, fixture repositories and screenshots/log excerpts in this TODO.
