FROM okteto/rust:1 as builder
WORKDIR /home/rust

COPY . .  
RUN cargo build --release
RUN cargo install --path .

FROM alpine:3.11 as prod

RUN apk --no-cache add curl ca-certificates
COPY --from=builder /home/rust/main /app/main
CMD /app/main