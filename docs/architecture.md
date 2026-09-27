# GitGat Architecture

## Goal

GitGat is a desktop-first Git workspace for technical and non-technical teams. The product should make repository activity, files, branches, pull requests, reviews, history, CI and file locking understandable without exposing Git complexity by default.

## Technology baseline

- .NET 10
- Avalonia 12
- SukiUI 7
- CommunityToolkit.Mvvm
- SQLite + EF Core for local app state
- System Git CLI as the source of truth

## Layers

### GitGat.Domain
Pure domain models and invariants. No UI, database, process or provider dependencies.

### GitGat.Application
Use cases and ports such as repository discovery, Git operations, pull requests and file locks. Depends only on Domain.

### GitGat.Infrastructure
Adapters for system Git, Git LFS, GitHub, persistence and platform services. Depends on Application and Domain.

### GitGat.Desktop
Avalonia/SukiUI shell, navigation, view models and composition root. Depends on Application/Infrastructure/Domain.

## Product rules

1. Git operations are represented by application interfaces and implemented with the installed system Git.
2. UI never shells out to Git directly.
3. GitHub/GitLab/Bitbucket are forge adapters behind application interfaces.
4. File locking is a first-class workflow built on Git LFS locking.
5. SQLite stores GitGat state only; repository truth stays in Git.
6. Common workflows stay simple. Rebase/reset/cherry-pick and similar features live under Advanced.
7. The design system belongs to GitGat, not to SukiUI. SukiUI is a component/theme foundation, not the product identity.
