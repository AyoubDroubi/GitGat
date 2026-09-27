# GitGat Rust TODO Index

Baseline: rust-v1
Track: MAJOR architecture migration
Start authorization: user explicitly authorized full Rust migration and implementation on 2026-09-28.
Development branch: main
Legacy snapshot: dotnet
Production: reserved and untouched

| ID | Scope | Status | Evidence target |
|---|---|---|---|
| RUST-001 | Rust-native foundation, module boundaries, process safety | IN_REVIEW | build + tests |
| RUST-002 | Repository open/clone, recent catalog, favorites, relocation-safe state | IN_REVIEW | integration tests + UI |
| RUST-003 | Changes, diff, stage, unstage, tracked discard, commit | IN_REVIEW | disposable repo tests + UI |
| RUST-004 | Fetch, FF-only pull, push, branches and history | IN_REVIEW | disposable repo tests + UI |
| RUST-005 | GitHub auth/identity and forge adapter | IN_REVIEW | parser tests + manual gh flow |
| RUST-006 | Pull requests, edit/review/comment/merge | IN_REVIEW | adapter + UI |
| RUST-007 | Git LFS lock/unlock/list | IN_REVIEW | adapter + UI |
| RUST-008 | Workspaces, recent repos and My Work | IN_REVIEW | SQLite tests + UI |
| RUST-009 | Actions, notifications and team activity | IN_REVIEW | adapter + UI |
| RUST-010 | Native navigation, shortcuts, drag/drop, dark UI and safety UX | IN_REVIEW | native compile on 3 OSes |
| RUST-011 | Conflicts, stash, worktrees, reflog, recovery, reset/rebase/cherry-pick | IN_REVIEW | safety tests + UI |
| RUST-012 | Quality, security and cross-platform hardening | IN_REVIEW | fmt/clippy/test/audit matrix |
| RUST-013 | Windows portable EXE and Inno Setup installer | IN_REVIEW | package workflow artifact |

## Completion gate

A Rust TODO moves to DONE only when the relevant implementation is present and the final quality/package workflows provide passing evidence. No TODO is marked DONE merely because code was written.
