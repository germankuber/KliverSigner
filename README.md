KliverSigner
============

A minimal, production‑ready Axum service for signing and verifying StarkNet message hashes.

- Async, modular architecture (Axum 0.8 + Tokio 1)
- API‑key auth middleware for protected routes
- Deterministic StarkNet signing via `starknet-signers`
- Signature verification via `starknet-crypto`
- Structured errors and builder‑based responses (`derive_builder`)
- Clean shutdown, tracing, and environment‑first config

Endpoints
- Public
  - `GET` `/health` — service status and metadata.
- Protected (Authorization: `ApiKey <API_KEY>`)
  - `POST` `/signatures` — sign felt hash. Body: `{ "hash": "0x..." }`. Response: `{ "r": "0x...", "s": "0x..." }`.
  - `POST` `/signatures/verify` — verify `(hash, r, s)` and return `{ "is_valid": bool, "public_key": "0x..." }`.
  - `GET` `/signers/self` — return `{ "public_key": "0x..." }`.

Quick Start
- Prereqs: Rust stable + Cargo
- Setup: `cp .env.example .env` and edit variables
- Run: `cargo run`

Curl Examples
- Health: `curl -s http://127.0.0.1:3000/health | jq`
- Sign:
  `curl -s -X POST http://127.0.0.1:3000/signatures -H 'content-type: application/json' -H 'Authorization: ApiKey YOUR_API_KEY' -d '{"hash":"0x1234"}' | jq`
- Verify:
  `curl -s -X POST http://127.0.0.1:3000/signatures/verify -H 'content-type: application/json' -H 'Authorization: ApiKey YOUR_API_KEY' -d '{"hash":"0x1234","r":"0x...","s":"0x..."}' | jq`
- Public key:
  `curl -s http://127.0.0.1:3000/signers/self -H 'Authorization: ApiKey YOUR_API_KEY' | jq`

Configuration
- Required: `API_KEY`, `STARKNET_PRIVATE_KEY`
- Optional: `HOST` (default `0.0.0.0`), `PORT` (default `3000`), `RUST_LOG`

Architecture
- `src/main.rs` — bootstrap, tracing, graceful shutdown
- `src/config.rs` — `AppConfig` from env (secrets redacted)
- `src/error.rs` — error → JSON mapping
- `src/middleware/require_api_key.rs` — `Authorization: ApiKey <key>` guard
- `src/routes/health.rs` — public health endpoint
- `src/routes/sign.rs` — `/signatures`, `/signatures/verify`, `/signers/self`
- `src/routes/mod.rs` — router composition

Design Notes
- Response structs use `derive_builder` for explicit construction and future extensibility.
- Consistent `Felt` types from `starknet-types-core` (v0.2.4) to avoid cross‑crate type drift.
- Signing via `starknet-signers`; verification via `starknet-crypto::verify`.

Security
- Never commit real private keys. Use `.env` only for local dev.
- Add TLS termination and rate limiting if exposed to the internet.

Testing (suggested)
- Unit: felt parsing/encoding helpers.
- Integration: `/health`, `/signatures`, `/signatures/verify`, `/signers/self` with deterministic vectors.

Contributions and feedback are welcome.
