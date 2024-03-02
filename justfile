alias b := build
alias t := test
alias r := run

default:
  @just --list

build:
    cargo build

run:
    cargo run

test:
    cargo test