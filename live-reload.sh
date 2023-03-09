#!/bin/bash

# Watch for changes and recompile and reload the browser
(trap 'kill 0' SIGINT; cargo watch -- wasm-pack build --target web & live-server --watch="pkg/wasm-tokio/src/index.html" --entry-file="src/index.html" --browser=firefox)
