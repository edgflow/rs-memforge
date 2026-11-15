#!/bin/sh
# MEMFORGE's safety net in one command: the tests, Miri over the whole workspace, and
# AddressSanitizer on the C client.
set -e
cargo test -q --workspace
sh scripts/miri.sh
sh scripts/asan.sh
echo "all checks passed"
