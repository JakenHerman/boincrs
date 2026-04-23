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
| Roadmap milestone complete / changed             | `docs/guide/roadmap.md`                                                        |
| New major feature surface                        | Add a new guide page under `docs/guide/`, register it in `docs/sidebars.js`    |

If in doubt, **update a docs page**. A PR that changes behavior without any
docs diff is treated as incomplete.

### Docs-only structural rules

- Repo-root `*.md` files (`README.md`, `CONTRIBUTING.md`, `SUPPORT.md`,
  `ROADMAP.md`) stay **short** — they link to the docs site for anything
  long-form. Do not re-expand them into full-length docs; expand the matching
  `docs/guide/*.md` instead.
- `CHANGELOG.md` is **owned by release-plz** (see
  [§ Changelog & release protocol](#changelog--release-protocol) below). Do
  not hand-edit it in a feature PR. `docs/guide/changelog.md` is a stub that
  points readers at the GitHub Releases page; it is not a mirror.
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

## Changelog & release protocol

`CHANGELOG.md` and version bumps are **fully automated** by
[`release-plz`](https://release-plz.ieni.dev/) (see `release-plz.toml` and
`.github/workflows/release-plz.yml`). Agents and contributors **do not
hand-edit** any of:

- `CHANGELOG.md`
- `version` in `Cargo.toml`
- Version rows in `Cargo.lock`
- `docs/guide/changelog.md` (a stub that links to the Releases page)

Those files are regenerated from commit messages when release-plz opens its
`chore: release` PR. Touching them in a feature PR will cause merge conflicts
against the Release PR — don't.

### The one rule: use Conventional Commits

Every commit that lands on `main` (directly or via squash-merge, whichever
the PR uses) **must** use a
[Conventional Commit](https://www.conventionalcommits.org/) subject. This is
how release-plz knows a release is needed and what kind of version bump to
cut. PR titles are checked automatically by
`.github/workflows/pr-title.yml` — a PR with a non-conforming title cannot
be merged.

| Subject prefix | User-visible? | Triggers release? | Bump |
| --- | --- | --- | --- |
| `feat: …` | yes | yes | MINOR (e.g. `0.2.0 → 0.3.0`) |
| `fix: …` | yes | yes | PATCH (e.g. `0.2.0 → 0.2.1`) |
| `perf: …` | yes | yes | PATCH |
| `feat!: …` or `BREAKING CHANGE:` footer | yes | yes | MINOR pre-1.0, MAJOR after |
| `docs: …` | no | no | — |
| `refactor:`, `chore:`, `test:`, `ci:`, `style:`, `build:` | no | no | — |

Scopes are optional but encouraged for clarity, e.g.
`feat(scoring): add balk reason to runner advance`.

Examples that are good:

- `feat(ui): show checkpoint time in selected-task header`
- `fix(persist): sanitize colons in save filenames on Windows`
- `feat!(boinc): rename BoincTransport::connect to open`
- `docs(configuration): document BOINCRS_PROFILE_FILE`
- `chore: bump ratatui to 0.30`

The commit subject **is** the changelog entry. Write it in imperative mood,
one line, no trailing period, no ticket numbers unless they add context.

### What agents must do

1. **Pick the right prefix.** If the change is user-visible, it must be
   `feat`, `fix`, `feat!`, or `perf` — never `chore` or `refactor`. When in
   doubt, prefer `feat` or `fix`; a missing bullet is worse than an extra
   one.
2. **Write the subject like a changelog entry.** It will appear verbatim in
   `CHANGELOG.md`.
3. **Still update `docs/` in the same PR** for any user-visible change (see
   the mapping table above). The commit-message bullet is the changelog;
   the docs site is the reference manual. They are separate obligations.
4. **Do not touch `CHANGELOG.md`, `release-plz.toml`, or the version in
   `Cargo.toml`** except in a deliberate repo-maintenance PR.

### How releases actually ship

This is informational — agents don't run any of these steps themselves.

1. Conventional commits land on `main`.
2. The `release-plz` workflow opens (or updates) a `chore: release` PR that
   bumps `Cargo.toml`, refreshes `Cargo.lock`, and appends an entry to
   `CHANGELOG.md`.
3. A maintainer reviews and merges the Release PR.
4. `release-plz` pushes the `vX.Y.Z` tag and creates the GitHub Release.
5. The tag push triggers `.github/workflows/release.yml`, which builds
   Linux / macOS / Windows binaries and attaches them to the release.

## Definition of done (for agent PR-sized work)

- `cargo test` passes (includes doctests).
- `cargo fmt --check` and `cargo clippy -- -D warnings` are clean.
- No new `unwrap` / `expect` in `src/**`.
- **Docs updated** for any user-visible behavior change (see the mapping
  table above). The docs build (`npm run build` in `docs/`) succeeds.
- The commit subject (or PR title, if squash-merging) is a valid
  [Conventional Commit](https://www.conventionalcommits.org/). No hand edits
  to `CHANGELOG.md` or the `version` row in `Cargo.toml` / `Cargo.lock`.
