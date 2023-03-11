import init, * as wasm from "./pkg/wasm_tokio.js";

await init();
wasm.init_panic_hook();
await wasm.initThreadPool(navigator.hardwareConcurrency);
wasm.run();
