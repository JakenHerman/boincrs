---
id: release-process
title: Release process & versioning
sidebar_position: 10
description: Release cadence, branch and tag strategy, SemVer policy, RC-to-stable promotion, and changelog policy for boincrs.
---

# Release process & versioning

This page is the **policy**: when and how `boincrs` releases happen, and what
the version numbers mean. For the operational, pre-merge sign-off gate see the
[Release checklist](./release-checklist.md); for where the notes and binaries
land see [Changelog](./changelog.md).

## Cadence

`boincrs` releases **as needed**, not on a fixed calendar. A release is cut
whenever release-worthy commits (`feat:`, `fix:`, `perf:`) have landed on `main`
and a maintainer decides to ship them. There is no minimum or maximum interval —
small, frequent releases are preferred over large batched ones.

## Branch & tag strategy

Trunk-based, with `main` always releasable:

1. Work happens on short-lived feature branches.
2. Each branch opens a pull request and is **squash-merged** into `main` after
   CI is green. The PR title becomes the commit subject on `main`, so it must be
   a valid [Conventional Commit](https://www.conventionalcommits.org/) (enforced
   by the `PR title` workflow).
3. [release-plz](https://release-plz.ieni.dev/) watches `main` and keeps a
   `chore: release` PR open with the next version, an updated `Cargo.lock`, and
   the generated `CHANGELOG.md` entry.
4. Merging the `chore: release` PR pushes the annotated tag `vX.Y.Z` and creates
   the GitHub Release.

The only long-lived ref is `main`. Version tags (`vX.Y.Z`) are immutable once
pushed — a mistake is corrected by cutting a new patch, never by moving a tag.

## Versioning (SemVer)

`boincrs` follows [Semantic Versioning](https://semver.org/). release-plz maps
Conventional Commit types to bumps:

| Change on `main` | Pre-1.0 (`0.y.z`) | Post-1.0 (`x.y.z`) |
| --- | --- | --- |
| `fix:` / `perf:` | patch | patch |
| `feat:` | minor | minor |
| `feat!:` / `BREAKING CHANGE:` | minor | **major** |

Under `0.y.z` the API/UX is not yet considered stable, so breaking changes only
bump the minor. Publishing **1.0.0** is a deliberate maintainer decision that
declares the CLI, configuration surface, and BOINC RPC behavior stable enough to
promise backward compatibility under SemVer.

## Release-candidate → stable promotion

Most releases go straight to stable. When a change is risky enough to warrant a
soak, cut a pre-release tag `vX.Y.Z-rc.N`:

1. Land the candidate commits on `main`.
2. Tag `vX.Y.Z-rc.1`; validate against the [Release checklist](./release-checklist.md)
   (live BOINC smoke on a supported host, compatibility matrix, etc.).
3. If issues are found, fix on `main` and cut `-rc.2`, and so on.
4. **Promote** by cutting the final `vX.Y.Z` — the same commit range, now without
   the pre-release suffix — once the checklist passes with no open blockers.

For **1.0.0** specifically, promotion also requires the
[1.0 readiness tracker](https://github.com/jakenherman/boincrs/issues/10) exit
criteria to be met and validated.

## Changelog policy

- `CHANGELOG.md` is **generated**, never hand-edited. Its entries come verbatim
  from Conventional Commit subjects via release-plz, following
  [Keep a Changelog](https://keepachangelog.com/) sections.
- **Categorization** is driven by the commit type: `feat:` → *Added*/*Changed*,
  `fix:` → *Fixed*, `perf:` → *Performance*. Types that are not user-visible
  (`docs:`, `refactor:`, `chore:`, `test:`, `ci:`, `style:`, `build:`) do not
  produce changelog entries.
- **Breaking changes** must be flagged with a `!` after the type (`feat!:`) or a
  `BREAKING CHANGE:` footer. release-plz surfaces these prominently in the
  release notes so downstream users see them before upgrading.
- Because the subject *is* the changelog entry, write subjects in imperative
  mood, one line, no trailing period. See the commit-message rules in the
  [Contributing guide](./contributing.md#commit-messages-conventional-commits).

## Who does what

| Step | Owner | Automated? |
| --- | --- | --- |
| Merge feature PRs to `main` | Maintainer | title lint + CI gate |
| Keep the `chore: release` PR current | release-plz | yes |
| Run the pre-merge sign-off | Maintainer | [checklist](./release-checklist.md) |
| Tag + create the GitHub Release | release-plz (on PR merge) | yes |
| Build and attach the release artifacts | `release` workflow | yes |
