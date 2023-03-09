#!/bin/bash

# Watch for changes and recompile and reload the browser
(trap 'kill 0' SIGINT; cargo watch -- wasm-pack build --target web & live-server --watch="./index.*,./pkg" --entry-file="src/index.html" --browser=firefox)
