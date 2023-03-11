import init, * as wasm from "./pkg/wasm_tokio.js";

console.log('Worker: Starting OT setup...');
await init();
wasm.init_panic_hook();
await wasm.initThreadPool(navigator.hardwareConcurrency);
wasm.run();
console.log('Worker: Finished OT setup!');

