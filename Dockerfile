# Build

FROM rustlang/rust:nightly-alpine3.12 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release --target=x86_64-unknown-linux-musl

COPY . .

# Run

FROM rustlang/rust:nightly-alpine3.12

RUN addgroup -g 1000 demo_rust_rocket

RUN adduser -D -s /bin/sh -u 1000 -G demo_rust_rocket demo_rust_rocket

WORKDIR /home/demo_rust_rocket/bin/

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/demo_rust_rocket .

RUN chown demo_rust_rocket:demo_rust_rocket demo_rust_rocket

USER demo_rust_rocket

CMD ["./rust-site"]
