#!/bin/sh
# Every library and test in the workspace under Miri, with file I/O allowed and strict
# provenance on.
set -e
export MIRIFLAGS="-Zmiri-disable-isolation -Zmiri-strict-provenance"
cargo +nightly-2026-11-14 miri test -q --workspace --tests
