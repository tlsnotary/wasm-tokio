#!/bin/bash

# This file is used for hot-reloading during development.
#
# It watches for file changes and if a change is detected it recompiles with
# wasm-pack and hosts it using live-server. Browser is automatically refreshed.
#
# Dependencies:
# Install cargo-watch: cargo install cargo-watch
# Install live-server: npm install -g live-server
#
# Set your browser
browser="firefox"

# This allows to kill both processes with Ctrl+C
(trap 'kill 0' SIGINT; cargo watch -- wasm-pack build --target web \
& live-server --watch="./www/,./pkg/" --entry-file="www/index.html" --browser=$browser --cors)
