#!/bin/sh

sudo systemctl start scylla-server

sleep 60

./target/release/lily

