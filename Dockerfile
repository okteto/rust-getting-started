FROM rust:1.100.0-bookworm AS dev

RUN apt-get update && \
    apt-get install -y --no-install-recommends lldb-14=1:14.0.6-12 && \
    rm -rf /var/lib/apt/lists/* && \
    cargo install --locked --root /usr/local watchexec-cli@2.8.0

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY . .
RUN cargo build && \
    cargo build --release && \
    cp target/release/hello-world /usr/local/bin/hello-world

FROM debian:bookworm-slim AS prod

COPY --from=dev /usr/local/bin/hello-world /usr/local/bin/hello-world
CMD ["/usr/local/bin/hello-world"]
