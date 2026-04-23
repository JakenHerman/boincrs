# AGENTS.md

Guidance for AI assistants and contributors working on **boincrs** in this
repository.

## Project summary

**boincrs** is a Rust terminal UI (TUI) that talks to a **local BOINC client**
over the **GUI RPC** interface (`ratatui`, `crossterm`, `tokio`). It manages
projects, tasks, transfers, and client run modes from the keyboard.

## Before you change code

1. Read nearby modules and match existing patterns (errors, async, module
   layout).
2. Prefer **small, focused diffs** — do not refactor unrelated code.
3. **Production code must not use** `.unwrap()` or `.expect()` under `src/**`
   (crate denies `clippy::unwrap_used` / `expect_used` on the library).
4. Use **`thiserror`** for errors; avoid `anyhow` in production paths.
5. If you touch BOINC XML parsing, run tests — regressions are easy to
   introduce.

## Commands

From the repository root:

```bash
cargo fmt
cargo clippy
cargo test
```

For live BOINC integration (optional, ignored tests, needs local daemon +
secrets):

```bash
BOINCRS_PASSWORD_FILE=/etc/boinc-client/gui_rpc_auth.cfg \
  cargo test --test live_local_boinc -- --ignored --nocapture
```

PrimeGrid / Asteroids@home attach smoke (requires account keys, see
`.env.example`):

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
| Repo-root docs | `README.md`, `CONTRIBUTING.md`, `SUPPORT.md`, `ROADMAP.md`, `CHANGELOG.md` |
| Docs site (public) | `docs/` (Docusaurus) → deploys to GitHub Pages |
| Guide pages | `docs/guide/**/*.md` |
| Landing page | `docs/src/pages/index.jsx` |

## Conventions

- **Errors:** `AppResult<T>` / `AppError` in `src/error.rs`.
- **Transport:** `BoincTransport` trait in `src/boinc/transport.rs` —
  mock-friendly.
- **RPC:** `BoincRpcClient` in `src/boinc/rpc_client.rs` — auth + `call`.
- **Parsing:** `src/boinc/protocol.rs` — tolerant string extraction for real
  BOINC XML quirks.
- **UI:** `src/ui/layout.rs` renders from `AppState`; do not embed BOINC wire
  details in widgets.

## Security

- Never commit real `.env` contents, BOINC GUI RPC passwords, or project
  authenticators.
- Do not paste user secrets into issues, PRs, or agent logs.

## Documentation

The canonical, user-facing documentation lives on the **Docusaurus site under
`docs/`**, deployed to GitHub Pages at
[jakenherman.github.io/boincrs](https://jakenherman.github.io/boincrs).

Structure:

- Landing page: `docs/src/pages/index.jsx`
- Guide pages (sidebar docs): `docs/guide/**/*.md` (`intro`, `getting-started`,
  `why-boincrs`, `installation`, `configuration`, `usage`, `keyboard`,
  `accessibility`, `compatibility`, `testing`, `architecture/*`,
  `decisions/*`, `release-checklist`, `roadmap`, `changelog`, `contributing`,
  `support`).
- Config: `docs/docusaurus.config.js`, `docs/sidebars.js`,
  `docs/src/css/custom.css`.
- Deploy workflow: `.github/workflows/docs.yml`.

### Docs stay in sync with code — always

When a change affects user-visible behavior, the docs site **must** be
updated in the same change. This is non-negotiable for PRs, and it applies to
AI-assisted changes exactly the same way it applies to human-authored ones.

**Change → required docs update** (minimum):

| Kind of change                                   | Update at minimum                                                              |
| ------------------------------------------------ | ------------------------------------------------------------------------------ |
| New or changed keybinding                        | `docs/guide/keyboard.md`, `docs/guide/usage.md`                                |
| New or changed env var / CLI flag                | `docs/guide/configuration.md`, `.env.example`                                  |
| New or changed BOINC RPC call / parser behavior  | `docs/guide/compatibility.md`, add/refresh fixtures under `tests/fixtures/`    |
| UI state label / focus cue / confirmation flow   | `docs/guide/usage.md`, `docs/guide/accessibility.md`                           |
| Reconnect / error-handling behavior              | `docs/guide/usage.md`, `docs/guide/decisions/0001-error-handling.md`           |
| Release process / compatibility sign-off         | `docs/guide/release-checklist.md`, `docs/guide/compatibility.md`               |
| New architecture module or controller            | `docs/guide/architecture/**` (add or update page) + sidebar in `sidebars.js`   |
| Roadmap milestone complete / changed             | `docs/guide/roadmap.md`, `docs/guide/changelog.md`, root `CHANGELOG.md`        |
| New major feature surface                        | Add a new guide page under `docs/guide/`, register it in `docs/sidebars.js`    |

If in doubt, **update a docs page**. A PR that changes behavior without any
docs diff is treated as incomplete.

### Docs-only structural rules

- Repo-root `*.md` files (`README.md`, `CONTRIBUTING.md`, `SUPPORT.md`,
  `ROADMAP.md`) stay **short** — they link to the docs site for anything
  long-form. Do not re-expand them into full-length docs; expand the matching
  `docs/guide/*.md` instead.
- `CHANGELOG.md` in the repo root is the canonical source; `docs/guide/changelog.md`
  mirrors it. Update the root file first, then sync the mirror.
- The **`Changelog entry required` CI job** (`.github/workflows/ci.yml` →
  `.github/scripts/check-changelog.sh`) fails any PR that modifies a
  user-visible path (`src/**`, `.env.example`, most `docs/guide/*.md`)
  without adding a new bullet under `## [Unreleased]`. Escape hatches for
  genuinely invisible changes: add the `skip-changelog` label, or put the
  literal token `[skip-changelog]` anywhere in the PR body.
- When you add a new page under `docs/guide/`, register it in
  `docs/sidebars.js` and cross-link it from at least one existing page.
- Preserve existing heading IDs (front-matter `id:`) unless you are doing a
  deliberate, intentional rename — they anchor inbound links.

### Local docs preview

```bash
cd docs
npm install
npm run start   # http://localhost:3000/boincrs/
```

### Docs CI / deploy

`.github/workflows/docs.yml` builds the site on PRs that touch docs and
deploys to GitHub Pages on pushes to `main`. A failing docs build blocks
merges the same way a failing `cargo test` does.

## Definition of done (for agent PR-sized work)

- `cargo test` passes (includes doctests).
- `cargo fmt --check` and `cargo clippy -- -D warnings` are clean.
- No new `unwrap` / `expect` in `src/**`.
- **Docs updated** for any user-visible behavior change (see the mapping
  table above). The docs build (`npm run build` in `docs/`) succeeds.
- `CHANGELOG.md` updated under `## [Unreleased]` if the change is
  user-visible — the `Changelog entry required` CI job will block the merge
  otherwise.
