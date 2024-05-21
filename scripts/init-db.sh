#!/usr/bin/env bash

cp -n .env.dist .env

source .env

echo "Run migrations";

RETRIES=30
until sqlx migrate run --source=server/migrations || [ $RETRIES -eq 0 ];
do
  echo "Waiting for postgres server, $((RETRIES--)) remaining attempts..."
  sleep 1;
done