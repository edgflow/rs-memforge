#!/bin/sh
# Builds the C door with AddressSanitizer, links the C client against it and against
# rustc's own ASan runtime, and runs the client. Apple's clang has an older ASan, so the C
# file is compiled plainly: the runtime still guards every malloc, and checks the Rust side.
set -e
export RUSTFLAGS="-Zsanitizer=address"
cargo +nightly-2026-11-14 build -q -p memforge-ffi --target aarch64-apple-darwin
RT="$(rustc +nightly-2026-11-14 --print sysroot)/lib/rustlib/aarch64-apple-darwin/lib"
clang -g examples/c_client.c -I crates/memforge-ffi/include \
    -L target/aarch64-apple-darwin/debug -lmemforge_ffi \
    -L "$RT" -lrustc-nightly_rt.asan -Wl,-rpath,"$RT" -o target/c_client_asan
export ASAN_OPTIONS="strip_path_prefix=$(pwd)/"
./target/c_client_asan "$@"
