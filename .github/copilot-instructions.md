# GitHub Copilot instructions — boincrs

These instructions apply to **every** Copilot-assisted change in this
repository (Copilot Chat, Copilot coding agent, PR-level suggestions).
They complement `AGENTS.md`, which Copilot should also honor.

## Project context

`boincrs` is a Rust terminal UI (TUI) for a local BOINC client over the GUI
RPC interface. It is keyboard-first, accessibility-aware, and ships with a
public documentation site.

- **Code:** Rust, `ratatui` + `crossterm` + `tokio`, lives under `src/`.
- **Tests:** `tests/` (including fixture-backed BOINC compatibility tests).
- **Docs site (public):** Docusaurus under `docs/`, deployed to
  [jakenherman.github.io/boincrs](https://jakenherman.github.io/boincrs) by
  `.github/workflows/docs.yml`.

## Core code rules

1. **No `.unwrap()` / `.expect()`** in `src/**`. The crate denies
   `clippy::unwrap_used` and `clippy::expect_used`. Use `AppResult<T>` /
   `AppError` (`src/error.rs`) with `?` instead. Tests may use `.expect()`.
2. **Use `thiserror`** for new error variants; do not introduce `anyhow` in
   production paths.
3. **Small, focused diffs.** Do not refactor unrelated code while implementing
   a feature or fix.
4. **Match existing module patterns** for errors, async, and module layout
   before inventing new ones.
5. **Parser changes require fixtures.** Any change to
   `src/boinc/protocol.rs` or related XML handling needs (a) a fixture under
   `tests/fixtures/compatibility/` and (b) a passing
   `cargo test --test compatibility_matrix_tests` run.
6. **Typed state → UI.** `src/ui/**` only consumes `AppState`. Do not embed
   BOINC wire details in widgets.

## Required verification before proposing a change

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

For BOINC-facing code, also run:

```bash
cargo test --test compatibility_matrix_tests
```

If you cannot run these, explicitly note that in the PR description so the
reviewer knows to run them.

## Documentation is part of the change — enforce this

Copilot **must** produce a docs update in the same PR whenever code changes
affect user-visible behavior. A code-only PR that changes behavior is
incomplete and should not be proposed.

### Trigger table — when code change X lands, docs page Y must update

| If the change touches…                                                             | Update at minimum                                                                    |
| ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| Key handling in `src/app/**` or `src/ui/**` (new / changed keybinding)              | `docs/guide/keyboard.md`, `docs/guide/usage.md`                                      |
| `src/main.rs`, `src/app/**` env var loading, or `.env.example`                      | `docs/guide/configuration.md`, `.env.example`                                        |
| `src/boinc/rpc_client.rs`, `src/boinc/transport.rs`, `src/boinc/protocol.rs`, auth  | `docs/guide/compatibility.md`, fixtures under `tests/fixtures/compatibility/`        |
| Selected-task header, focus cues, status labels, confirmation flow (`src/ui/**`)    | `docs/guide/usage.md`, `docs/guide/accessibility.md`                                 |
| Reconnect / backoff / error classification (`src/error.rs`, controller)             | `docs/guide/usage.md`, `docs/guide/decisions/0001-error-handling.md`                 |
| New architecture module or significant controller refactor                          | `docs/guide/architecture/**` (new or updated page) + `docs/sidebars.js`              |
| Roadmap milestone complete / changed                                                | `docs/guide/roadmap.md`, `docs/guide/changelog.md`, root `CHANGELOG.md`              |
| New supported BOINC target / dropped target                                         | `docs/guide/compatibility.md`, `docs/guide/release-checklist.md`, CI matrix          |
| Release process / sign-off gate                                                     | `docs/guide/release-checklist.md`                                                    |
| Diagnostics bundle format / output path                                             | `docs/guide/usage.md` (section on diagnostics bundle)                                |
| New major feature surface                                                           | Add a new page under `docs/guide/`, register in `docs/sidebars.js`, link from `usage.md` |

If the change does not map to any row above (for example, an internal
refactor with no user-visible effect), state that explicitly in the PR body
under a short "Docs impact: none — <reason>" line. Copilot should default to
assuming a docs update **is** needed unless this reasoning is explicit.

### Docs-side rules Copilot must follow

- **Repo-root Markdown stays short.** `README.md`, `CONTRIBUTING.md`,
  `SUPPORT.md`, and `ROADMAP.md` are pointers to the docs site. Do not
  re-expand them into long-form docs — expand the corresponding
  `docs/guide/*.md` instead, and have the root file link to it.
- **`CHANGELOG.md` is canonical.** Update the root `CHANGELOG.md` first,
  then mirror the changes in `docs/guide/changelog.md` in the same PR. The
  `Changelog entry required` GitHub Actions job will fail the PR if a
  user-visible path changes without a new bullet under `## [Unreleased]`.
  Bypass only when truly not user-visible by adding the `skip-changelog`
  label or `[skip-changelog]` in the PR body — and justify the bypass in
  the PR description.
- **New guide pages must be registered** in `docs/sidebars.js` under
  `guideSidebar` and cross-linked from at least one existing page.
- **Preserve front-matter `id:` values** and existing heading anchors. Renaming
  an `id:` breaks inbound links — avoid unless intentional.
- **Prefer relative links** inside `docs/guide/**` (e.g. `./usage.md` rather
  than absolute `https://jakenherman.github.io/...`). This keeps local dev
  working and lets Docusaurus rewrite them.
- **Run `npm run build`** in `docs/` locally when a change is large enough to
  affect multiple pages, to confirm no broken links. The docs workflow will
  also check this on CI.

### Style guidance for docs pages

- Start with a one-sentence lede under the H1 that explains the page's scope.
- Use fenced code blocks with language tags (` ```bash`, ` ```rust`,
  ` ```toml`, ` ```text`).
- Prefer tables for option / keybinding / env-var reference content — it is
  easier to scan than prose.
- Use admonitions (`:::note`, `:::warning`) sparingly, for things a reader
  really needs to not miss.
- Keep imperative tone ("Run …", "Set …") in step-by-step guides.
- Link back to related pages instead of duplicating content.

## Commit and PR hygiene

- Commit message: `<area>: <short imperative summary>` where `<area>` is
  something like `ui`, `boinc`, `app`, `docs`, `ci`, `errors`, etc.
- Use the repository's PR template (`.github/pull_request_template.md`) and
  fill in every section — **Summary**, **Changes**, **Docs impact**,
  **Changelog**, **Verification**. Don't delete the template; answer it.
- If the change is user-visible, the PR must:
  - Update the matching `docs/guide/**` page(s).
  - Add a bullet under `## [Unreleased]` in `CHANGELOG.md`.
- Never commit `.env` contents, GUI RPC passwords, or project authenticators.

## When uncertain

If Copilot is unsure whether a change is user-visible, default to updating
the docs. Over-documenting is cheaper than drift.
