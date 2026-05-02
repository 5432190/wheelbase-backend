# AGENTS.md

## Cursor Cloud specific instructions

### Overview

WheelBase is a peer-to-peer RV/camper rental marketplace backend built with Rust (Axum + SQLx + async-stripe). It provides vehicle listings, bookings, Stripe Connect host onboarding, and webhook-driven payment confirmation.

### Services

| Service | Required | Notes |
|---------|----------|-------|
| PostgreSQL 16 | Yes | `postgres://postgres:password@localhost:5432/wheelbase` |
| Rust stable toolchain | Yes | Enforced by `rust-toolchain.toml` (includes clippy + rustfmt) |
| Stripe test API keys | Yes (for full e2e) | App starts and serves non-Stripe routes without valid keys |

### Starting the application

```bash
pg_ctlcluster 16 main start   # if PostgreSQL is not already running
cargo run                      # starts on PORT from .env (default 3000)
```

Migrations run automatically on startup via `sqlx::migrate!()`.

### Lint and format checks

```bash
cargo fmt --check
cargo clippy
```

Note: `cargo clippy -- -D warnings` will fail due to 2 pre-existing minor warnings (single_match and unnecessary_cast) in the existing codebase. Use `cargo clippy` without `-D warnings` for a passing exit code.

### Environment variables

Copy `.env.example` to `.env`. Required vars:
- `DATABASE_URL` — PostgreSQL connection string
- `STRIPE_SECRET_KEY` — Stripe test secret key (app starts without it but Stripe-dependent endpoints will 401)
- `STRIPE_WEBHOOK_SECRET` — for webhook signature verification
- `STRIPE_PLATFORM_ACCOUNT` — Stripe Connect platform account ID

### Key gotchas

- The `.sqlx/` directory only contains a `.gitkeep`; this codebase uses runtime string queries (not compile-time `query!` macros), so `SQLX_OFFLINE` is not needed for builds.
- The app does NOT require `RUST_LOG` to start, but set `RUST_LOG=info` or `RUST_LOG=wheelbase_backend=debug` to see tracing output.
- PostgreSQL must have `uuid-ossp` extension enabled (the migration `CREATE EXTENSION IF NOT EXISTS "uuid-ossp"` handles this if the DB user is superuser).
- Seed data (2 test profiles) is inserted by the migration; do not re-run the migration SQL manually or you'll get unique constraint violations.
