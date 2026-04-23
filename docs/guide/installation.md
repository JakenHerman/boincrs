---
id: installation
title: Installation
sidebar_position: 4
description: Install boincrs from source on Linux, macOS, or Windows.
---

# Installation

`boincrs` is currently distributed as a **source build**. Binary releases and
packaged installers are tracked on the [roadmap](./roadmap.md).

## Prerequisites

- [BOINC](https://boinc.berkeley.edu/download.php) client (with GUI RPC
  enabled) for the local daemon.
- [Rust](https://rustup.rs/) stable toolchain with `cargo`.
- `git` to clone the repository.

## Build from source

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
