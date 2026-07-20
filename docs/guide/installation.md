---
id: installation
title: Installation
sidebar_position: 4
description: Install boincrs from source on Linux, macOS, or Windows.
---

# Installation

`boincrs` ships as **package-manager installs**, **prebuilt binaries**, and a
**source build**. Pick whichever fits your platform.

## Homebrew (macOS / Linux)

```bash
brew tap jakenherman/boincrs https://github.com/jakenherman/boincrs
brew install boincrs
```

Upgrade later with `brew upgrade boincrs`. The formula installs the prebuilt
binary for your architecture (Apple Silicon, Intel macOS, or x86-64 Linux) and
verifies it against the release checksum.

## Chocolatey (Windows)

```powershell
choco install boincrs
```

Upgrade later with `choco upgrade boincrs`. The package downloads the Windows
release zip, verifies its SHA256, and puts `boincrs` on your `PATH`.

## Prebuilt binaries

Every [GitHub Release](https://github.com/jakenherman/boincrs/releases) attaches
a tarball (Linux/macOS) or zip (Windows) plus a `checksums.txt`. Download the
asset for your platform, verify it, and place `boincrs` somewhere on your
`PATH`:

```bash
# Example: Linux x86-64
curl -LO https://github.com/jakenherman/boincrs/releases/latest/download/checksums.txt
# download the matching boincrs-<version>-x86_64-unknown-linux-gnu.tar.gz, then:
sha256sum -c checksums.txt --ignore-missing
tar xzf boincrs-*-x86_64-unknown-linux-gnu.tar.gz
install -m 0755 boincrs ~/.local/bin/boincrs
```

## Verify the install

```bash
boincrs --version
```

## Build from source

Prerequisites:

- [BOINC](https://boinc.berkeley.edu/download.php) client (with GUI RPC
  enabled) for the local daemon.
- [Rust](https://rustup.rs/) stable toolchain with `cargo`.
- `git` to clone the repository.

```bash
git clone https://github.com/jakenherman/boincrs.git
cd boincrs
cargo build --release
```

Binary output:

- Linux / macOS: `./target/release/boincrs`
- Windows: `.\target\release\boincrs.exe`

You can also run without building a release binary:

```bash
cargo run
```

## Platform notes

### Linux

```bash
sudo apt install boinc-client boinc-manager  # or your distro equivalent
# ensure GUI RPC is enabled and note the path to gui_rpc_auth.cfg
```

The default password file is often `/etc/boinc-client/gui_rpc_auth.cfg`.
Depending on your distribution and permissions you may need to `sudo cat` it or
run `boincrs` as a user in the `boinc` group.

### macOS

Install [BOINC](https://boinc.berkeley.edu/download.php) via the official
installer or Homebrew. The GUI RPC password file lives inside the BOINC app
data directory managed by the installer.

### Windows

Install [BOINC](https://boinc.berkeley.edu/download.php) and install Rust via
[rustup](https://rustup.rs/). GUI RPC is enabled by default; the password file
lives under the BOINC data folder (for example under `C:\ProgramData\BOINC`).

## Verify

```bash
cargo test
```

See [Testing](./testing.md) for live-daemon and compatibility-matrix tests.

Next: [Configuration](./configuration.md).
