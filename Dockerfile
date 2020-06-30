FROM okteto/rust:1 as builder
WORKDIR /home/rust

COPY Cargo.toml .
COPY Cargo.lock .
COPY . . 
RUN cargo build --release

FROM rust:1 as prod
WORKDIR /app

COPY --from=builder /home/rust/target/release/hello-world /app/hello-world
CMD /app/hello-world