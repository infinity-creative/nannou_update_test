use async_std::task::block_on;
use wasm_bindgen::prelude::wasm_bindgen;

use sketch::{run_app};

mod sketch;
mod carbon;
mod sketch_model;

// web app entry_point
#[wasm_bindgen]
pub async fn main_web() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let model = sketch_model::Model::new();

    block_on(async {
        run_app(model).await;
    });
}
