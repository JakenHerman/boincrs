---
id: compatibility
title: BOINC compatibility matrix
sidebar_position: 9
description: Which BOINC client branches boincrs actively validates, how, and with what host OS coverage.
---

# BOINC compatibility matrix

This page defines the BOINC client branches `boincrs` actively validates.
The matrix is about **GUI RPC compatibility for local clients**, not every
historical BOINC installer that has ever existed upstream.

## Supported targets

| BOINC target | Typical host OS combinations | Validation status | Known notes |
| --- | --- | --- | --- |
| `7.16.x` | Legacy macOS `10.9`–`12.x` installs and older Windows/macOS hosts still pinned to `7.16` | Supported with fixture coverage and manual legacy-host smoke when BOINC-facing code changes | Older clients may require CA bundle updates to reach some projects; replies commonly use tags like `active_task_state`, `last_bytes_xferred`, and `error`. |
| `7.20.x` | macOS `10.10`–`12.x` transition hosts that cannot move to `8.x` yet | Supported with fixture coverage and manual smoke on a matching macOS host when BOINC-facing code changes | Some replies expose app identity via `plan_class` and mode values under nested `<mode>` tags. |
| `8.2.x`  | Linux `x64` / `ARM64`, macOS `10.13+`, Windows `10/11` `x64` | Supported current branch with fixture coverage plus automated host-OS CI on Linux, macOS, and Windows | Upstream's current mainstream branch; `boincrs` validates GUI RPC behavior, not optional Docker/host packaging features. |

## Automated validation

Each supported BOINC branch has a dedicated fixture-backed compatibility test:

```bash
cargo test --test compatibility_matrix_tests boinc_7_16_fixture_compatibility
cargo test --test compatibility_matrix_tests boinc_7_20_fixture_compatibility
cargo test --test compatibility_matrix_tests boinc_8_2_fixture_compatibility
```

Fixtures live under `tests/fixtures/compatibility/` and deliberately cover the
parser fallbacks that vary across BOINC releases, including alternate
task-state, transfer, application, and mode encodings.

GitHub Actions also runs a host-OS build/test matrix on:

- `ubuntu-latest`
- `macos-latest`
- `windows-latest`

That host matrix validates that `boincrs` itself still builds and passes tests
on the common desktop environments used with local BOINC clients.

## Manual validation

Automation does not replace a live daemon check. Before a release:

1. Install or launch the target BOINC client version on the intended host OS.
2. Run the ignored live read-surface test:
   ```bash
   BOINCRS_PASSWORD_FILE=/path/to/gui_rpc_auth.cfg \
     cargo test --test live_local_boinc -- --ignored --nocapture
   ```
3. Run `cargo run` and complete the
   [smoke checklist](./architecture/smoke-checklist.md).
4. Record the host OS, BOINC version, and outcome in the release sign-off notes.

If the change touched BOINC protocol parsing, auth, transport, or controller
refresh logic, repeat the live check on one legacy branch target (`7.16.x` or
`7.20.x`) in addition to the current `8.2.x` branch.

## Known limitations and quirks

- `boincrs` currently supports **local** GUI RPC workflows. Remote profile
  management is outside this compatibility matrix.
- BOINC versions before `7.18` can need upstream certificate bundle fixes to
  contact some projects; `boincrs` does not patch that at runtime.
- GUI RPC password file locations vary by packaging and OS. Common examples
  are `/etc/boinc-client/gui_rpc_auth.cfg` on Linux and BOINC-managed app data
  paths on macOS or Windows installs.
- New BOINC fields may appear without breaking `boincrs`, but missing or
  renamed fields still need fixture updates before a new BOINC branch is
  promoted to "supported".

## Maintenance

When BOINC-facing behavior changes:

1. Update or add fixtures under `tests/fixtures/compatibility/`.
2. Keep this matrix in sync with the targets covered by CI.
3. Update the [release checklist](./release-checklist.md) and this page if
   support policy changes.
