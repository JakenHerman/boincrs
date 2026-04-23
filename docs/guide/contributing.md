---
id: contributing
title: Contributing
sidebar_position: 14
description: How to set up a dev environment for boincrs and submit changes upstream.
---

# Contributing

Thanks for helping improve `boincrs`.

## Development setup

1. Install Rust via [rustup](https://rustup.rs/).
2. Clone the repository:
   ```bash
   git clone https://github.com/jakenherman/boincrs.git
   cd boincrs
   ```
3. Copy the environment template:
   ```bash
   cp .env.example .env
   ```
4. Edit `.env` with your local BOINC settings
   (see [Configuration](./configuration.md)).
5. Build and test:
   ```bash
   cargo test
   cargo run
   ```

## Contribution flow

1. Create a feature branch.
2. Make focused changes with clear commit messages that follow
   [Conventional Commits](https://www.conventionalcommits.org/) (see
   [Commit messages](#commit-messages-conventional-commits) below).
3. Ensure the full gate passes:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   ```
4. **Update docs if behavior changed.** This includes the
   [docs site](https://jakenherman.github.io/boincrs) under `docs/guide/**`
   (not just `README.md`). See
   [Keeping docs in sync](#keeping-docs-in-sync) below.
5. Open a pull request with:
   - change summary
   - test notes
   - screenshots / GIFs for UI changes

## Code guidelines

- Keep production code free of `.unwrap()` / `.expect()` (tests may use them).
  Enforced at the crate level via `#![deny(clippy::unwrap_used,
  clippy::expect_used)]`.
- Prefer typed errors (`thiserror`) and explicit handling. See the
  [error-handling ADR](./decisions/0001-error-handling.md).
- Favor reusable module boundaries over UI-specific coupling.
- Keep terminal rendering stable for narrow widths where practical.

## Testing guidance

- Unit and integration tests:
  ```bash
  cargo test
  cargo test --test compatibility_matrix_tests
  ```
- Local BOINC daemon integration:
  ```bash
  BOINCRS_PASSWORD_FILE=/etc/boinc-client/gui_rpc_auth.cfg \
    cargo test --test live_local_boinc -- --ignored --nocapture
  ```
- PrimeGrid + Asteroids beta attach test:
  ```bash
  BOINCRS_PASSWORD_FILE=/etc/boinc-client/gui_rpc_auth.cfg \
  BOINCRS_PRIMEGRID_ACCOUNT_KEY='YOUR_PRIMEGRID_KEY' \
  BOINCRS_ASTEROIDS_ACCOUNT_KEY='YOUR_ASTEROIDS_KEY' \
    cargo test --test live_beta_projects -- --ignored --nocapture
  ```

If you touch BOINC protocol parsing, auth, transport, or refresh / controller
behavior:

- Refresh the compatibility fixtures under `tests/fixtures/compatibility/`.
- Update [Compatibility](./compatibility.md) when support expectations
  change.

## Keeping docs in sync

`boincrs` treats the docs site and the code as a single unit of work. Any of
the following PR shapes require a docs update in the same PR:

| Kind of change                                     | Update at minimum                                                            |
| -------------------------------------------------- | ---------------------------------------------------------------------------- |
| New or changed keybinding                          | [Keyboard reference](./keyboard.md), [Usage](./usage.md)                     |
| New or changed environment variable                | [Configuration](./configuration.md), `.env.example`                          |
| New or changed BOINC RPC call / parser             | [Compatibility](./compatibility.md), fixtures under `tests/fixtures/`        |
| UI state label / focus cue / confirmation flow     | [Usage](./usage.md), [Accessibility](./accessibility.md)                     |
| Reconnect / error-handling behavior                | [Usage](./usage.md), [ADR 0001](./decisions/0001-error-handling.md)          |
| Release process / gates                            | [Release checklist](./release-checklist.md)                                  |

Reviewers will ask for docs updates if they are missing. See `AGENTS.md` and
`.github/copilot-instructions.md` in the repo for the automation side of this.

## Commit messages: Conventional Commits

Commit subjects on `main` (and PR titles, if the PR is squash-merged) **must**
follow [Conventional Commits](https://www.conventionalcommits.org/). Releases
are fully automated by [release-plz](https://release-plz.ieni.dev/), which
reads commit subjects to decide whether to cut a release and how much to
bump the version.

| Prefix | User-visible? | Triggers release? | Bump |
| --- | --- | --- | --- |
| `feat:` | yes | yes | MINOR |
| `fix:` | yes | yes | PATCH |
| `perf:` | yes | yes | PATCH |
| `feat!:` or `BREAKING CHANGE:` footer | yes | yes | MINOR pre-1.0, MAJOR after |
| `docs:` | no | no | — |
| `refactor:`, `chore:`, `test:`, `ci:`, `style:`, `build:` | no | no | — |

Scopes are optional but encouraged for clarity
(`feat(ui): …`, `fix(boinc): …`). The subject is the changelog entry — write
it in imperative mood, one line, no trailing period. Examples:

- `feat(ui): show checkpoint time in selected-task header`
- `fix(persist): sanitize colons in save filenames on Windows`
- `feat!(boinc): rename BoincTransport::connect to open`

## Files release-plz owns — do not hand-edit

These files are regenerated by release-plz when the `chore: release` PR is
opened; hand-editing them in a feature PR creates merge conflicts against
that PR.

- `CHANGELOG.md`
- `version = "…"` in `Cargo.toml`
- The version rows in `Cargo.lock`
- `docs/guide/changelog.md` (a stub that links to the Releases page)

See [Changelog](./changelog.md) for where to find release notes and prebuilt
binaries.

## Reporting bugs

Please include:

- OS + terminal emulator
- `boincrs` version / commit
- steps to reproduce
- expected vs actual behavior
- relevant screenshot(s)
- sanitized logs / output (use `D` in the TUI to export a diagnostics bundle
  first and scrub anything sensitive before attaching)

## Security

Do not post BOINC passwords, project account keys, or full `.env` contents in
issues / PRs.
