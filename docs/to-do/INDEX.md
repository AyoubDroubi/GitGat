# GitGat Rust TODO Index

Baseline: rust-v1
Track: MAJOR architecture migration
Start authorization: user explicitly authorized full Rust migration and implementation on 2026-09-28.
Development branch: main
Legacy snapshot: dotnet
Production: reserved and untouched

| ID | Scope | Status | Evidence target |
|---|---|---|---|
| RUST-001 | Rust-native foundation, module boundaries, process safety | DONE | build + tests |
| RUST-002 | Repository open/clone, recent catalog, favorites, relocation-safe state | DONE | integration tests + UI |
| RUST-003 | Changes, diff, stage, unstage, tracked discard, commit | DONE | disposable repo tests + UI |
| RUST-004 | Fetch, FF-only pull, push, branches and history | DONE | disposable repo tests + UI |
| RUST-005 | GitHub auth/identity and forge adapter | DONE | parser tests + manual gh flow |
| RUST-006 | Pull requests, edit/review/comment/merge | DONE | adapter + UI |
| RUST-007 | Git LFS lock/unlock/list | DONE | adapter + UI |
| RUST-008 | Workspaces, recent repos and My Work | DONE | SQLite tests + UI |
| RUST-009 | Actions, notifications and team activity | DONE | adapter + UI |
| RUST-010 | Native navigation, shortcuts, drag/drop, dark UI and safety UX | DONE | native compile on 3 OSes |
| RUST-011 | Conflicts, stash, worktrees, reflog, recovery, reset/rebase/cherry-pick | DONE | safety tests + UI |
| RUST-012 | Quality, security and cross-platform hardening | DONE | fmt/clippy/test/audit matrix |
| RUST-013 | Windows portable EXE and Inno Setup installer | DONE | package workflow artifact |
| RUST-014 | Strict Git LFS lockable workflow and desktop enforcement | IN PROGRESS | quality + main live LFS gate pending |
| RUST-015 | Cross-platform read-only and lock ownership E2E | IN PROGRESS | live lockable/read-only flow added; separate-identity fixture still pending |
| RUST-016 | Forge provider abstraction (GitHub/Azure DevOps) | IN PROGRESS | implementation present; quality/regression gate pending |
| RUST-017 | Azure DevOps Repos/PRs/Pipelines integration | IN PROGRESS | implementation present; authenticated Azure live E2E pending |
| RUST-018 | Provider-side required lock-policy check before merge | TODO | GitHub required check + Azure branch policy |
| RUST-019 | GitGat Control Plane / admin portal for governance and audit | TODO | policy/audit service + admin UI |

## Current locking expansion

The locking expansion follows `docs/LOCKING-ARCHITECTURE.md`. The provider Git LFS server remains the authoritative active-lock source. The future Control Plane is governance/audit only and must not become a second independent lock registry.

## Completion gate

A Rust TODO moves to DONE only when the relevant implementation is present and the final quality/package workflows provide passing evidence. No TODO is marked DONE merely because code was written.


## Final validation evidence

Validated Rust source baseline: e0388a6754c366586377354d4bee0339c735341a.

- Quality run 36354944202: SUCCESS.
  - Windows: SUCCESS.
  - Ubuntu: SUCCESS.
  - macOS: SUCCESS.
  - cargo fmt: SUCCESS.
  - cargo clippy with warnings denied: SUCCESS.
  - cargo test: SUCCESS.
  - release build and self-check: SUCCESS.
  - cargo audit: SUCCESS.
- Windows packaging run 36354910656: SUCCESS.
  - Portable GitGat.exe self-check: SUCCESS.
  - Inno Setup compilation: SUCCESS.
  - Silent installer lifecycle and installed executable self-check: SUCCESS.
  - Silent uninstall verification: SUCCESS.
  - SHA256SUMS.txt produced.
  - Artifact: GitGat-0.1.0-win-x64, artifact ID 10943536556.
- The current Rust source files, Cargo.toml and rust-toolchain.toml have identical blob SHAs to the validated quality baseline. Later commits only changed CI/docs housekeeping.
- Legacy .NET implementation remains preserved on branch dotnet at a008fccda733f15c0fa1b39b3db56f1a8bea2091.
- production remains untouched.
