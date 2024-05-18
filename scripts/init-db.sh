#!/usr/bin/env bash

cp -n .env.dist .env

source .env

echo "Run migrations";

until cargo sqlx migrate run --source=server/migrations;
do
  echo "Waiting for postgres...";
  sleep 1;
done