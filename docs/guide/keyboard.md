---
id: keyboard
title: Keyboard reference
sidebar_position: 7
description: Every boincrs keybinding in one place.
---

# Keyboard reference

`boincrs` is keyboard-driven. Every binding below works from anywhere in the
TUI except where noted. Destructive actions always prompt for confirmation
(`y` / `n` / `Esc`).

## Navigation

| Keys                  | Action                                       |
| --------------------- | -------------------------------------------- |
| `tab` / `shift-tab`   | Move focus forward / backward across panes   |
| `left` / `right`      | Move pane focus backward / forward           |
| `j` / `down`          | Move selection down within the focused pane  |
| `k` / `up`            | Move selection up within the focused pane    |

## Global

| Keys               | Action                                                                 |
| ------------------ | ---------------------------------------------------------------------- |
| `r`                | Refresh — or force an immediate reconnect when in retrying state       |
| `q`                | Quit and restore the terminal                                          |
| `D`                | Export a `boincrs-diag-<epoch>.txt` diagnostics bundle                 |
| `y` / `n` / `Esc`  | Confirm / cancel a pending destructive action                          |

## Projects pane

Select a project first, then press an action key.

| Key | Action                                           |
| --- | ------------------------------------------------ |
| `u` | Update project                                   |
| `s` | Suspend project                                  |
| `c` | Resume project                                   |
| `w` | Set project to no-new-work                       |
| `a` | Allow new work                                   |
| `x` | Detach project *(destructive — prompts)*          |
| `d` | Reset project *(destructive — prompts)*           |

## Tasks pane

Select a task first, then press an action key.

| Key | Action                                           |
| --- | ------------------------------------------------ |
| `t` | Suspend the selected task                        |
| `g` | Resume the selected task                         |
| `b` | Abort the selected task *(destructive — prompts)* |

## Transfers pane

| Key | Action                         |
| --- | ------------------------------ |
| `f` | Retry the selected transfer    |

## Client run modes

Mode keys toggle the corresponding BOINC run / network / GPU mode on the
daemon. The status bar reflects the current effective mode:

| Key | Mode set                                         |
| --- | ------------------------------------------------ |
| `1` | Run mode: always                                 |
| `2` | Run mode: auto (follows preferences)             |
| `3` | Run mode: never                                  |
| `4` | Network mode: always                             |
| `5` | Network mode: auto                               |
| `6` | Network mode: never                              |
| `7` | GPU mode: always                                 |
| `8` | GPU mode: auto                                   |
| `9` | GPU mode: never                                  |

See [Usage](./usage.md) for what each pane surfaces and
[Accessibility](./accessibility.md) for how keybindings and visual cues work
together.
