# Security Exceptions

## RUSTSEC-2023-0071 (`rsa` 0.9.x)

Status: **lockfile-only / inactive build graph**

The Control Plane supports PostgreSQL only. SQLx exposes MySQL as an optional driver; the generated dependency resolution can contain the optional `sqlx-mysql` dependency and its `rsa` dependency even when the MySQL feature is not enabled.

The Control Plane security workflow therefore performs two separate checks:

1. `cargo tree --edges normal -i rsa` must prove that `rsa` is **not** present in the active normal dependency graph.
2. Only after that proof does `cargo audit --ignore RUSTSEC-2023-0071` ignore the lockfile-only advisory.

If `rsa` ever becomes active, CI fails before the ignore is applied.

Review this exception when upgrading SQLx or changing database features.
