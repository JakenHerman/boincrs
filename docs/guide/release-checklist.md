---
id: release-checklist
title: Release checklist
sidebar_position: 11
description: Pre-merge gate for the release-plz "chore release" PR — compatibility sign-off, manual smoke, CI status.
---

# Release checklist

Releases are fully automated by
[release-plz](https://release-plz.ieni.dev/) (see [Changelog](./changelog.md)
for the shipping flow, and [Release process](./release-process.md) for the
cadence, versioning, and RC-to-stable policy). This checklist is the gate a
maintainer runs **before merging the `chore: release` PR** — merging that PR
tags the version and fires off the prebuilt binary builds.

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
2. The tag push triggers `.github/workflows/release.yml`, which:
   - builds the Linux / macOS / Windows binaries,
   - attaches them plus a `checksums.txt` (SHA256) to the release,
   - regenerates `Formula/boincrs.rb` and commits it to `main` (Homebrew tap),
   - builds the Chocolatey `.nupkg`, attaches it, and — when the
     `CHOCO_API_KEY` secret is set — pushes it to the community feed.

Confirm each of these after the workflow finishes:

- [ ] All four platform archives (linux, windows, mac arm64, mac x86_64) and
      `checksums.txt` are attached to the release.
- [ ] The `homebrew` job committed an updated `Formula/boincrs.rb` on `main`
      with the new version and checksums.
- [ ] `brew tap jakenherman/boincrs https://github.com/jakenherman/boincrs &&
      brew install boincrs` installs the new version and `boincrs --version`
      reports it.
- [ ] The `chocolatey` job attached `boincrs.<version>.nupkg`. If publishing,
      `choco install boincrs` (or the pending community-feed approval) reflects
      the new version.

If an OS build fails, re-run that job; the release itself is already live and is
not affected. The `homebrew` and `chocolatey` jobs can also be re-run
independently — both are idempotent (the formula commit is skipped when
unchanged, and the `.nupkg` upload uses `--clobber`).

> **First release only.** `Formula/boincrs.rb` does not exist until the first
> tagged release creates it, and Chocolatey community-feed pushes require the
> `CHOCO_API_KEY` secret. See [`packaging/README.md`](https://github.com/jakenherman/boincrs/tree/main/packaging)
> for local dry-runs and the one-time secret setup.
