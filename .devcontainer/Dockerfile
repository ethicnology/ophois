FROM rust:1.96-trixie

# zig provides the cross-linker for every musl target (no per-arch gcc toolchain)
RUN apt-get update \
    && apt-get install -y --no-install-recommends python3-pip \
    && rm -rf /var/lib/apt/lists/* \
    && pip3 install --break-system-packages ziglang

RUN cargo install cargo-zigbuild \
    && rustup target add \
        x86_64-unknown-linux-musl \
        aarch64-unknown-linux-musl
