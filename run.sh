#!/bin/bash

PRIVATE_KEY="lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua" \
PG_DB_NAME=sankar \
PG_DB_UNAME=sankar \
PG_DB_PWD=sankar \
DEV=TRUE \
REDIS_URI="127.0.0.1:6379" \
DB_URI="127.0.0.1:9042" \
INDEXER_URI="http://localhost:7700" \
LP_HOST=localhost \
LP_PORT=2023 \
RUST_LOG=info ./target/release/lily

