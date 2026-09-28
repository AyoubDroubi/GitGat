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

## Implementation started

Added:
- `.github/workflows/e2e-locking-two-identities.yml`
- `tests/live_lfs_identity.rs`

The live gate requires two separate GitHub identities through:
- `GITGAT_E2E_USER_A_TOKEN`
- `GITGAT_E2E_USER_B_TOKEN`

The harness explicitly tests:
- teammate lock appears as `Theirs`;
- protected file is read-only for the teammate;
- manual permission bypass does not let GitGat Stage succeed;
- raw `git add` does not let GitGat Commit succeed;
- raw Stage + raw Commit still hit strict Push verification;
- after owner unlocks, the second identity can acquire the lock and complete the normal GitGat flow.

## Remaining before DONE
- configure the two live GitHub identities and record a passing run;
- extend the live identity matrix to Azure DevOps;
- add Windows/macOS live read-only evidence where provider credentials allow;
- execute the defined auth/network/stale-lock failure scenarios.

## Evidence
Store workflow run IDs, fixture repositories and diagnostic excerpts in this TODO.
