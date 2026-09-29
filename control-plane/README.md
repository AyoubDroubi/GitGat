# GitGat Control Plane

Governance service for GitGat. It does **not** replace the provider Git LFS lock service as lock authority.

## Run

Set `DATABASE_URL` to PostgreSQL and optionally `GITGAT_BIND` (default `127.0.0.1:8080`), then:

```bash
cargo run --manifest-path control-plane/Cargo.toml
```

The service applies embedded SQL migrations at startup.

Endpoints:
- `GET /health` process liveness
- `GET /ready` PostgreSQL readiness
- `GET /api/v1/governance/summary` versioned governance API seed

This is the RUST-019 foundation. Authentication/RBAC and provider enrollment remain RUST-020/021.
