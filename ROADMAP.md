# boincrs Roadmap

## Beta (current)

- Local BOINC RPC connectivity and authentication
- PrimeGrid + Asteroids@home attach flow via account keys
- Projects / Tasks / Transfers panes
- Selected task details header
- Task grouping and ordering:
  - ready-to-report
  - running (by completion)
  - waiting / ready

## Next

- ~~Better task metadata coverage (more BOINC result fields)~~ ✓
- ~~Better transfer visibility (retry/error reasons and throughput)~~ ✓
- ~~Improved project/task selection for all panes~~ ✓
- ~~Exportable diagnostics bundle for bug reports~~ ✓
- ~~Optional `.env` auto-loading without shell `source`~~ ✓

## Release Candidate

- Packaging:
  - Linux package artifacts
  - macOS signed/universal binaries
  - Windows executable + installer path
- Backoff/reconnect and stronger error surfacing
- Wider BOINC version compatibility validation
- User-facing command docs and screenshots

## 1.0

- Stable release process and changelog policy
- Broader project attach templates and preset profiles
- Optional remote RPC profile support
- Improved accessibility and terminal theme behavior
