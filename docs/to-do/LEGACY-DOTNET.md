# Legacy .NET TODO disposition

Architecture decision: the .NET/Avalonia implementation was retired in favor of Rust.

Snapshot branch: dotnet
Snapshot commit: a008fccda733f15c0fa1b39b3db56f1a8bea2091
Production branch: untouched

All legacy implementation TODOs are canceled as implementation tracks because their technology baseline is no longer active. Their functional requirements were carried into the Rust roadmap.

| Legacy item | Status | Replacement |
|---|---|---|
| TODO-001 Desktop foundation & re-architecture | CANCELED | RUST-001 |
| TODO-002 Repository onboarding, clone & local catalog | CANCELED | RUST-002 |
| TODO-003 Files, changes, staging & commit workflow | CANCELED | RUST-003 |
| TODO-004 Fetch/pull/push, branches & history | CANCELED | RUST-004 |
| TODO-005 GitHub connection, identity & forge abstraction | CANCELED | RUST-005 |
| TODO-006 Pull requests, reviews, comments & merge | CANCELED | RUST-006 |
| TODO-007 Git LFS file locking | CANCELED | RUST-007 |
| TODO-008 Multi-repository workspaces & My Work | CANCELED | RUST-008 |
| TODO-009 GitHub Actions, notifications & team activity | CANCELED | RUST-009 |
| TODO-010 Design system, UX polish & accessibility | CANCELED | RUST-010 |
| TODO-011 Recovery, conflicts & Advanced Git | CANCELED | RUST-011 |
| TODO-012 Quality, performance, security & cross-platform hardening | CANCELED | RUST-012 |
| TODO-013 Windows executable & installer artifact | CANCELED | RUST-013 |

Cancellation means the .NET implementation track was superseded; it does not mean the product requirement was abandoned.
