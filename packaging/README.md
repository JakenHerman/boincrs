# Packaging

This directory holds the package-manager integrations for boincrs. Both are
driven automatically from a tagged release by
[`.github/workflows/release.yml`](../.github/workflows/release.yml), and both
can be exercised locally by a maintainer.

All packages install the **prebuilt binaries** attached to the matching GitHub
Release, and pin them to the SHA256 checksums published in that release's
`checksums.txt`.

## Homebrew (`homebrew/`)

The repository doubles as a Homebrew tap. On each release the workflow renders
[`homebrew/render.sh`](homebrew/render.sh) into `Formula/boincrs.rb` (at the
repo root) with the release version and per-platform tarball checksums, then
commits it to `main`.

Users install with:

```bash
brew tap jakenherman/boincrs https://github.com/jakenherman/boincrs
brew install boincrs
```

`Formula/boincrs.rb` first appears when the initial stable release is cut — it
is machine-generated, so it is intentionally not hand-authored ahead of time.

Local dry run:

```bash
VERSION=1.0.0 \
  SHA_LINUX=<sha> SHA_ARM=<sha> SHA_INTEL=<sha> \
  bash packaging/homebrew/render.sh
```

## Chocolatey (`chocolatey/`)

[`chocolatey/build.ps1`](chocolatey/build.ps1) substitutes the version and the
Windows-zip checksum into the `.nuspec` and install templates, then runs
`choco pack`. The workflow attaches the resulting `.nupkg` to the release and,
when a `CHOCO_API_KEY` secret is configured, pushes it to the Chocolatey
community feed.

Users install with:

```powershell
choco install boincrs
```

Local build (requires Chocolatey CLI):

```powershell
pwsh packaging/chocolatey/build.ps1 -Version 1.0.0 -Sha256 <windows-zip-sha256>
# -> packaging/chocolatey/out/boincrs.1.0.0.nupkg
```

The `out/` directory and any `*.nupkg` are git-ignored.

## Adding a maintainer secret

`CHOCO_API_KEY` (repo Actions secret) enables automatic publishing to the
Chocolatey community feed. Without it, the release still produces and attaches a
`.nupkg` that can be pushed manually with `choco push`.
