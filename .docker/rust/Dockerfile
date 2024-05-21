FROM rust:1.78-alpine3.19 as builder

RUN apk add --no-cache \
    alpine-sdk \
    libressl-dev \
    bash

RUN cargo install --version=0.7.4 sqlx-cli --no-default-features --features rustls,postgres
RUN cargo install --version=0.1.48 cargo-udeps
RUN cargo install --version=0.30.0 cargo-tarpaulin

FROM rust:1.78-alpine3.19

RUN apk add --no-cache \
    alpine-sdk \
    libressl-dev \
    bash \
    ca-certificates

RUN rm -rf /var/cache/apk/*

COPY --from=builder /usr/local/cargo/bin/cargo-tarpaulin /usr/local/cargo/bin/cargo-tarpaulin
COPY --from=builder /usr/local/cargo/bin/cargo-udeps /usr/local/cargo/bin/cargo-udeps
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/cargo/bin/sqlx

RUN rustup component add clippy

COPY .docker/cacert.pem /usr/local/share/ca-certificates/cacert.crt

RUN update-ca-certificates