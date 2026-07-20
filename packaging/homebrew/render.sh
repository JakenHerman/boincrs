#!/usr/bin/env bash
#
# Render the Homebrew formula for boincrs to stdout.
#
# The formula installs the prebuilt binaries attached to the GitHub Release for
# a given version. It is regenerated on every tagged release by
# .github/workflows/release.yml and committed to Formula/boincrs.rb, which makes
# this repository usable directly as a Homebrew tap:
#
#     brew tap jakenherman/boincrs https://github.com/jakenherman/boincrs
#     brew install boincrs
#
# Required environment variables:
#   VERSION    release version without the leading "v" (e.g. 1.0.0)
#   SHA_LINUX  sha256 of boincrs-<VERSION>-x86_64-unknown-linux-gnu.tar.gz
#   SHA_ARM    sha256 of boincrs-<VERSION>-aarch64-apple-darwin.tar.gz
#   SHA_INTEL  sha256 of boincrs-<VERSION>-x86_64-apple-darwin.tar.gz
#
# Usage (local dry run):
#   VERSION=1.0.0 SHA_LINUX=... SHA_ARM=... SHA_INTEL=... \
#     bash packaging/homebrew/render.sh > Formula/boincrs.rb

set -euo pipefail

: "${VERSION:?VERSION is required}"
: "${SHA_LINUX:?SHA_LINUX is required}"
: "${SHA_ARM:?SHA_ARM is required}"
: "${SHA_INTEL:?SHA_INTEL is required}"

base="https://github.com/jakenherman/boincrs/releases/download/v${VERSION}"

cat <<EOF
# typed: false
# frozen_string_literal: true

# This file is generated on each tagged release by
# .github/workflows/release.yml (see packaging/homebrew/render.sh).
# Do not hand-edit; changes will be overwritten on the next release.
class Boincrs < Formula
  desc "Fast, keyboard-first terminal UI for a local BOINC client"
  homepage "https://github.com/jakenherman/boincrs"
  version "${VERSION}"
  license "MIT"

  on_macos do
    on_arm do
      url "${base}/boincrs-${VERSION}-aarch64-apple-darwin.tar.gz"
      sha256 "${SHA_ARM}"
    end
    on_intel do
      url "${base}/boincrs-${VERSION}-x86_64-apple-darwin.tar.gz"
      sha256 "${SHA_INTEL}"
    end
  end

  on_linux do
    on_intel do
      url "${base}/boincrs-${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "${SHA_LINUX}"
    end
  end

  def install
    bin.install "boincrs"
  end

  test do
    assert_match "boincrs #{version}", shell_output("#{bin}/boincrs --version")
  end
end
EOF
