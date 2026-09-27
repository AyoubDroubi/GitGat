# GitGat Execution Roadmap

## Phase 0 — Foundation

- [x] .NET 10 solution
- [x] Avalonia 12 desktop project
- [x] SukiUI 7 theme foundation
- [x] CommunityToolkit.Mvvm
- [x] GitGat design tokens
- [x] Clean architecture project boundaries
- [x] System Git process adapter
- [x] Repository shell/navigation
- [x] CI build validation

## Phase 1 — Repository Workspace

- [ ] Open local repository
- [ ] Clone repository
- [ ] Multi-project/repository switcher
- [ ] Persist recent repositories in SQLite
- [ ] Repository overview
- [ ] Working tree file list
- [ ] Stage / unstage / discard
- [ ] Commit
- [ ] Fetch / pull / push
- [ ] Branch switch/create/delete
- [ ] History and commit detail

## Phase 2 — Collaboration

- [ ] Connect GitHub
- [ ] Pull request list/detail
- [ ] Changed files and review comments
- [ ] Approve / request changes / comment
- [ ] Create and merge PRs
- [ ] Notifications / mentions
- [ ] My Work across repositories
- [ ] GitHub Actions runs/jobs/logs

## Phase 3 — File Locking

- [ ] Detect Git LFS
- [ ] Configure lockable patterns
- [ ] Lock file
- [ ] Unlock file
- [ ] Show lock owner
- [ ] Lock status in file tree
- [ ] My locks workspace
- [ ] Push protection / friendly lock conflicts

## Phase 4 — Advanced Git

Advanced features stay out of the default workflow.

- [ ] Rebase
- [ ] Cherry-pick
- [ ] Reset
- [ ] Reflog recovery
- [ ] Worktrees
- [ ] Stashes
- [ ] Conflict editor
- [ ] Submodules

## UX principle

The normal user should be able to work with Projects, Files, Changes, Branches, Pull Requests, History and Locks without needing to understand Git command syntax.
