#!/usr/bin/env bash
#
# Fails the build when a user-visible code / docs change lands without a
# matching CHANGELOG.md entry under the `## [Unreleased]` section.
#
# Escape hatches (for pure refactors, renames, CI-only tweaks, etc.):
#   1. Add the `skip-changelog` label to the PR.
#   2. Or include the literal token `[skip-changelog]` anywhere in the PR body.
#
# Required env vars:
#   BASE_SHA    commit SHA to diff against (usually the PR base)
#   HEAD_SHA    commit SHA to diff to      (usually the PR head)
#   PR_LABELS   JSON array of label names (may be empty)
#   PR_BODY     PR description text       (may be empty)
#
# Designed to run in a standard GitHub Actions runner with
# `actions/checkout@v4` and `fetch-depth: 0`.

set -euo pipefail

: "${BASE_SHA:?BASE_SHA is required}"
: "${HEAD_SHA:?HEAD_SHA is required}"
PR_LABELS="${PR_LABELS:-[]}"
PR_BODY="${PR_BODY:-}"

log() { printf '%s\n' "$*"; }

# --- Escape hatches -----------------------------------------------------------
if printf '%s' "$PR_LABELS" | grep -Eqi '"skip-changelog"'; then
  log "skip-changelog label present; bypassing changelog enforcement."
  exit 0
fi

if printf '%s' "$PR_BODY" | grep -Fq '[skip-changelog]'; then
  log "[skip-changelog] marker found in PR body; bypassing changelog enforcement."
  exit 0
fi

# --- Did this PR touch anything user-visible? ---------------------------------
# These paths represent change surfaces whose behavior a user of boincrs can
# observe. Anything else (tests-only refactors, CI fiddling, internal docs
# comments) doesn't require a changelog entry on its own.
CHANGED="$(git diff --name-only "$BASE_SHA" "$HEAD_SHA")"

log "Changed files:"
printf '  %s\n' $CHANGED || true

# Extended regex matching paths that require a changelog entry.
USER_VISIBLE_RE='^(src/|\.env\.example$|docs/guide/keyboard\.md$|docs/guide/configuration\.md$|docs/guide/usage\.md$|docs/guide/accessibility\.md$|docs/guide/compatibility\.md$|docs/guide/getting-started\.md$|docs/guide/installation\.md$|docs/guide/why-boincrs\.md$|docs/guide/intro\.md$|docs/guide/release-checklist\.md$|docs/guide/architecture/)'

if ! echo "$CHANGED" | grep -Eq "$USER_VISIBLE_RE"; then
  log "No user-visible code/docs paths were modified; changelog entry not required."
  exit 0
fi

log "User-visible change detected — a CHANGELOG.md entry is required."

# --- Is CHANGELOG.md in the diff at all? --------------------------------------
if ! echo "$CHANGED" | grep -qx 'CHANGELOG.md'; then
  cat >&2 <<'EOF'
::error file=CHANGELOG.md,title=Missing changelog entry::User-visible change detected, but CHANGELOG.md was not modified.

Please add a bullet under the `## [Unreleased]` section of CHANGELOG.md
describing the user-visible change.

If this change truly has no user-visible impact (e.g. pure refactor, test
cleanup, CI tweak), either:
  - add the `skip-changelog` label to the PR, or
  - include the literal token [skip-changelog] somewhere in the PR body.
EOF
  exit 1
fi

# --- Does the CHANGELOG.md diff actually add lines? ---------------------------
# `+` lines that are not the `+++ b/CHANGELOG.md` header count as additions.
ADDED=$(
  git diff --unified=0 "$BASE_SHA" "$HEAD_SHA" -- CHANGELOG.md \
    | grep -E '^\+' \
    | grep -Ev '^\+\+\+ ' \
    | wc -l \
    | tr -d ' '
)

if [ "${ADDED:-0}" -eq 0 ]; then
  cat >&2 <<'EOF'
::error file=CHANGELOG.md,title=Empty changelog diff::CHANGELOG.md was touched but no lines were added.

Add a concrete bullet under `## [Unreleased]`. Comment or formatting tweaks
alone are not enough — reviewers expect to see the behavioral change called
out.
EOF
  exit 1
fi

# --- Are the additions actually inside the Unreleased section? ----------------
# We use a simple awk sweep over the head-side CHANGELOG: find the line range
# under `## [Unreleased]` and make sure at least one added line (from the PR
# diff) falls inside it.
#
# This keeps people from accidentally adding notes under an older release
# header.
HEAD_CHANGELOG_TMP="$(mktemp)"
PATCH_TMP="$(mktemp)"
trap 'rm -f "$HEAD_CHANGELOG_TMP" "$PATCH_TMP"' EXIT

git show "$HEAD_SHA:CHANGELOG.md" > "$HEAD_CHANGELOG_TMP"
git diff --unified=0 "$BASE_SHA" "$HEAD_SHA" -- CHANGELOG.md > "$PATCH_TMP"

UNRELEASED_OK=$(
  awk '
    BEGIN { unreleased_start = 0; unreleased_end = 0; in_unreleased = 0; added_in_range = 0 }
    FNR == NR {
      # First file: determine Unreleased line range on the HEAD side.
      if ($0 ~ /^##[[:space:]]+\[Unreleased\]/) {
        unreleased_start = FNR + 1
        in_unreleased = 1
        next
      }
      if (in_unreleased && $0 ~ /^##[[:space:]]+\[/) {
        unreleased_end = FNR - 1
        in_unreleased = 0
      }
      if (in_unreleased) { unreleased_end = FNR }
      next
    }
    # Second file: walk the patch and decide whether any + line sits inside
    # the unreleased range (on the new/head side).
    /^@@/ {
      # hunk header: @@ -a,b +c,d @@
      if (match($0, /\+([0-9]+)(,([0-9]+))?/)) {
        start = substr($0, RSTART + 1, RLENGTH - 1)
        n = split(start, parts, ",")
        new_line = parts[1] + 0
      }
      next
    }
    /^\+[^+]/ {
      if (unreleased_start > 0 &&
          new_line >= unreleased_start &&
          new_line <= unreleased_end) {
        added_in_range = 1
      }
      new_line++
      next
    }
    /^-/ { next }
    /^ / { new_line++; next }
    END { print added_in_range }
  ' "$HEAD_CHANGELOG_TMP" "$PATCH_TMP"
)

if [ "$UNRELEASED_OK" != "1" ]; then
  cat >&2 <<'EOF'
::error file=CHANGELOG.md,title=Entry not under [Unreleased]::CHANGELOG.md was changed, but no new line was added inside the `## [Unreleased]` section.

Please add your bullet directly under `## [Unreleased]` so the next release
notes pick it up automatically.
EOF
  exit 1
fi

log "CHANGELOG.md has a new entry under [Unreleased]. OK."
