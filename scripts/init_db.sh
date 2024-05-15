#!/usr/bin/env bash

source .env

cargo sqlx migrate run --source=server/migrations