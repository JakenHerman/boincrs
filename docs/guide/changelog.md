---
id: changelog
title: Changelog
sidebar_position: 13
description: Where to find release notes and prebuilt binaries for boincrs.
---

# Changelog

`boincrs` release notes live in
[`CHANGELOG.md`](https://github.com/jakenherman/boincrs/blob/main/CHANGELOG.md)
on GitHub. Prebuilt Linux, macOS, and Windows binaries are attached to each
entry on the [Releases page](https://github.com/jakenherman/boincrs/releases).

Both are generated automatically by
[release-plz](https://release-plz.ieni.dev/) from
[Conventional Commit](https://www.conventionalcommits.org/) messages on
`main` — there is nothing to hand-edit on this page.

## How releases happen

1. Land commits on `main` with conventional subjects (`feat:`, `fix:`,
   `feat!:`, etc.). See the
   [commit-message rules](./contributing.md#commit-messages-conventional-commits)
   in the contributing guide.
2. `release-plz` opens or updates a `chore: release` pull request with the
   next version, a refreshed `Cargo.lock`, and the new `CHANGELOG.md` entry.
3. Merging that PR tags the release and creates the GitHub Release (with
   notes sourced from `CHANGELOG.md`).
4. The tag push triggers the `release` workflow, which builds the three OS
   binaries and attaches them to the same release.

See the [Release checklist](./release-checklist.md) for the pre-merge
compatibility sign-off that runs against the `chore: release` PR.
