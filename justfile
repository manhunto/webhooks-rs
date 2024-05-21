rust-dev-name := "ghcr.io/manhunto/webhooks-rs-dev"
rust-dev-version := "latest"

alias b := build
alias f := format
alias fmt := format
alias c := clippy
alias t := test
alias rs := run-server
alias rd := run-dispatcher
alias rps := run-producer-server
alias rds := run-destination-server
alias du := docker-up
alias dd := docker-down
alias rdb := rust-dev-build
alias rdp := rust-dev-push

default:
    @just --list

build *OPTIONS:
    cargo build --all-targets {{ OPTIONS }}

format:
    cargo fmt --all

# Run main server
run-server *OPTIONS:
    cargo run --package=server {{ OPTIONS }}

# Run consumer that sends messages to destination servers
run-dispatcher *OPTIONS:
    cargo run --package=server --bin=dispatcher {{ OPTIONS }}

# Run example server that produces messages
run-producer-server *OPTIONS:
    cargo run --example producer-server {{ OPTIONS }}

# Run example server that listens for messages and act like real server (with random response delay)
run-destination-server *OPTIONS:
    cargo run --example destination-server {{ OPTIONS }}

# Run cli with args
run-cli *ARGS:
    cargo run --package=cli -- {{ ARGS }}

test:
    cargo test --workspace

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

udeps:
    cargo +nightly udeps --all-targets

coverage:
    cargo +nightly tarpaulin --all-features --workspace --ignore-tests --timeout 120

docker-up *OPTIONS:
    docker compose --env-file=.env up {{ OPTIONS }}

docker-down:
    docker compose down --remove-orphans

check: build clippy test
    cargo sort --workspace
    cargo fmt --check --all

init:
    just docker-up --detach
    ./scripts/init-db.sh

rust-dev-build:
    docker build --platform linux/amd64 . -t {{ rust-dev-name }}:{{ rust-dev-version }} -f .docker/rust/Dockerfile

rust-dev-push:
    docker push ghcr.io/manhunto/webhooks-rs-dev:latest