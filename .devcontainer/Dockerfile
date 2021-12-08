FROM rust:1.57.0
WORKDIR /app
# https://github.com/rustwasm/wasm-pack/issues/1079
RUN cargo install wasm-pack --version 0.9.1
RUN rustup component add rls rust-analysis rust-src rustfmt
RUN apt update
RUN apt install -y nodejs npm &&\
    npm install -g n &&\
    n stable &&\
    apt purge -y nodejs npm
RUN apt install -y default-jre 