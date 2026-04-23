---
id: accessibility
title: Accessibility & theming
sidebar_position: 8
description: How boincrs stays readable across dark, light, and high-contrast terminal themes.
---

# Accessibility & theming

`boincrs` is designed to stay readable regardless of your terminal's color
scheme, font, or emulator quirks.

## Principles

- **Text labels first.** Every meaningful state has a textual tag —
  `[RUN]`, `[WAIT]`, `[READY]`, `[REPORT]`, `[ACTIVE]`, `[IDLE]`, `[ERROR]`,
  `[suspended]`, `[no-new-work]`. Color is additive, never load-bearing.
- **Focus without color.** The focused pane shows `[focus]` next to its title.
  The selected row uses `>>` plus reverse video, so focus stays visible in
  monochrome terminals too.
- **Terminal theme wins.** `boincrs` inherits foreground and background from
  your emulator. It does not ship a custom palette or attempt to detect
  "light" vs "dark" themes.

## `NO_COLOR` support

Setting the `NO_COLOR` environment variable disables all color accents:

```bash
NO_COLOR=1 cargo run
```

Everything else — labels, reverse-video selection, focus markers, keyboard
cues — stays the same. The UI remains legible in terminals with limited color
support or monochrome settings.

## Keyboard-only workflow

- Move pane focus with `tab` / `shift-tab` or `left` / `right`.
- Move within a pane with `j` / `k` or `up` / `down`.
- Invoke actions via single-letter keys (see [Keyboard reference](./keyboard.md)).
- Confirm destructive actions with `y`; cancel with `n` or `Esc`.

No action requires the mouse.

## Known constraints

- Terminal applications cannot reliably detect whether the emulator uses a
  light, dark, or high-contrast theme. `boincrs` favors default
  foreground/background colors plus explicit labels over theme-specific
  palettes.
- Reverse video, bold, and underline rendering can vary between emulators and
  fonts, especially over SSH and inside multiplexers like `tmux`. Text labels
  keep the meaning stable even when styling is inconsistent.
- `boincrs` does not currently expose a per-widget palette override.
  Accessibility is driven by your terminal theme and the `NO_COLOR`
  convention.

## Testing accessibility locally

Run through the [smoke checklist](./architecture/smoke-checklist.md) at least
once in:

1. Your normal theme.
2. A light theme (with a light-background terminal).
3. `NO_COLOR=1`.

Each run should still produce a readable view with visible focus and selection
cues.
