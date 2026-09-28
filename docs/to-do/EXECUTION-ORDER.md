# GitGat Execution Order and Dependency Gates

This file defines the required implementation order. TODO numbers express scope; these gates define when work may safely progress.

## Dependency chain

```text
RUST-014 Strict LFS
   |
   v
RUST-015 Multi-identity E2E
   |
   +----------------------+
   |                      |
   v                      v
RUST-016 Provider      RUST-018 Server
Abstraction            Lock Policy design
   |
   v
RUST-017 Azure DevOps live integration
   |
   +-----------> RUST-018 Server Lock Policy implementation
                        |
                        v
                 RUST-019 Control Plane Foundation
                        |
              +---------+---------+
              |                   |
              v                   v
        RUST-020 RBAC       RUST-021 Enrollment
              |                   |
              +---------+---------+
                        |
                        v
                 RUST-022 Policies
                        |
              +---------+---------+
              |                   |
              v                   v
        RUST-023 Locks      RUST-025 Audit
              |                   |
              +---------+---------+
                        |
                        v
                 RUST-024 Force Unlock
                        |
                        v
                 RUST-026 Stale Locks
                        |
                        v
                 RUST-027 Desktop Sync
                        |
                        v
                 RUST-028 Provider Enforcement
                        |
              +---------+---------+---------+
              |                   |         |
              v                   v         v
        RUST-029 Ops       RUST-030 Sec  RUST-031 Data
                                 |         /
               +------------------+--------+
                                  |
                                  v
                         RUST-032 Deployment
                                  |
                                  v
                         RUST-033 Full E2E
                                  |
                                  v
                         RUST-034 Docs/Runbooks
                                  |
                                  v
                         RUST-035 Certification
```

## Gate A - Desktop locking trustworthy

Required before moving to organization governance:
- RUST-014 complete;
- RUST-015 complete for GitHub;
- no known lock ownership ambiguity;
- protected Stage/Commit/Push fail closed.

## Gate B - Multi-provider contract stable

Required before the Control Plane depends on provider behavior:
- RUST-016 complete;
- RUST-017 Azure live PR/Pipeline flow proven;
- provider capability matrix documented;
- authentication failures tested.

## Gate C - Anti-bypass enforcement proven

Required before calling the product enterprise-ready:
- RUST-018 policy evaluator works;
- GitHub required status/check proven;
- Azure DevOps required PR status/policy proven;
- PR head SHA changes invalidate/re-evaluate results.

## Gate D - Control Plane governance safe

Before force-unlock:
- RUST-019 foundation;
- RUST-020 RBAC;
- RUST-021 enrollment/provider connection;
- RUST-022 policy model;
- RUST-025 audit ledger.

RUST-024 must not ship before this gate.

## Gate E - Managed desktop safe

Before desktop relies on Control Plane policy:
- organization/repository identity stable;
- policy API versioned;
- drift semantics defined;
- offline fail-closed behavior tested;
- server enforcement remains authoritative if desktop is bypassed.

## Gate F - Production infrastructure safe

Before staging/production:
- security threat model reviewed;
- database migrations/restore tested;
- secrets externalized;
- observability operational;
- rollback path documented.

## Gate G - Release

RUST-035 requires:
- RUST-014 through RUST-034 complete or formally deferred;
- no P0/P1 defect;
- live GitHub and Azure evidence;
- three-OS desktop evidence;
- Control Plane staging evidence;
- security and restore evidence.

## Parallelism allowed

Safe parallel work:
- RUST-016 and early RUST-018 design;
- RUST-020 and RUST-021 after RUST-019 foundation;
- RUST-023 and RUST-025 after repository/policy identities stabilize;
- RUST-029, RUST-030 and RUST-031 after core Control Plane APIs stabilize.

Unsafe parallel work:
- Force unlock before RBAC/audit;
- production deployment before migrations/restore;
- stale-lock automation before active-lock reconciliation;
- desktop managed-mode enforcement before server-side policy enforcement is defined;
- production rollout before full-system E2E.
