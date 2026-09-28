# GitGat

GitGat is a native desktop Git workspace for GitHub and Azure DevOps, written in Rust.

## Current architecture

- Rust 1.98.1 / Rust 2024 edition
- eframe + egui native desktop UI
- system Git CLI as repository source of truth
- Git LFS CLI for lock workflows
- provider facade: GitHub CLI for GitHub and Azure CLI + Azure DevOps extension for Azure Repos/Pipelines
- SQLite via rusqlite for GitGat-only local state
- Inno Setup for the Windows installer

The previous .NET/Avalonia implementation is preserved on the permanent branch named dotnet. The active development branch is main. The reserved production branch is intentionally untouched.

## Safety defaults

GitGat chooses safe Git behavior over shortcuts:

- pull is fast-forward only;
- force push is not exposed;
- branch deletion uses the safe -d form;
- discard never deletes untracked files;
- branch switching and history-changing operations are guarded;
- destructive advanced actions require typed confirmation;
- risky history actions are written to an operation journal;
- GitHub tokens and Azure DevOps PATs are never persisted by GitGat.

## Develop

    cargo fmt --all -- --check
    cargo clippy --all-targets -- -D warnings
    cargo test
    cargo run

Use cargo run -- --self-check for a non-GUI packaging smoke check.

See docs/ARCHITECTURE.md and docs/to-do/INDEX.md.
