---
id: usage
title: Usage
sidebar_position: 6
description: A tour of the boincrs TUI — panes, task groups, selected-task header, status cues.
---

# Usage

A tour of what `boincrs` shows you once it's connected.

## Layout at a glance

```text
┌ Selected Task ───────────────────────────────────────────────┐
│ project · app · progress · status · elapsed / remaining      │
│ deadline · checkpoint · exit status · task name              │
└──────────────────────────────────────────────────────────────┘
┌ [focus] Projects ┬ Tasks            ┬ Transfers              ┐
│ >> PrimeGrid     │ READY TO REPORT  │ ↑ 12.3 MB 74% 220 KB/s │
│    Asteroids     │ RUNNING (by %)   │ ↓ 4.1 MB retry: 503    │
│    WCG           │ WAITING / READY  │ ↓ 2.8 MB done          │
└──────────────────┴──────────────────┴────────────────────────┘
status: connected · run=auto · net=auto · gpu=auto
```

## The three panes

### Projects

Lists every attached project. Selecting a project scopes per-project action
keys (update, suspend, resume, no-new-work, allow-new-work, detach, reset).
The highlight arrow marks the currently selected project.

### Tasks

Tasks are grouped into three sections in a deliberate order:

1. **READY TO REPORT** — work is done and waiting to upload/report.
2. **RUNNING** — active work, sorted by completion percentage descending.
3. **WAITING / READY** — queued, waiting, suspended, or otherwise inactive.

Selecting a task updates the **Selected Task** header above with extended
metadata.

### Transfers

Each transfer row shows direction (`↑` upload / `↓` download), progress
percentage, human-readable byte totals, current transfer speed, and any error
message from the daemon. Failed transfers can be retried in place.

## Selected Task header

The header above the panes displays the currently selected task in detail:

- Project + application + plan class
- Progress (percent) and textual status (e.g. `[RUN]`, `[WAIT]`, `[REPORT]`)
- Elapsed / remaining CPU time
- Deadline (absolute and relative)
- Checkpoint CPU time (`chkpt`)
- Exit status when the task has completed (`exit`)
- Task name

## Status cues (without color)

- Focused panes show `[focus]` next to the pane title.
- The selected row shows `>>` plus reverse video.
- Task state tags: `[RUN]`, `[WAIT]`, `[READY]`, `[REPORT]`, `[SUSPENDED]`,
  `[ERROR]`.
- Transfer state tags: `[ACTIVE]`, `[IDLE]`, `[ERROR]`.
- Project flags: `[suspended]`, `[no-new-work]`.

Set `NO_COLOR=1` to drop color entirely; the tags and cues stay intact.

## Connection state

The status bar reflects daemon liveness:

- **Connected** — green label; regular refreshes.
- **Retrying** — yellow label with attempt number and countdown to next retry.
  Press `r` to force an immediate reconnect attempt.
- **Terminal error** — bold red label with restart guidance (e.g. auth
  failures, which aren't recoverable by retrying).

See [Architecture: App Controller](./architecture/app-controller.md) for the
event-loop model that drives this.

## Common workflows

### Attach a new project

1. Add the account key to `.env` or `BOINCRS_ATTACH_PROJECTS`.
2. Restart `boincrs`. It will `project_attach` + `project_update` on startup.
3. The project should appear in the Projects pane; its tasks flow into Tasks.

### Update / suspend / resume a project

1. Move focus to Projects (`tab` / `shift-tab`).
2. Select the project with `j` / `k`.
3. Press the action key — `u` update, `s` suspend, `c` resume, `w` no-new-work,
   `a` allow-new-work.

### Destructive actions (detach / reset / abort)

Destructive actions hold a pending state and prompt for confirmation:

- Press `y` to confirm, `n` or `Esc` to cancel.
- Nothing is sent to BOINC until you confirm.

### Retry a failed transfer

1. Focus the Transfers pane.
2. Select the failing transfer.
3. Press `f` to retry.

### Export a diagnostics bundle

Press `D` at any time to write `boincrs-diag-<epoch>.txt` with a snapshot of
client modes, projects, tasks, and transfers. Attach it to bug reports after
reviewing for anything sensitive.

Next: [Keyboard reference](./keyboard.md) for the full set of bindings.
