# RUST-033 - Full-system E2E and Failure Validation

Status: TODO

## Goal
Validate the entire GitGat platform as one system across desktop, providers and Control Plane.

## Golden journey
Enroll repository -> apply policy -> developer A locks -> developer B is blocked -> A edits/stages/commits/pushes -> PR opens -> required GitGat policy passes -> review/merge -> A unlocks -> B refreshes/pulls/locks -> B edits successfully.

Run the same golden journey for:
- GitHub
- Azure DevOps
- Windows desktop
- macOS desktop
- Linux desktop

## Failure scenarios
- provider API unavailable;
- Control Plane unavailable;
- database unavailable;
- stale webhook;
- duplicate webhook;
- auth expiry;
- provider permission revoked;
- lock server unavailable;
- lock verification returns inconsistent data;
- policy drift;
- PR head changes during evaluation;
- force unlock races with owner unlock;
- stale cached lock;
- interrupted push;
- deployment rollback.

## Evidence
- automated run IDs;
- live-provider fixture repository IDs;
- pass/fail matrix;
- retained diagnostics for failed scenarios;
- known limitations.

## Acceptance
All P0 journeys and defined failure modes have repeatable passing evidence or an explicitly accepted limitation.
