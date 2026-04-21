# AGENTS.md

Guidance for AI assistants and contributors working on **boincrs** in this repository.

## Project summary

**boincrs** is a Rust terminal UI (TUI) that talks to a **local BOINC client** over the **GUI RPC** interface (`ratatui`, `crossterm`, `tokio`). It manages projects, tasks, transfers, and client run modes from the keyboard.

## Before you change code

1. Read nearby modules and match existing patterns (errors, async, module layout).
2. Prefer **small, focused diffs** — do not refactor unrelated code.
3. **Production code must not use** `.unwrap()` or `.expect()` under `src/**` (crate denies `clippy::unwrap_used` / `expect_used` on the library).
4. Use **`thiserror`** for errors; avoid `anyhow` in production paths.
5. If you touch BOINC XML parsing, run tests — regressions are easy to introduce.

## Commands

From the repository root:

```bash
cargo fmt
cargo clippy
cargo test
```

For live BOINC integration (optional, ignored tests, needs local daemon + secrets):

```bash
BOINCRS_PASSWORD_FILE=/etc/boinc-client/gui_rpc_auth.cfg \
  cargo test --test live_local_boinc -- --ignored --nocapture
```

PrimeGrid / Asteroids@home attach smoke (requires account keys, see `.env.example`):

```bash
BOINCRS_PASSWORD_FILE=/etc/boinc-client/gui_rpc_auth.cfg \
  BOINCRS_PRIMEGRID_ACCOUNT_KEY='…' \
  BOINCRS_ASTEROIDS_ACCOUNT_KEY='…' \
  cargo test --test live_beta_projects -- --ignored --nocapture
```

## Layout (where things live)

| Area | Path |
| --- | --- |
| Binary entry | `src/main.rs` |
| Library surface | `src/lib.rs` |
| Errors | `src/error.rs` |
| App loop / state | `src/app/` |
| BOINC RPC + parsing | `src/boinc/` |
| TUI | `src/ui/` |
| Tests | `tests/` |
| Human docs | `README.md`, `docs/`, `CHANGELOG.md` |

## Conventions

- **Errors:** `AppResult<T>` / `AppError` in `src/error.rs`.
- **Transport:** `BoincTransport` trait in `src/boinc/transport.rs` — mock-friendly.
- **RPC:** `BoincRpcClient` in `src/boinc/rpc_client.rs` — auth + `call`.
- **Parsing:** `src/boinc/protocol.rs` — tolerant string extraction for real BOINC XML quirks.
- **UI:** `src/ui/layout.rs` renders from `AppState`; do not embed BOINC wire details in widgets.

## Security

- Never commit real `.env` contents, BOINC GUI RPC passwords, or project authenticators.
- Do not paste user secrets into issues, PRs, or agent logs.

## Documentation

- User-facing: `README.md`, `ROADMAP.md`, `CONTRIBUTING.md`, `SUPPORT.md`, `CHANGELOG.md`.
- Architecture / decisions: `docs/architecture/`, `docs/decisions/`.

When behavior changes, update **README** and **CHANGELOG** in the same change when practical.

## Definition of done (for agent PR-sized work)

- `cargo test` passes (includes doctests).
- No new `unwrap`/`expect` in `src/**`.
- Lints clean for touched files.
- Docs updated if user-visible behavior or setup changed.
