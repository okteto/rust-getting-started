FROM rust:1.100.0-trixie AS dev

# Cargo compiles into this folder, not into the "target" folder of the working
# directory, so the builds and the tests of the Okteto Manifest use one path
ENV CARGO_TARGET_DIR=/usr/src/target

RUN apt-get update && \
    apt-get install -y --no-install-recommends lldb-19=1:19.1.7-3+b1 && \
    rm -rf /var/lib/apt/lists/*

# The cache mounts keep the Cargo registry and the target folder in the
# Okteto Build Service, which reuses them between builds. The tests of the
# Okteto Manifest use the same two caches
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/target \
    cargo install --locked --root /usr/local watchexec-cli@2.8.0

WORKDIR /usr/src/app
COPY . .
# A cache mount isn't part of the image, so "cp" takes the binary out of it
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/target \
    cargo build && \
    cargo build --release && \
    cp /usr/src/target/release/hello-world /usr/local/bin/hello-world

FROM debian:trixie-slim AS prod

COPY --from=dev /usr/local/bin/hello-world /usr/local/bin/hello-world
CMD ["/usr/local/bin/hello-world"]
