# Release Checklist

Use this checklist before publishing a `boincrs` release candidate or tagged release.

## Compatibility Sign-Off Gate

Do not ship a release until all of the following are true:

- [ ] `ci`, `host-matrix`, and `compatibility-matrix` GitHub Actions jobs are green.
- [ ] `docs/compatibility-matrix.md` still matches the BOINC branches we intend to support.
- [ ] A live `8.2.x` BOINC daemon smoke has passed on at least one supported host OS.
- [ ] If BOINC-facing code changed, a live legacy smoke (`7.16.x` or `7.20.x`) also passed.
- [ ] `docs/architecture/smoke-checklist.md` was completed against the release candidate.
- [ ] README and changelog entries mention any newly discovered compatibility limits or quirks.

## Manual Sign-Off Record

Capture the release sign-off in the PR description, release issue, or tag notes using a
short record like this:

```text
Compatibility sign-off:
- BOINC 8.2.x on <host OS>: PASS/FAIL
- BOINC 7.20.x or 7.16.x on <host OS>: PASS/FAIL (required for BOINC-facing changes)
- Smoke checklist: PASS/FAIL
- Compatibility CI fixtures: PASS/FAIL
```

## Commands

Current-branch live validation:

```bash
BOINCRS_PASSWORD_FILE=/path/to/gui_rpc_auth.cfg \
  cargo test --test live_local_boinc -- --ignored --nocapture
```

Fixture compatibility checks:

```bash
cargo test --test compatibility_matrix_tests
```
