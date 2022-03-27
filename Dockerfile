FROM rust:1.56 as develop-stage

WORKDIR /app
# ホットリロード用
RUN cargo install cargo-watch
