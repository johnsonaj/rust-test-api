FROM rust:latest as builder

RUN apt-get update

RUN apt-get install musl-tools -y
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/rust-test-api

COPY Cargo.toml Cargo.toml
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
# this build step will cache your dependencies
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN rm src/*.rs
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/rust-test-api*

COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest

RUN addgroup -g 1000 rust-test-api
RUN adduser -D -s /bin/sh -u 1000 -G rust-test-api rust-test-api
WORKDIR /home/rust-test-api/bin/
COPY --from=builder /usr/src/rust-test-api/target/x86_64-unknown-linux-musl/release/rust-test-api .

RUN chown rust-test-api:rust-test-api rust-test-api
USER rust-test-api
CMD ["./rust-test-api"]