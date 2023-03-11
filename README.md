# Wasm-Tokio Example

This is a test project to run KOS OT `rand_setup` in the browser using WASM, making use of a
single-threaded tokio runtime and multiple rayon threads.

## How to run it

### Requirements
1. Install google-chrome browser.
2. Install [simple-http-server](https://github.com/TheWaWaR/simple-http-server)
3. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/)

### Run
```
# Build wasm modules and js wrapper
wasm-pack build -t web -d www/pkg --release

# Serve files
simple-http-server --ip 127.0.0.1 --nocache -t1 -i --coop --coep www
```

Now open Chrome at `localhost:8000`. You should see some messages in the
console.




