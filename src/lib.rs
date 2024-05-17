//! Artificial Intelligence




#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use crate::gui::app::PhoenixGUI;


// exclude this on gui mode
pub mod data_sets;
pub mod matrix;
pub mod neural_network;
// gui stuff

pub mod data;
pub mod gui;

pub const SEED: u64 = 1234;

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn phoenix() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(PhoenixGUI::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
