---
id: project-templates-and-profiles
title: Project templates & profiles
sidebar_position: 4
description: Curated BOINC project registry plus a preset profile format for repeatable onboarding.
---

# Project attach templates and preset profiles

`boincrs` ships with two cooperating onboarding primitives:

1. **Project templates** — a curated registry of well-known BOINC projects
   mapping a short **slug** (e.g. `primegrid`) to a canonical master URL and
   metadata.
2. **Preset profiles** — a small persisted document that bundles attach
   requests and run/network/GPU mode overrides, so returning users can
   re-apply a known-good setup in one place.

Both live in `src/boinc/`:

| File | Purpose |
| --- | --- |
| `src/boinc/templates.rs` | In-memory registry, slug lookup, URL validation |
| `src/boinc/profiles.rs` | Parse / format / load / save preset profiles |
| `src/boinc/bootstrap.rs` | Applies env vars + profile to a live RPC client |

## Template registry

`boincrs::boinc::templates::all_templates()` returns the curated list. Each
entry has a `slug`, human `name`, canonical `url`, short `summary`, and a set
of `categories` (e.g. `astronomy`, `biology`).

Current slugs: `asteroids`, `einstein`, `gpugrid`, `lhc`, `milkyway`,
`primegrid`, `rosetta`, `seti`, `worldcommunitygrid`, `yoyo`.

Resolution is performed by `resolve_template(input)` which accepts either a
slug **or** a fully qualified URL. Invalid inputs produce a `TemplateError`
with a human-readable hint (including the full list of known slugs when a
slug cannot be resolved).

## Preset profile format

Profiles are a tiny `key = value` text format — no new dependencies.

```
# boincrs profile: desktop
name = desktop
run_mode = auto
network_mode = auto
gpu_mode = never
attach = primegrid|ACCOUNT_KEY_A
attach = rosetta|ACCOUNT_KEY_B
attach = https://boinc.example.org/custom/|ACCOUNT_KEY_C
```

Recognized keys:

| Key | Required | Notes |
| --- | --- | --- |
| `name` | yes | ASCII letters, digits, `-`, `_` |
| `run_mode` | no | `always` \| `auto` \| `never` |
| `network_mode` | no | `always` \| `auto` \| `never` |
| `gpu_mode` | no | `always` \| `auto` \| `never` |
| `attach` | repeatable | `slug_or_url|account_key` |

Every validation error carries the offending line number and key, so users can
fix typos without trial-and-error. Unknown keys are rejected rather than
silently ignored, so a misspelled `runmode =` won't quietly disable itself.

## Startup flow

When `boincrs` starts, `bootstrap::attach_projects_from_env` reads the
following environment variables in order:

1. `BOINCRS_PROFILE_FILE` — path to a preset profile. All attach entries and
   mode overrides in the profile are applied.
2. `BOINCRS_PRIMEGRID_ACCOUNT_KEY` / `BOINCRS_ASTEROIDS_ACCOUNT_KEY` —
   convenience shortcuts for the two projects shipped since the 0.1 beta.
3. `BOINCRS_ATTACH_TEMPLATES` — semicolon-delimited `slug|key` pairs
   (e.g. `primegrid|KEY1;rosetta|KEY2`).
4. `BOINCRS_ATTACH_PROJECTS` — legacy semicolon-delimited `url|key` pairs.

The resulting list is deduplicated by URL (first-write-wins), then each entry
is attached via `project_attach` followed by `project_update`. Mode overrides
from the profile are applied after attach completes.

Parsing failures from env vars are **not fatal**: malformed entries are
recorded in the returned `BootstrapReport.skipped` list and reported in the
TUI status line, while the remainder of the bootstrap continues. Profile-load
failures (e.g. unknown key, invalid mode value) **are** fatal — a bad profile
is a clear config bug the user needs to correct.

## Testing

- `src/boinc/templates.rs` — unit tests for slug lookup, URL validation,
  error messaging.
- `src/boinc/profiles.rs` — unit tests for parser happy path, every error
  variant, and round-trip serialization.
- `tests/bootstrap_templates_tests.rs` — integration tests covering template
  resolution, profile round-trip, comment/whitespace tolerance, and a mock
  RPC attach call.
