# GitGat repository instructions

- main is the active Rust development branch.
- dotnet is the immutable historical snapshot of the previous .NET/Avalonia implementation.
- production is reserved. Never change it without explicit authorization in the current conversation.
- The authoritative roadmap is docs/to-do/INDEX.md.
- Git is the source of repository truth; SQLite stores GitGat metadata only.
- Preserve safety invariants documented in docs/ARCHITECTURE.md.
- Do not add force-push or destructive untracked-file deletion to normal workflows.
- Do not store GitHub tokens in source, config files or SQLite.
- A TODO can be marked DONE only with build/test/release evidence.

## No GitHub Actions (mandatory)

GitHub Actions is prohibited in this repository (owner decision, 2026-10-28).

- Never create, restore, or edit files under `.github/workflows/`, and never trigger or rely on workflow runs.
- Never propose GitHub Actions as a CI/CD solution. Deployment goes through the Railway (backend) and Cloudflare Workers/Pages (frontend) Git integrations from the production branch.
- Verify locally (build and tests) before pushing. A broken production deploy gets fixed and redeployed; that is acceptable.
