# GitGat Rust Architecture

## Product boundary

GitGat is a native desktop Git client. Repository state belongs to Git. GitHub credentials belong to GitHub CLI, while Azure DevOps authentication is delegated to Azure CLI / Microsoft Entra ID. GitGat persists only application metadata such as recent repositories, favorites, workspaces and recovery journal entries; it does not persist provider access tokens or Azure DevOps PATs.

## Modules

- domain.rs: data contracts with no process or UI behavior.
- process.rs: shell-free process execution and credential redaction.
- git.rs: safe system-Git and Git-LFS adapter.
- forge.rs: provider facade for GitHub and Azure DevOps. GitHub uses gh for PRs, reviews, Actions, notifications and activity. Azure DevOps uses az/azure-devops for Azure Repos PRs and Azure Pipelines, with REST gaps invoked through the authenticated Azure CLI.
- store.rs: SQLite catalog, workspaces and operation journal.
- ui.rs: eframe/egui desktop shell and product workflows.
- main.rs: composition root and packaging self-check.

## Invariants

1. Commands are launched without a shell; arguments remain separate.
2. Git remains the source of truth; no shadow Git graph is stored in SQLite.
3. Pull is fast-forward only.
4. No force-push operation exists in normal product workflows.
5. Discard refuses untracked files.
6. Local branch deletion uses git branch -d, never -D.
7. Dirty worktrees block branch switching, rebase, cherry-pick and hard reset.
8. Hard reset, rebase, cherry-pick, stash pop/drop and recovery actions are separated under Advanced.
9. Risky operations are written to a durable local journal.
10. Provider authentication is delegated to gh or az; GitGat never stores GitHub tokens or Azure DevOps PATs.
11. GitHub/token-like process output is redacted before it is surfaced.
12. production is not changed without explicit user authorization.

## Cross-platform intent

The desktop runtime is native on Windows, macOS and Linux. CI validates compilation and tests on all three operating systems. Windows is the first packaged distribution target.
