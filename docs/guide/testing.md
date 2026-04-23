---
id: testing
title: Testing & verification
sidebar_position: 10
description: How to run boincrs tests locally, including the live-daemon and PrimeGrid/Asteroids integration suites.
---

# Testing & verification

## Default test suite

```bash
cargo test
```

Runs unit, integration, parser, and compatibility-fixture tests. CI also runs
`cargo test --doc --locked` for doctests.

## Format and lint

```bash
cargo fmt --check
cargo clippy -- -D warnings
```

CI treats clippy warnings as errors.

## Compatibility fixtures

Run all three supported BOINC branches in one pass:

```bash
cargo test --test compatibility_matrix_tests
```

Individual branch fixtures:

```bash
cargo test --test compatibility_matrix_tests boinc_7_16_fixture_compatibility
cargo test --test compatibility_matrix_tests boinc_7_20_fixture_compatibility
cargo test --test compatibility_matrix_tests boinc_8_2_fixture_compatibility
```

## Live local BOINC daemon (ignored test)

Requires a running local BOINC client and access to its GUI RPC password:

```bash
BOINCRS_PASSWORD_FILE=/etc/boinc-client/gui_rpc_auth.cfg \
  cargo test --test live_local_boinc -- --ignored --nocapture
```

## PrimeGrid + Asteroids@home beta attach (ignored test)

Requires both account keys plus a running BOINC daemon:

```bash
BOINCRS_PASSWORD_FILE=/etc/boinc-client/gui_rpc_auth.cfg \
BOINCRS_PRIMEGRID_ACCOUNT_KEY='…' \
BOINCRS_ASTEROIDS_ACCOUNT_KEY='…' \
  cargo test --test live_beta_projects -- --ignored --nocapture
```

Details:
[PrimeGrid + Asteroids beta architecture](./architecture/beta-primegrid-asteroids.md).

## Manual smoke

Run `cargo run` and complete the
[local smoke checklist](./architecture/smoke-checklist.md). Record the host
OS, BOINC version, and outcome when you do this before a release.

## When to re-run what

| Change                                           | Run                                                    |
| ------------------------------------------------ | ------------------------------------------------------ |
| Anything in `src/ui/**`                          | `cargo test`, plus a visual smoke                      |
| Anything in `src/boinc/**`                       | `cargo test --test compatibility_matrix_tests`         |
| Auth, transport, controller refresh              | Compatibility fixtures **+** live local daemon test    |
| New BOINC field / parser change                  | Add a fixture, update the compatibility matrix         |
| Before a release                                 | Full [release checklist](./release-checklist.md)        |
