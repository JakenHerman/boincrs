<!--
Thanks for contributing to boincrs!

Full guidance: https://jakenherman.github.io/boincrs/guide/contributing
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

## Changelog

<!--
The CI job `Changelog entry required` fails any PR that touches user-visible
code or docs without adding a bullet under `## [Unreleased]` in CHANGELOG.md.

If the change is not user-visible, tick the escape-hatch box below and also
add the `skip-changelog` label to the PR (or include `[skip-changelog]`
anywhere in this body).
-->

- [ ] Added a `## [Unreleased]` entry to `CHANGELOG.md`.
- [ ] No changelog entry needed (`skip-changelog`) — reason: <!-- why -->

## Verification

<!-- What did you run locally? -->

- [ ] `cargo fmt --check`
- [ ] `cargo clippy -- -D warnings`
- [ ] `cargo test`
- [ ] Docs build (`cd docs && npm run build`) — if docs were touched
- [ ] Manual smoke against a local BOINC daemon — if BOINC-facing code changed

## Screenshots / recordings

<!-- Required for UI changes. Paste images or terminal asciinema links. -->
