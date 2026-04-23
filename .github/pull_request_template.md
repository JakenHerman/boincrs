<!--
Thanks for contributing to boincrs!

Full guidance: https://jakenherman.github.io/boincrs/guide/contributing

PR title / commit subject MUST be a Conventional Commit:
  feat(scope): …     (user-visible feature → MINOR)
  fix(scope): …      (user-visible bug fix → PATCH)
  perf(scope): …     (user-visible perf win → PATCH)
  feat!(scope): …    (breaking change)
  docs: …            (docs-only, no release)
  refactor|chore|test|ci|style|build: …   (no release)

release-plz derives CHANGELOG.md and the next version from these subjects.
Do NOT hand-edit CHANGELOG.md, Cargo.toml version, or Cargo.lock version
rows in a feature PR — release-plz owns them.
-->

## Summary

<!-- What does this PR change, and why? 1–3 sentences. -->

## Changes

<!-- Bullet list of the concrete changes. -->

-

## Docs impact

<!--
Pick exactly one of the options below (leave the checkboxes intact so the
reviewer can see what you asserted).

If the PR changes user-visible behavior — keybindings, env vars, UI labels,
BOINC RPC surface, error handling, release flow — you MUST update the
matching page under `docs/guide/**` in this same PR. See AGENTS.md and
.github/copilot-instructions.md for the mapping.
-->

- [ ] Updated `docs/guide/**` page(s):
      <!-- list them, e.g. `docs/guide/keyboard.md`, `docs/guide/usage.md` -->
- [ ] No docs update needed — reason: <!-- e.g. internal refactor, test cleanup, CI-only change -->

## Release impact

<!--
The commit subject / PR title IS the changelog entry. Pick the prefix that
matches the change and write it as a one-line imperative description (no
trailing period, no ticket IDs unless they add context).
-->

- [ ] PR title / squash-merge subject uses a Conventional Commit prefix
      (`feat`, `fix`, `perf`, `feat!`, `docs`, `refactor`, `chore`, `test`,
      `ci`, `style`, `build`).
- [ ] If the change is user-visible, the prefix is `feat` / `fix` / `perf`
      / `feat!` (not `chore` / `refactor`).
- [ ] I did not hand-edit `CHANGELOG.md`, the `version` row in `Cargo.toml`,
      or the version rows in `Cargo.lock`.

## Verification

<!-- What did you run locally? -->

- [ ] `cargo fmt --check`
- [ ] `cargo clippy -- -D warnings`
- [ ] `cargo test`
- [ ] Docs build (`cd docs && npm run build`) — if docs were touched
- [ ] Manual smoke against a local BOINC daemon — if BOINC-facing code changed

## Screenshots / recordings

<!-- Required for UI changes. Paste images or terminal asciinema links. -->
