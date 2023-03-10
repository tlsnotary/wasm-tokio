use wasm_bindgen::prelude::*;

mod kos;

#[wasm_bindgen]
pub fn init_panic_hook() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn run() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(setup_kos());
}

async fn setup_kos() {
    const OT_COUNT: usize = 10_000;

    let (sender, receiver) = kos::init_kos();
    let (sender, receiver) =
        tokio::try_join!(sender.rand_setup(OT_COUNT), receiver.rand_setup(OT_COUNT)).unwrap();
}
