//! Artificial Intelligence


use std::cmp::Ordering;

use std::sync::Mutex;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;


use lazy_static::lazy_static;


use crate::matrix::Matrix;

use crate::neural_network::NeuralNetworkWorker;

// exclude this on gui mode
pub mod data_sets;
pub mod matrix;
pub mod neural_network;
// gui stuff

pub mod gui;
mod data;

pub const SEED: u64 = 1234;

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn phoenix() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();
    println!("Starting...");

    let web_options = eframe::WebOptions::default();
    use crate::gui::app::PhoenixGUI;

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(PhoenixGUI::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}