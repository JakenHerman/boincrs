# Contributing to boincrs

Thanks for helping improve `boincrs`. The full, up-to-date contributing guide
lives on the docs site:

**➡ [jakenherman.github.io/boincrs/guide/contributing](https://jakenherman.github.io/boincrs/guide/contributing)**

The short version is below; always defer to the docs site if the two disagree.

## Quick start

```bash
git clone https://github.com/jakenherman/boincrs.git
cd boincrs
cp .env.example .env           # edit with your local BOINC settings
cargo test
cargo run
```

## Gate before opening a PR

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## Docs are part of the change

If a PR changes user-visible behavior — keybindings, env vars, UI labels,
BOINC RPC surface, error handling, release flow — it **must** update the
matching page under `docs/guide/**`. Reviewers will ask for this.

See the [Keeping docs in sync table](https://jakenherman.github.io/boincrs/guide/contributing#keeping-docs-in-sync)
for the full mapping of change-kind → required docs page.

## Security

Do not post BOINC passwords, project account keys, or full `.env` contents in
issues or PRs.
