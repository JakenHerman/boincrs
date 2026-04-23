---
id: getting-started
title: Getting started
sidebar_position: 3
description: Install boincrs, point it at your local BOINC daemon, and run it end-to-end.
---

# Getting started

This walks you from zero to a running `boincrs` TUI connected to your local
BOINC daemon. It takes about five minutes on a machine that already has BOINC
installed.

## 1. Prerequisites

- A local [BOINC](https://boinc.berkeley.edu/download.php) client running with
  **GUI RPC enabled**.
- The **GUI RPC password** — typically in `gui_rpc_auth.cfg` (on Linux often at
  `/etc/boinc-client/gui_rpc_auth.cfg`).
- [Rust](https://rustup.rs/) (stable) + `cargo`.
- A reachable BOINC endpoint (default `127.0.0.1:31416`).
- Optional: **project authenticator keys** to auto-attach projects.

:::note
The GUI RPC password is **not** the same as your project website passwords.
Find it in the `gui_rpc_auth.cfg` file that BOINC writes during install.
:::

## 2. Clone and build

```bash
git clone https://github.com/jakenherman/boincrs.git
cd boincrs
cargo build --release
```

The release binary lands at:

- Linux / macOS: `./target/release/boincrs`
- Windows: `.\target\release\boincrs.exe`

## 3. Configure the environment

Copy the template and fill in what applies:

```bash
cp .env.example .env
```

Minimum fields for a local run:

```bash
BOINCRS_ENDPOINT=127.0.0.1:31416
BOINCRS_PASSWORD_FILE=/etc/boinc-client/gui_rpc_auth.cfg
```

Optional auto-attach (see [Configuration](./configuration.md) for the full list):

```bash
BOINCRS_PRIMEGRID_ACCOUNT_KEY=your_primegrid_authenticator
BOINCRS_ASTEROIDS_ACCOUNT_KEY=your_asteroids_authenticator
```

`boincrs` **auto-loads `.env`** at startup if one exists in the working
directory. You can also `source` it manually:

```bash
set -a && source .env && set +a
```

## 4. Run it

```bash
cargo run
# or
./target/release/boincrs
```

You should see three panes — Projects, Tasks, Transfers — plus a
**Selected Task** header at the top.

## 5. Learn the core keys

| Keys                       | Action                                             |
| -------------------------- | -------------------------------------------------- |
| `tab` / `shift-tab`        | Move focus between panes                           |
| `left` / `right`           | Same as above                                      |
| `j` / `k` or arrows        | Move selection within the focused pane             |
| `r`                        | Refresh (or force reconnect when retrying)         |
| `q`                        | Quit                                               |
| `y` / `n` / `Esc`          | Confirm / cancel destructive actions               |

Full list: [Keyboard reference](./keyboard.md).

## 6. Verify

From the repo root:

```bash
cargo test
```

See [Testing](./testing.md) for live-daemon and integration options.

## Troubleshooting

- **Auth fails on connect.** Confirm `BOINCRS_PASSWORD_FILE` points at your
  `gui_rpc_auth.cfg` and that BOINC's GUI RPC is listening on the endpoint.
- **Blank panes.** Press `r` to force a refresh; if the daemon is down, the UI
  will show retrying state with a countdown.
- **Colors look odd.** Set `NO_COLOR=1`; `boincrs` uses text labels for all
  critical state so nothing important relies on color.
- **Need a snapshot for a bug report.** Press `D` in the TUI to write a
  `boincrs-diag-<epoch>.txt` file.

Next: [Configuration](./configuration.md) or [Usage](./usage.md).
