import init, * as wasm from "./pkg/wasm_tokio.js";

await init();
wasm.init_panic_hook();
await wasm.initThreadPool(2);
console.log('Worker: Starting OT setup...');
wasm.run();
console.log('Worker: Finished OT setup!');

