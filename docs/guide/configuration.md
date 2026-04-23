---
id: configuration
title: Configuration
sidebar_position: 5
description: Environment variables, password files, and project auto-attach for boincrs.
---

# Configuration

`boincrs` is configured entirely through environment variables. When a `.env`
file exists in the working directory it is loaded automatically at startup;
existing environment variables always win.

## Environment variables

| Variable                           | Required? | Purpose                                                                 |
| ---------------------------------- | --------- | ----------------------------------------------------------------------- |
| `BOINCRS_ENDPOINT`                 | Optional  | BOINC GUI RPC endpoint. Default: `127.0.0.1:31416`.                     |
| `BOINCRS_PASSWORD`                 | Either/or | GUI RPC password as a literal value. Prefer `BOINCRS_PASSWORD_FILE`.    |
| `BOINCRS_PASSWORD_FILE`            | Either/or | Path to `gui_rpc_auth.cfg`. Preferred over inlining the password.       |
| `BOINCRS_PRIMEGRID_ACCOUNT_KEY`    | Optional  | PrimeGrid account authenticator for auto-attach on startup.             |
| `BOINCRS_ASTEROIDS_ACCOUNT_KEY`    | Optional  | Asteroids@home account authenticator for auto-attach on startup.        |
| `BOINCRS_ATTACH_PROJECTS`          | Optional  | Custom attach list: `url1\|key1;url2\|key2`.                             |
| `NO_COLOR`                         | Optional  | When set, disables color accents. Labels and cues remain unchanged.     |

## GUI RPC password

Use BOINC's **GUI RPC** password, not project website passwords.

- On Linux the file is commonly `/etc/boinc-client/gui_rpc_auth.cfg`.
- On macOS and Windows the path depends on your BOINC install location.
- Either point `BOINCRS_PASSWORD_FILE` at the file or set `BOINCRS_PASSWORD`
  directly (avoid the latter in committed configs).

## Project auto-attach

When any of `BOINCRS_PRIMEGRID_ACCOUNT_KEY`, `BOINCRS_ASTEROIDS_ACCOUNT_KEY`,
or `BOINCRS_ATTACH_PROJECTS` is set, `boincrs` performs the equivalent of
`project_attach` followed by `project_update` for each entry on startup.

### PrimeGrid

Grab your account key from the PrimeGrid
[project preferences page](https://www.primegrid.com/prefs.php?subset=project).
It is listed as **account key** / **authenticator**, not your password.

### Asteroids@home

Grab your account key from the Asteroids@home
[account page](https://asteroidsathome.net/boinc/home.php).

### Custom attach list

`BOINCRS_ATTACH_PROJECTS` takes one or more `url|key` pairs separated by `;`:

```bash
BOINCRS_ATTACH_PROJECTS="https://www.primegrid.com/|PRIMEGRID_KEY;https://asteroidsathome.net/boinc/|ASTEROIDS_KEY"
```

See [PrimeGrid + Asteroids beta](./architecture/beta-primegrid-asteroids.md)
for the exact attach flow and verification test.

## `.env` template

```bash
BOINCRS_ENDPOINT=127.0.0.1:31416
BOINCRS_PASSWORD_FILE=/etc/boinc-client/gui_rpc_auth.cfg
BOINCRS_PRIMEGRID_ACCOUNT_KEY=PUT_PRIMEGRID_AUTHENTICATOR_HERE
BOINCRS_ASTEROIDS_ACCOUNT_KEY=PUT_ASTEROIDS_AUTHENTICATOR_HERE
```

:::warning Security
Never commit real `.env` contents, password files, or project authenticators.
`boincrs` treats these as secrets; you should too. See
[Support & security](./support.md).
:::

Next: [Usage](./usage.md).
