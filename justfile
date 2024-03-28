alias b := build
alias c := clippy
alias t := test
alias rs := run-server
alias rd := run-dispatcher
alias rps := run-producer-server
alias rds := run-destination-server

default:
    @just --list

build:
    cargo build

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

test:
    cargo test

clippy:
    cargo clippy

docker-start:
    docker compose up

docker-stop:
    docker compose down --remove-orphans

check: build clippy test
    cargo sort --workspace
