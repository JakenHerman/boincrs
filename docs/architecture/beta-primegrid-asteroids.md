# Beta: PrimeGrid + Asteroids@home

`boincrs` can auto-attach projects on startup when account keys are provided through environment variables.

## Startup auto-attach env vars

- `BOINCRS_PRIMEGRID_ACCOUNT_KEY`: PrimeGrid account key
- `BOINCRS_ASTEROIDS_ACCOUNT_KEY`: Asteroids@home account key
- `BOINCRS_ATTACH_PROJECTS`: optional custom list in the form:
  - `https://example.com/boinc/|account_key;https://example2.com/|account_key2`

When any of these are provided, startup performs:
1. `project_attach`
2. `project_update`

## Live beta verification test

Run:

`BOINCRS_PASSWORD_FILE=/etc/boinc-client/gui_rpc_auth.cfg BOINCRS_PRIMEGRID_ACCOUNT_KEY=... BOINCRS_ASTEROIDS_ACCOUNT_KEY=... cargo test --test live_beta_projects -- --ignored --nocapture`

The test verifies both projects appear in BOINC project status and inspects tasks for target project URLs.

## Expected beta UI behavior after attach

- PrimeGrid and Asteroids@home should appear in the Projects pane.
- Tasks should appear in grouped sections:
  - `READY TO REPORT`
  - `RUNNING`
  - `WAITING / READY`
- Task ordering should prioritize report-ready tasks, then running tasks by progress.
- Selecting a task with `j/k` or arrow keys should update the top `Selected Task` panel.
