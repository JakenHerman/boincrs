# Contributing to boincrs

Thanks for helping improve `boincrs`.

## Development setup

1. Install Rust via [rustup](https://rustup.rs/).
2. Clone the repository:
   ```bash
   git clone https://www.github.com/jakenherman/boincrs.git
   cd boincrs
   ```
3. Copy environment template:
   ```bash
   cp .env.example .env
   ```
4. Edit `.env` with your local BOINC settings.
5. Build and test:
   ```bash
   cargo test
   cargo run
   ```

## Contribution flow

1. Create a feature branch.
2. Make focused changes with clear commit messages.
3. Ensure tests pass:
   ```bash
   cargo test
   ```
4. Update docs if behavior changed (`README.md` and relevant docs under `docs/`).
5. Open a pull request with:
   - change summary
   - test notes
   - screenshots/GIFs for UI changes

## Code guidelines

- Keep production code free of `.unwrap()`/`.expect()` (tests may use them).
- Prefer typed errors (`thiserror`) and explicit handling.
- Favor reusable module boundaries over UI-specific coupling.
- Keep terminal rendering stable for narrow widths where practical.

## Testing guidance

- Unit and integration:
  ```bash
  cargo test
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

## Reporting bugs

Please include:
- OS + terminal emulator
- `boincrs` version/commit
- steps to reproduce
- expected vs actual behavior
- relevant screenshot(s)
- sanitized logs/output

## Security

Do not post BOINC passwords, project account keys, or full `.env` contents in issues/PRs.
