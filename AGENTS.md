# AGENTS.md

## Project

A link shortener built on Cloudflare Workers. The Worker is written in Rust
(`worker` crate + `axum`) and compiled to WASM via `worker-build`. Short links
are stored in Cloudflare KV (`KV` binding).

## Layout

- `src/lib.rs` — all Worker code (router, handlers, business logic)
- `Cargo.toml` — Rust dependencies (`worker`, `axum`, `serde`, `tower-service`)
- `wrangler.toml` — Worker config: name `link`, KV binding `KV`
- `build/` — generated WASM/JS bundle output (committed; produced by `worker-build`)
- `target/`, `.wrangler/` — build artifacts and local state, not committed

## Commands

- Build: `worker-build --release` (wrangler runs this automatically via its
  `build.command` when deploying)
- Dev: `wrangler dev` (local, against real KV)
- Deploy: `wrangler deploy`
- Test: `cargo test` — verify before committing changes

## Routes

- `GET /` — `root`, returns `hello, world!`
- `GET /api/health` — `health`, returns `{ "status": "ok" }`
- `POST /api/shorten` — `shorten`, accepts `{ "url": "..." }`; **TODO** — only
  echoes the URL back, does not persist a short link yet. Redirect handling for
  short codes is not implemented.

## Conventions

- Rust edition 2021, async handlers via `axum`; KV accessed through the `KV`
  binding.
- The router lives in `fn router(kv: KvStore)` and is wired up in the `#[event(fetch)]` handler.
- Keep changes to `src/lib.rs`; do not hand-edit `build/` — it is generated.
- Sole code owner is `@anott03` (see `CODEOWNERS`).

## Notes

- KV namespace id is referenced in `wrangler.toml`; don't rotate it without
  also updating the config.
- The `nvim.log` file at the repo root is generated and gitignored — leave it alone.