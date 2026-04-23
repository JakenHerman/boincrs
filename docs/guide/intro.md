---
id: intro
title: Introduction
sidebar_position: 1
description: boincrs is a Rust terminal UI for a local BOINC client over the GUI RPC interface.
---

# boincrs

`boincrs` is a keyboard-first Rust **terminal UI** that drives your **local
BOINC client** over the GUI RPC interface. It is designed to be the fastest way
to understand, attach, and manage projects on a machine that already runs the
BOINC daemon.

## Status

**Beta.** Active development targets BOINC `7.16.x`, `7.20.x`, and `8.2.x`. See
[Compatibility](./compatibility.md) for the full support matrix.

## What it does

- Renders a three-pane TUI for **Projects · Tasks · Transfers**.
- Shows a **Selected Task** header with progress, deadline, checkpoint, exit
  status, and app identity.
- Supports **auto-attach** for PrimeGrid and Asteroids@home via account keys.
- Handles **reconnect with exponential backoff** if the daemon drops.
- Exports a **diagnostics bundle** on demand for bug reports.
- Remains readable without color, without mouse, and across dark/light themes.

## What it is not

- Not a remote BOINC dashboard. Local GUI RPC only, for now.
- Not a project chooser or recommender — you still pick projects you trust.
- Not a replacement for BOINC Manager — it is a complement for keyboard-driven
  workflows, SSH sessions, and minimal desktops.

## Where to go next

- [Why boincrs](./why-boincrs.md) — motivation and design goals.
- [Getting started](./getting-started.md) — install, configure, run.
- [Keyboard reference](./keyboard.md) — every keybinding in one place.
- [Architecture](./architecture/app-controller.md) — how the pieces fit together.
