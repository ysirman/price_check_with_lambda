
FROM ekidd/rust-musl-builder:1.57.0 AS builder
# build
USER rust
ADD --chown=rust:rust . ./
# lambda.zip作成のために必要
RUN sudo apt-get update && sudo apt-get install zip
RUN cargo build --release


FROM rust:1.56 AS develop-stage
WORKDIR /app
# ホットリロード用
RUN cargo install cargo-watch
