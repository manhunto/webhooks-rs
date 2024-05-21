FROM alpine:3.19.1
LABEL authors="kubasladek"

RUN apk update && apk upgrade --no-cache

# Get Ubuntu packages
RUN apk add --no-cache  \
    alpine-sdk \
    curl \
    libressl-dev \
    bash

# Get Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=1.78.0

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install --version=0.7.4 sqlx-cli --no-default-features --features rustls,postgres
RUN cargo install --version=0.1.48 cargo-udeps
RUN cargo install --version=0.30.0 cargo-tarpaulin

RUN rustup default nightly
RUN rustup default stable