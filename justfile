alias b := build
alias t := test
alias r := run
alias rr := run-release

default:
  @just --list

build:
    cargo build

run:
    cargo run

run-release:
    cargo run --release

test:
    cargo test