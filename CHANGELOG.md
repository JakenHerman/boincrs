# Changelog

All notable changes to `boincrs` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Reconnect backoff: transient RPC failures now trigger bounded exponential backoff
  (1 s → 30 s, ±25% jitter) rather than a hard crash or silent stall. The daemon
  connection is automatically re-established when the daemon returns.
- Connection state tracking: `AppState` now carries a `ConnectionState` variant
  (`Connected`, `Retrying`, `TerminalError`) so the UI always reflects liveness.
- Actionable error UI: the status bar renders retrying state in yellow (with attempt
  count and next-retry countdown) and fatal errors in bold red with restart guidance.
  Press `r` to force an immediate reconnect attempt while in retrying state.
- Error classification: `AppError::is_transient()` distinguishes recoverable failures
  (I/O, protocol framing, invalid response) from terminal ones (auth failure, UI).
- 15 integration tests covering error classification, backoff bounds, and the full
  reconnect state-machine (transient failure → retrying → recovery → connected).
- Transfer visibility: each transfer now shows direction (↑/↓), progress percentage,
  human-readable byte counts, current transfer speed, and error message when present.
- Selection cursor in Projects and Transfers panes — `j/k` and arrow keys now navigate
  all three panes, and project/transfer actions apply to the highlighted item.
- Richer task metadata: `checkpoint_cpu_time`, `received_time`, and `exit_status` are
  now parsed from BOINC results. `chkpt` and `exit` appear in the Selected Task header.
- Diagnostics bundle export: press `D` to write a `boincrs-diag-<epoch>.txt` snapshot
  of all current state (client modes, projects, tasks, transfers) for bug reports.
- Automatic `.env` loading: if a `.env` file exists in the working directory, boincrs
  reads it at startup and applies any keys not already set in the environment.

### Fixed
- Corrected `compute_nonce_hash` doctest expected value.

### Changed
- Project and transfer panes now use stateful list rendering with a `▶` highlight symbol,
  consistent with the Tasks pane.
- Selected task detail header condensed to fit `chkpt` field alongside existing fields.

---

## [0.1.0-beta] - 2026-04-20

### Added
- Initial Rust TUI foundation for local BOINC GUI RPC management.
- Multi-pane UI for projects, tasks, and transfers.
- Selected task detail header with progress, timing, deadline, and app info.
- Keyboard navigation (`tab`, `j/k`, arrow keys) and action keybindings.
- Status-aware task rendering with colorized icons and grouped headings.
- PrimeGrid and Asteroids@home attach flow via project authenticators.
- Project/task/transfer action commands and mode toggles.
- Docs baseline: README, roadmap, contributing, support, architecture notes.

### Changed
- Task ordering now prioritizes:
  1. ready-to-report
  2. running (by completion descending)
  3. waiting/ready tasks
- Layout refined to emphasize task visibility and selected-task context.
- Parser hardened for BOINC XML variations and flag-style tags.

### Fixed
- Auth handling aligned to nonce-hash (`md5(nonce + password)`).
- Support for `ready_to_report` self-closing flags (`<ready_to_report/>`).
- Improved resilience against nested/extra result tags in real BOINC payloads.
