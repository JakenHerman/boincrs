---
id: roadmap
title: Roadmap
sidebar_position: 12
description: What boincrs covers today, what's next, and what's staged for 1.0.
---

# Roadmap

## Beta (current)

- Local BOINC RPC connectivity and authentication
- PrimeGrid + Asteroids@home attach flow via account keys
- Projects / Tasks / Transfers panes
- Selected-task details header
- Task grouping and ordering:
  1. ready-to-report
  2. running (by completion)
  3. waiting / ready

## Next

- ~~Better task metadata coverage (more BOINC result fields)~~ ✓
- ~~Better transfer visibility (retry / error reasons and throughput)~~ ✓
- ~~Improved project / task selection for all panes~~ ✓
- ~~Exportable diagnostics bundle for bug reports~~ ✓
- ~~Optional `.env` auto-loading without shell `source`~~ ✓
- ~~BOINC compatibility matrix and validation gates~~ ✓

## Release candidate

- **Packaging**:
  - Linux package artifacts
  - macOS signed / universal binaries
  - Windows executable + installer path
- Backoff / reconnect hardening and stronger error surfacing.
- User-facing command docs and screenshots on the docs site.

## 1.0

- Stable release process and changelog policy.
- Broader project attach templates and preset profiles.
- Optional remote RPC profile support (behind a feature gate).
- Improved accessibility and terminal theme behavior.

See [Contributing](./contributing.md) if you'd like to pick up any of the
above.
