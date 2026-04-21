![boincrs banner](boincrs.jpg)

# boincrs

Terminal UI for a **local BOINC client** over the GUI RPC interface. **Beta** — PrimeGrid and Asteroids@home auto-attach via account keys are supported.

## Requirements

- [BOINC](https://boinc.berkeley.edu/download.php) client running locally with GUI RPC enabled
- [Rust](https://rustup.rs/) (stable) and `cargo`
- Reachable BOINC endpoint (default `127.0.0.1:31416`)
- GUI RPC password (often `/etc/boinc-client/gui_rpc_auth.cfg` on Linux), or `BOINCRS_PASSWORD`
- Optional: project **authenticator** keys for auto-attach

## Install from source

```bash
git clone https://www.github.com/jakenherman/boincrs.git
cd boincrs
cargo build --release
```

Binary: `./target/release/boincrs` (Linux/macOS) or `.\target\release\boincrs.exe` (Windows).

**Linux:** install Rust and BOINC, then build as above.  
**macOS / Windows:** install Rust and [BOINC](https://boinc.berkeley.edu/download.php), then the same clone/build steps.

## Learn more

| Topic | Link |
| --- | --- |
| BOINC | [boinc.berkeley.edu](https://boinc.berkeley.edu/) |
| Downloads | [boinc.berkeley.edu/download.php](https://boinc.berkeley.edu/download.php) |
| Projects | [boinc.berkeley.edu/projects.php](https://boinc.berkeley.edu/projects.php) |
| Open science (background) | [Wikipedia: Open science](https://en.wikipedia.org/wiki/Open_science) |
| PrimeGrid | [primegrid.com](https://www.primegrid.com/) |
| Asteroids@home | [asteroidsathome.net/boinc](https://asteroidsathome.net/boinc/) |

## Beta UI (at a glance)

- **Panes:** Projects | Tasks | Transfers  
- **Header:** selected task — project, progress, status, elapsed/remaining, deadline, application, name; plus client run/network/gpu summary  
- **Task groups:** `READY TO REPORT` → `RUNNING` (by % done) → `WAITING / READY`  
- **Navigation:** `tab` cycles panes; with Tasks focused, `j`/`k` or arrow keys move selection  

## Configuration

### GUI RPC password

Use the BOINC **GUI RPC** password, not project website passwords. Set `BOINCRS_PASSWORD` or point `BOINCRS_PASSWORD_FILE` at your `gui_rpc_auth.cfg`.

### Project authenticators (attach)

Use each project’s **account key** from its BOINC account pages — not the web login password.

- **PrimeGrid:** [Project prefs](https://www.primegrid.com/prefs.php?subset=project)  
- **Asteroids@home:** [Account / home](https://asteroidsathome.net/boinc/home.php)  

### Environment

```bash
cp .env.example .env
# edit .env, then:
set -a && source .env && set +a   # Linux/macOS
```

With `BOINCRS_PRIMEGRID_ACCOUNT_KEY` / `BOINCRS_ASTEROIDS_ACCOUNT_KEY` set, startup runs `project_attach` then `project_update` for those projects.

### Run

```bash
cargo run
# or: ./target/release/boincrs
```

### Keyboard

| Keys | Action |
| --- | --- |
| `tab` | Cycle Projects / Tasks / Transfers |
| `j` `k` / arrows | Move task selection (Tasks pane) |
| `r` | Refresh |
| `q` | Quit |
| `y` / `n` | Confirm / cancel destructive actions |
| `u/s/c/w/a/x/d` | Project actions |
| `t/g/b` | Task actions |
| `f` | Retry transfer |
| `1`–`9` | Run / network / GPU modes |

## Testing & verification

```bash
cargo test
```

**Against a live local BOINC daemon** (ignored tests — needs GUI RPC password):

```bash
BOINCRS_PASSWORD_FILE=/etc/boinc-client/gui_rpc_auth.cfg \
  cargo test --test live_local_boinc -- --ignored --nocapture
```

**PrimeGrid + Asteroids attach** (needs account keys):

```bash
BOINCRS_PASSWORD_FILE=/etc/boinc-client/gui_rpc_auth.cfg \
BOINCRS_PRIMEGRID_ACCOUNT_KEY='…' \
BOINCRS_ASTEROIDS_ACCOUNT_KEY='…' \
  cargo test --test live_beta_projects -- --ignored --nocapture
```

**Manual smoke:** projects and tasks populate; task groups and sorting look right; selection updates the header; safe actions work; destructive actions prompt `y/n`. See `docs/architecture/smoke-checklist.md`.

## Roadmap

`ROADMAP.md`

## Changelog

`CHANGELOG.md`

## Contributing

`CONTRIBUTING.md`

## Support

`SUPPORT.md`

## License

MIT — see `LICENSE`.

## Security

Do not commit real `.env` values. Treat GUI RPC password and project authenticators as secrets.

## More docs

- `docs/architecture/app-controller.md`
- `docs/architecture/smoke-checklist.md`
- `docs/architecture/beta-primegrid-asteroids.md`
- `docs/decisions/0001-error-handling.md`
