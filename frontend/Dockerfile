FROM rust:1.67.1

WORKDIR /app

RUN rustup toolchain install stable
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
