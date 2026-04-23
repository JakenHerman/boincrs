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
2. Make focused changes with clear commit messages.
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

## Changelog entries are required

The `Changelog entry required` CI job (see `.github/workflows/ci.yml` and
`.github/scripts/check-changelog.sh`) **fails any PR** that modifies a
user-visible path — `src/**`, `.env.example`, or a user-facing guide page —
without adding a new bullet under `## [Unreleased]` in `CHANGELOG.md`.

If your change genuinely has no user-visible impact (pure refactor, test
cleanup, CI-only tweak), bypass the check by one of:

- Adding the `skip-changelog` label to the PR, or
- Including the literal token `[skip-changelog]` somewhere in the PR body.

Justify the bypass in the PR description.

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
