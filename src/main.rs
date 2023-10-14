// native app entry_point



use async_std::task::block_on;
use sketch::{run_app};

mod sketch;
mod carbon;
mod sketch_model;

fn main() {

    let model = sketch_model::Model::new();

    block_on(async {
        run_app(model).await;
    });
}
