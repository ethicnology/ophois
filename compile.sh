#!/bin/bash
# docker + cargo cross + toolchain target needed
targets=("aarch64-unknown-linux-gnu" "i686-pc-windows-gnu" "i686-pc-windows-msvc" "i686-unknown-linux-gnu" "x86_64-apple-darwin" "x86_64-pc-windows-gnu" "x86_64-pc-windows-msvc" "x86_64-unknown-linux-gnu")

for target in ${targets[@]}; do
    eval "cross build --release --target $target " 
done