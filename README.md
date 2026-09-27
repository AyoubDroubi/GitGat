# GitGat

GitGat is a modern, cross-platform desktop Git workspace focused on making Git understandable for both technical and non-technical teams.

## Foundation

- .NET 10 (LTS)
- Avalonia 12
- SukiUI 7
- CommunityToolkit.Mvvm
- Clean Architecture with a desktop-first feature structure
- System Git as the source of truth
- Git LFS locking for binary/asset workflows
- SQLite + EF Core for local application state

Development starts from feature branches. The `main` branch remains the integration baseline.

## Product change tracking

GitGat uses the repository-native governance system under [docs/to-do](docs/to-do/README.md).

- [Global TODO index](docs/to-do/INDEX.md)
- [Master implementation plan](docs/to-do/MASTER-PLAN.md)
- [Operational activity ledger](docs/to-do/operations/ACTIVITY-LEDGER.md)

Normal development targets `main`. Release/stable branch actions are handled separately only when explicitly authorized.
