# GitGat End-to-End Delivery Roadmap

This roadmap expands GitGat from the current Rust desktop client into a production-ready, multi-provider Git + Git LFS locking platform with an optional organization Control Plane.

## Product end state

GitGat should provide:

- native Git desktop workflows;
- GitHub and Azure DevOps forge integrations;
- strict Git LFS file locking across teams;
- provider-side merge enforcement so users cannot bypass locking through another Git client;
- organization-wide governance through the GitGat Control Plane;
- repository enrollment, policy management, RBAC, audit, exceptions and force-unlock approval;
- active-lock and stale-lock monitoring across repositories;
- secure provider connections without storing end-user GitHub/Azure PATs in the desktop client;
- cross-platform desktop validation and production deployment evidence;
- a documented recovery and incident process.

## Architectural rule

The remote Git LFS lock service remains the authoritative source for active locks.

The GitGat Control Plane stores governance, policy, audit and derived lock observations. It must never become an independent competing lock authority.

## Phases

### Phase A - Locking Core
- RUST-014 Strict Git LFS desktop enforcement
- RUST-015 Cross-platform multi-identity locking E2E

### Phase B - Multi-provider Forge
- RUST-016 Forge provider abstraction
- RUST-017 Azure DevOps Repos / PRs / Pipelines live integration

### Phase C - Server-side Enforcement
- RUST-018 Required lock-policy checks before merge

### Phase D - Control Plane Foundation
- RUST-019 Control Plane service and admin portal foundation
- RUST-020 Identity, organizations, teams and RBAC
- RUST-021 Repository enrollment, provider connections and webhooks

### Phase E - Governance
- RUST-022 Lock policy and protected-pattern management
- RUST-023 Organization-wide active lock dashboard and synchronization
- RUST-024 Force-unlock approval and policy exception workflows
- RUST-025 Durable audit ledger and compliance history
- RUST-026 Stale-lock detection, reminders and escalation

### Phase F - Desktop / Control Plane Integration
- RUST-027 Desktop enrollment, policy sync and offline behavior
- RUST-028 Provider status checks, branch protection and anti-bypass enforcement

### Phase G - Operations and Security
- RUST-029 Admin operations, observability and support tooling
- RUST-030 Security hardening and threat-model closure
- RUST-031 Database lifecycle, backup, migrations and retention
- RUST-032 Control Plane deployment and production infrastructure

### Phase H - Release Certification
- RUST-033 Full-system E2E and chaos/failure validation
- RUST-034 Documentation, onboarding and administrator runbooks
- RUST-035 Production release certification and rollout

## Global completion rules

A TODO is DONE only if:
1. implementation exists;
2. automated tests cover the required behavior;
3. live-provider behavior is validated where applicable;
4. security/safety invariants are preserved;
5. user-visible workflow is usable without hidden CLI knowledge;
6. evidence is linked in the TODO and INDEX;
7. no known P0/P1 defect remains in the scope.

## Branch rule

Development remains against main. production remains reserved and must not be changed without explicit authorization.
