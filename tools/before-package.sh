#!/usr/bin/env bash
# The build `cargo packager` runs before it packages anything — it never
# compiles on its own (see `before-packaging-command` in Cargo.toml).
#
# Two shapes:
#
# - everywhere else, build the release binary the packager is about to pick up;
# - on macOS, when both target directories already exist, that means CI built
#   the arm64 and x86_64 halves and this joins them into one universal binary,
#   so a single dmg runs on either kind of Mac.
set -euo pipefail

if [ "$(uname)" = "Darwin" ] \
    && [ -f target/aarch64-apple-darwin/release/borrow-app ] \
    && [ -f target/x86_64-apple-darwin/release/borrow-app ]; then
    echo "lipo: making a universal borrow-app"
    lipo -create \
        -output target/release/borrow-app \
        target/aarch64-apple-darwin/release/borrow-app \
        target/x86_64-apple-darwin/release/borrow-app
    lipo -info target/release/borrow-app
else
    cargo build --release --bin borrow-app
fi
