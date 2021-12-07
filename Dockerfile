FROM rust:1.57.0-buster
WORKDIR /app
# https://github.com/rustwasm/wasm-pack/issues/1079
RUN cargo install wasm-pack --version 0.9.1