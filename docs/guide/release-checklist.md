---
id: release-checklist
title: Release checklist
sidebar_position: 11
description: Pre-merge gate for the release-plz "chore release" PR — compatibility sign-off, manual smoke, CI status.
---

# Release checklist

Releases are fully automated by
[release-plz](https://release-plz.ieni.dev/) (see [Changelog](./changelog.md)
for the shipping flow). This checklist is the gate a maintainer runs
**before merging the `chore: release` PR** — merging that PR tags the version
and fires off the prebuilt binary builds.

## Compatibility sign-off gate

Do not merge the `chore: release` PR until all of the following are true:

- [ ] `ci`, `host-matrix`, and `compatibility-matrix` GitHub Actions jobs are
      green on the PR.
- [ ] [Compatibility matrix](./compatibility.md) still matches the BOINC
      branches we intend to support.
- [ ] A live `8.2.x` BOINC daemon smoke has passed on at least one supported
      host OS against the PR's HEAD.
- [ ] If BOINC-facing code changed in this release window, a live legacy
      smoke (`7.16.x` or `7.20.x`) also passed.
- [ ] The [smoke checklist](./architecture/smoke-checklist.md) was completed
      against the release candidate.
- [ ] The `CHANGELOG.md` diff in the `chore: release` PR reads cleanly —
      every entry is a user-recognizable change, and no internal-only noise
      slipped in because a `chore:` commit was mislabeled as `feat:` / `fix:`.

## Manual sign-off record

Capture the release sign-off in a comment on the `chore: release` PR using a
short record like this:

```text
Compatibility sign-off:
- BOINC 8.2.x on <host OS>: PASS/FAIL
- BOINC 7.20.x or 7.16.x on <host OS>: PASS/FAIL (required for BOINC-facing changes)
- Smoke checklist: PASS/FAIL
- Compatibility CI fixtures: PASS/FAIL
```

## Commands

Current-branch live validation:

```bash
BOINCRS_PASSWORD_FILE=/path/to/gui_rpc_auth.cfg \
  cargo test --test live_local_boinc -- --ignored --nocapture
```

Fixture compatibility checks:

```bash
cargo test --test compatibility_matrix_tests
```

Full test + lint pass:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test --locked
cargo test --doc --locked
```

## After you merge

You do not manually tag, publish, or upload binaries — that happens
automatically:

1. Merging the `chore: release` PR causes release-plz to push the `vX.Y.Z`
   tag and create the GitHub Release (notes sourced from `CHANGELOG.md`).
2. The tag push triggers `.github/workflows/release.yml`, which builds the
   Linux / macOS / Windows binaries and attaches them to the release.

Watch the `release` workflow run and confirm all three OS artifacts are
attached. If an OS build fails, re-run that job; the release itself is
already live and is not affected.
