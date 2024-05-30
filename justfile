rust-dev-name := "ghcr.io/manhunto/webhooks-rs-dev"
rust-dev-version := "latest"
rust-dev-image := rust-dev-name + ":" + rust-dev-version

rabbitmq-dev-name := "ghcr.io/manhunto/webhooks-rs-rabbitmq"
rabbitmq-dev-version := "latest"
rabbitmq-dev-image := rabbitmq-dev-name + ":" + rabbitmq-dev-version

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
    cargo build --all-targets --workspace {{ OPTIONS }}

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

clippy-pedantic:
    cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic

udeps:
    cargo +nightly udeps --all-targets

coverage:
    cargo +nightly-2024-05-18 tarpaulin --all-features --workspace --ignore-tests --timeout 120

docker-up *OPTIONS:
    docker compose --env-file=.env up {{ OPTIONS }}

docker-down:
    docker compose down --remove-orphans

check:
    just build
    cargo fmt --check --all
    just clippy
    just test
    cargo sort --workspace

init:
    just docker-up --detach
    ./scripts/init-db.sh

rust-dev-build:
    docker build --platform linux/amd64 . -t {{ rust-dev-image }} -f .docker/rust/Dockerfile

rust-dev-push:
    docker push {{ rust-dev-image }}

rabbitmq-dev-build:
    docker build --platform linux/amd64 . -t {{ rabbitmq-dev-image }} -f .docker/rabbitmq/Dockerfile

rabbitmq-dev-push:
    docker push {{ rabbitmq-dev-image }}

create-migration NAME:
    sqlx migrate add --source=server/migrations "{{ NAME }}"