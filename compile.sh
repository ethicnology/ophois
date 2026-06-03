#!/bin/bash
# Run inside the .devcontainer (cargo-zigbuild + musl targets preinstalled).
# zig handles cross-linking for every target — no docker/cross needed here.
set -e
targets=("x86_64-unknown-linux-musl" "aarch64-unknown-linux-musl")

for target in "${targets[@]}"; do
    cargo zigbuild --release --target="$target"
    file "target/$target/release/ophois"
done
