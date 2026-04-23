---
id: 0001-error-handling
title: "ADR 0001: Typed error strategy"
sidebar_position: 1
description: Decision record for using typed errors (thiserror) with AppResult across boincrs.
---

# ADR 0001 — Typed error strategy

## Status

Accepted.

## Context

`boincrs` coordinates multiple fallible boundaries:

- terminal rendering and input
- TCP RPC communication with the BOINC core
- XML-ish response decoding
- action dispatch and confirmation flow

Using unstructured errors makes it harder to surface actionable failures in the
TUI and test expected error behavior.

## Decision

- Use [`thiserror`](https://docs.rs/thiserror) for explicit, typed
  domain/application errors.
- Keep a shared `AppError` for MVP with clear variants — `Io`, `Protocol`,
  `AuthenticationFailed`, `InvalidResponse`, `Ui`.
- Forbid `.unwrap()` / `.expect()` in `src/**` via crate-level linting.
- Allow `.expect()` in tests when it improves test readability.

## Consequences

- Errors are easier to reason about and to map to status messages in the UI.
- API boundaries are cleaner (`AppResult<T>` across modules).
- Slightly more boilerplate than dynamic error containers, but much better
  control and maintainability.

## Downstream impact

- Drives the reconnect/backoff design: `AppError::is_transient()` distinguishes
  recoverable failures from terminal ones (auth, UI).
- Lets the UI render `Retrying` vs `TerminalError` states distinctly — see
  [App controller](../architecture/app-controller.md).
