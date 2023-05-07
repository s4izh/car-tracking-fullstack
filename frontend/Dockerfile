FROM rust:1.67.1

# Instalar Node.js y npm
RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get update && apt-get install -y nodejs

# Instalar npx
RUN npm install -g npm@9.6.5
RUN npm install -g postcss-cli@8.3.1 autoprefixer@10.4.0 tailwindcss@3.1.4
#RUN npm install -g npx


WORKDIR /app

RUN rustup toolchain install stable
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
