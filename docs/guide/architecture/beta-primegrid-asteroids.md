---
id: beta-primegrid-asteroids
title: PrimeGrid + Asteroids@home (beta)
sidebar_position: 2
description: Auto-attach flow for the first two beta-supported BOINC projects.
---

# Beta: PrimeGrid + Asteroids@home

`boincrs` can auto-attach projects on startup when account keys are provided
through environment variables.

## Startup auto-attach env vars

- `BOINCRS_PRIMEGRID_ACCOUNT_KEY` — PrimeGrid account key
- `BOINCRS_ASTEROIDS_ACCOUNT_KEY` — Asteroids@home account key
- `BOINCRS_ATTACH_PROJECTS` — optional custom list of the form
  `https://example.com/boinc/|account_key;https://example2.com/|account_key2`
- `BOINCRS_ATTACH_TEMPLATES` — curated-slug list of the form
  `primegrid|KEY1;rosetta|KEY2`
  (see [Project templates & profiles](./project-templates-and-profiles.md))
- `BOINCRS_PROFILE_FILE` — path to a preset profile bundling attach entries
  and run / network / GPU mode overrides

When any of these are provided, startup performs:

1. `project_attach`
2. `project_update`

## Live beta verification test

```bash
BOINCRS_PASSWORD_FILE=/etc/boinc-client/gui_rpc_auth.cfg \
BOINCRS_PRIMEGRID_ACCOUNT_KEY=... \
BOINCRS_ASTEROIDS_ACCOUNT_KEY=... \
  cargo test --test live_beta_projects -- --ignored --nocapture
```

The test verifies both projects appear in BOINC project status and inspects
tasks for target project URLs.

## Expected beta UI behavior after attach

- PrimeGrid and Asteroids@home appear in the Projects pane.
- Tasks appear in grouped sections:
  - `READY TO REPORT`
  - `RUNNING`
  - `WAITING / READY`
- Task ordering prioritizes report-ready tasks, then running tasks by progress.
- Selecting a task with `j` / `k` or arrow keys updates the top
  **Selected Task** panel.

See [Usage](../usage.md) for the surrounding pane model and
[Configuration](../configuration.md) for the full set of env vars.
