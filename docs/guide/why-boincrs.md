---
id: why-boincrs
title: Why boincrs?
sidebar_position: 2
description: Why a Rust terminal UI for BOINC exists, and what it optimizes for.
---

# Why boincrs?

BOINC already runs as a background daemon on your machine — it talks to a local
**GUI RPC** endpoint (usually `127.0.0.1:31416`) that any client can use to
drive it. The official BOINC Manager is a GUI app; `boincrs` is a **TUI for
the same daemon**, for people who live in the terminal.

## The problems it solves

### 1. You want to check on BOINC over SSH

BOINC Manager is a desktop application. If you're watching a headless Linux box
over SSH, you either tail logs, shell out to `boinccmd`, or don't check at all.
`boincrs` renders a full live view of the daemon in the same terminal you're
already in.

### 2. You want a keyboard-first interface

BOINC Manager is fine with a mouse. If your daily workflow is keyboard-centric
(tmux, vim, tiling WMs, screen readers), a TUI removes context-switches. Every
project, task, transfer, and mode action in `boincrs` is reachable with a
single keystroke.

### 3. You want something predictable for old hardware

Many BOINC hosts are older machines donating spare cycles. Spinning up an Electron
dashboard or even the native Manager adds overhead that takes away from the
work the host exists to do. A Rust TUI runs anywhere you already have a
terminal.

### 4. You want accessibility without "accessibility mode"

Rather than a separate high-contrast skin, `boincrs` keeps textual state
labels like `[RUN]`, `[REPORT]`, `[ACTIVE]`, `[ERROR]` in every mode. Color is
additive, never load-bearing. `NO_COLOR=1` strips color without hiding any
signal. See [Accessibility](./accessibility.md).

## Design goals

- **Keyboard-first.** Navigation via `tab`/`shift-tab`, `j`/`k`, arrows.
  Actions via single-letter keys. Destructive actions always confirm.
- **Readable in any terminal theme.** No color-only state, no assumptions about
  light/dark/high-contrast palettes.
- **Predictable errors.** Typed `AppError` via `thiserror`. No `.unwrap()` /
  `.expect()` in `src/**` (enforced by the crate). See the
  [Error handling decision record](./decisions/0001-error-handling.md).
- **Resilient to flaky daemons.** Bounded exponential backoff with jitter,
  connection state surfaced in the UI, force-retry key in retrying state.
- **Parser tolerance.** BOINC's XML changes across versions. Parsers are
  fixture-backed for `7.16`, `7.20`, and `8.2`.
- **Small, focused diffs.** No framework churn, no heavy abstraction.

## What it deliberately avoids

- **Remote BOINC management.** GUI RPC is a sensitive auth surface; remote
  profiles are on the [roadmap](./roadmap.md) but deliberately out of scope today.
- **Project recommendations.** `boincrs` trusts the user to pick projects.
- **Custom color palettes.** Terminal themes already own color. `boincrs`
  inherits them.

If any of the above resonates, head to [Getting started](./getting-started.md).
