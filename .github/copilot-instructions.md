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
- **Releases:** fully automated by
  [release-plz](https://release-plz.ieni.dev/) from Conventional Commits on
  `main`. See the [Changelog & release
  protocol](#changelog--release-protocol) section below.

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
| Roadmap milestone complete / changed                                                | `docs/guide/roadmap.md`                                                              |
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
- **`CHANGELOG.md` is owned by release-plz.** Do not hand-edit it in a
  feature PR. The file is regenerated from Conventional Commit subjects when
  release-plz opens its `chore: release` PR. The same applies to the
  `version` field in `Cargo.toml` and the matching row in `Cargo.lock`.
- **`docs/guide/changelog.md` is a stub** that links to the GitHub Releases
  page — not a mirror. Do not add release bullets there.
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

## Changelog & release protocol

Releases are **fully automated** by
[release-plz](https://release-plz.ieni.dev/). See `release-plz.toml`,
`.github/workflows/release-plz.yml`, and `.github/workflows/release.yml`.

### The one rule: use Conventional Commits

Every commit that lands on `main` — directly, or as the squash-merge subject
of a PR — **must** use a
[Conventional Commit](https://www.conventionalcommits.org/) subject.
release-plz reads these commits to decide whether a new release is due and
how much to bump the version. PR titles are checked automatically by
`.github/workflows/pr-title.yml`; Copilot should set the PR title to a
valid Conventional Commit from the start to avoid a failing check.

| Subject prefix | User-visible? | Triggers release? | Bump |
| --- | --- | --- | --- |
| `feat: …` | yes | yes | MINOR (e.g. `0.2.0 → 0.3.0`) |
| `fix: …` | yes | yes | PATCH (e.g. `0.2.0 → 0.2.1`) |
| `perf: …` | yes | yes | PATCH |
| `feat!: …` or `BREAKING CHANGE:` footer | yes | yes | MINOR pre-1.0, MAJOR after |
| `docs: …` | no | no | — |
| `refactor:`, `chore:`, `test:`, `ci:`, `style:`, `build:` | no | no | — |

Scopes are optional but encouraged (`feat(ui): …`, `fix(boinc): …`,
`docs(configuration): …`).

The subject **is** the changelog entry. Write it in imperative mood, one
line, no trailing period, no ticket IDs unless they add real context.

Good examples:

- `feat(ui): show checkpoint time in selected-task header`
- `fix(persist): sanitize colons in save filenames on Windows`
- `feat!(boinc): rename BoincTransport::connect to open`

### What Copilot must do

1. **Pick the right prefix.** If the change is user-visible, it must be
   `feat`, `fix`, `feat!`, or `perf` — never `chore` or `refactor`. Default
   to `feat` / `fix` when unsure.
2. **Write the subject like a changelog entry.** It will appear verbatim in
   `CHANGELOG.md`.
3. **Still update `docs/`** in the same PR for any user-visible change (see
   the trigger table above).
4. **Never hand-edit `CHANGELOG.md`, the `version` row in `Cargo.toml`, or
   the matching row in `Cargo.lock`** in a feature PR. release-plz owns
   them.

### How releases actually ship

Informational only — Copilot does not run any of these steps.

1. Conventional commits land on `main`.
2. release-plz opens (or updates) a `chore: release` PR that bumps
   `Cargo.toml`, refreshes `Cargo.lock`, and appends an entry to
   `CHANGELOG.md`.
3. A maintainer reviews and merges the Release PR.
4. release-plz pushes the `vX.Y.Z` tag and creates the GitHub Release.
5. The tag push triggers `release.yml`, which builds Linux / macOS / Windows
   binaries and attaches them to that release.

## Commit and PR hygiene

- **Commit subjects must be Conventional Commits** — see the protocol above.
- A good PR description mentions both the code change and the docs change.
  If a Copilot-opened PR touches user-facing behavior without a matching
  `docs/` diff, treat the PR as incomplete.
- Run `cargo fmt` and `cargo clippy -- -D warnings` locally before pushing
  to avoid CI round-trips.
- Do not bump `version` in `Cargo.toml`, touch `Cargo.lock`'s version rows,
  or edit `CHANGELOG.md` as part of a feature PR — release-plz owns those
  files.
- Never commit `.env` contents, GUI RPC passwords, or project authenticators.

## When uncertain

If Copilot is unsure whether a change is user-visible, default to updating
the docs **and** using `feat` / `fix` for the commit subject. Over-documenting
and over-bulleting the changelog are both cheaper than drift.
