# GitGat Master Implementation Plan

This is the ordered product path from the current foundation to a runnable Windows desktop artifact.

> Chat is input. Repository is memory.
>
> This plan does not bypass per-TODO lifecycle gates. Each TODO must become approved/ready and receive explicit start authorization before implementation.

## Branch policy

- Active development: `main`.
- Normal product work remains on `main`.
- `production` exists as a reserved stable branch and is not modified until Ayoub explicitly authorizes a production action.

## Phase 0 — Foundation closure

1. [TODO-001 — Desktop foundation & re-architecture](./TODO-001-foundation-rearchitecture/)
   - Existing .NET 10 / Avalonia 12 / SukiUI 7 foundation is on main.
   - Release build CI already passed.
   - Remaining gate: independent review + reconstructed governance closeout.

## Phase 1 — Daily local Git workflow

2. [TODO-002 — Repository onboarding, clone & local catalog](./TODO-002-repository-onboarding-catalog/)
   - Open/clone repositories.
   - SQLite recent repository catalog.
   - Search/favorites/missing-path recovery.

3. [TODO-003 — Files, changes, staging & commit](./TODO-003-working-tree-changes-commit/)
   - Working tree.
   - Diffs.
   - Stage/unstage.
   - Commit.
   - Guarded discard.

4. [TODO-004 — Sync, branches & history](./TODO-004-sync-branches-history/)
   - Fetch/pull/push.
   - Ahead/behind.
   - Branch management.
   - History/commit detail.
   - Divergence/conflict detection.

## Phase 2 — GitHub collaboration

5. [TODO-005 — GitHub identity/auth & forge abstraction](./TODO-005-github-identity-auth-forge/)
   - Secure connection.
   - OS credential storage.
   - Provider abstraction.
   - Auth health/reconnect.

6. [TODO-006 — Pull requests, reviews, comments & merge](./TODO-006-pull-requests-reviews-comments/)
   - Full PR loop in-app.
   - Changed files/comments.
   - Approve/request changes.
   - Create/edit/merge.

7. [TODO-007 — Git LFS file locking](./TODO-007-git-lfs-file-locking/)
   - Lock/unlock.
   - Owner/status badges.
   - My Locks.
   - Guided lockable patterns.

## Phase 3 — Multi-project/team workspace

8. [TODO-008 — Multi-repository workspaces & My Work](./TODO-008-workspaces-my-work/)
   - Logical workspace containing multiple repositories.
   - Cross-repo PR/review/mention inbox.

9. [TODO-009 — Actions, notifications & team activity](./TODO-009-actions-notifications-activity/)
   - GitHub Actions runs/jobs/logs.
   - Safe workflow actions.
   - Notifications and meaningful activity.

## Phase 4 — Product quality of experience

10. [TODO-010 — GitGat design system, UX & accessibility](./TODO-010-design-system-ux-accessibility/)
    - GitGat-owned visual system over SukiUI.
    - Consistent navigation/components/states.
    - Keyboard/focus/accessibility.
    - Dark-first modern desktop polish.

11. [TODO-011 — Recovery, conflicts & Advanced Git](./TODO-011-recovery-conflicts-advanced-git/)
    - Conflict editor/flow.
    - Stashes.
    - Rebase/cherry-pick/reset under Advanced.
    - Worktrees.
    - Reflog/recovery.
    - Operation journal.

## Phase 5 — Release readiness

12. [TODO-012 — Quality, security & cross-platform hardening](./TODO-012-quality-cross-platform-hardening/)
    - Unit/integration tests.
    - Disposable Git repository fixtures.
    - CI platform matrix.
    - Large-repo performance/cancellation.
    - Security/privacy/destructive-operation review.
    - Full independent review and fixes.

## Phase 6 — Actual desktop program artifact

13. [TODO-013 — Windows executable & installer](./TODO-013-windows-exe-installer-artifact/)
    - Release `win-x64` self-contained publish.
    - Verify single-file `GitGat.exe` where compatible.
    - Produce free-tooling installer `GitGat-Setup-x64.exe` using Inno Setup or NSIS.
    - Correct app name/icon/version/install/uninstall.
    - SHA-256/checksum.
    - Clean-machine smoke test.
    - Register artifact in `operations/ARTIFACT-REGISTRY.md`.

## Definition of the first usable distributable

The roadmap reaches its first concrete delivery milestone when evidence proves:

- `GitGat.exe` launches on a clean supported Windows x64 machine without a separate .NET installation;
- the app can open a repository and detect System Git;
- core daily repository workflows required by release scope pass validation/review;
- `GitGat-Setup-x64.exe` installs/uninstalls cleanly;
- artifact commit/version/checksum are registered;
- no claim is made about production release/deployment until a separate explicit production decision.

## Packaging references

- Avalonia Windows deployment: https://docs.avaloniaui.net/xpf/deployment/windows
- Avalonia Windows packaging options: https://docs.avaloniaui.net/tools/parcel/packaging-for-windows

The plan intentionally uses standard .NET publish plus free installer tooling for the first artifact; paid packaging tooling is not required.
