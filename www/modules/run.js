import * as wasm from '../pkg/wasm_tokio.js';           

async function run() {
    await wasm.init();                                            
    await wasm.initThreadPool(navigator.hardwareConcurrency);
    wasm.init_panic_hook();                                  
    wasm.run();
}

run();
