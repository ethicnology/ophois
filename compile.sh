#!/bin/bash
# docker + cargo cross + toolchain target needed
targets=("i686-unknown-linux-musl" "x86_64-unknown-linux-musl" "aarch64-unknown-linux-musl")

for target in ${targets[@]}; do
    eval "cross build --release --target=$target " 
done