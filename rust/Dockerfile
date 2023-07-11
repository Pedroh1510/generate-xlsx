# FROM debian:buster-slim as builder

# RUN apt update && apt upgrade -y
# RUN apt install -y curl build-essential gcc make wget
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# ENV PATH="/root/.cargo/bin:${PATH}"
FROM rust:latest AS builder
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev curl build-essential gcc make cmake libssl-dev
run apt install -y llvm 
run apt install -y clang
WORKDIR /usr/src/myapp
# RUN rustc -V

COPY . .
# RUN cargo build
# RUN cargo install --path .
RUN cargo build --release
CMD ["./target/release/rust"]
# CMD ["cargo run"]
# FROM rust:latest
# COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
# CMD ["myapp"]