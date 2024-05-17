#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// hide console window on Windows in release

use egui::ViewportBuilder;
use phoenix_gui::gui::app::PhoenixGUI;
use egui::IconData;

const ICON: &[u8; 13450] = include_bytes!("../resources/phoenix_icon.png");

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    println!("Starting...");
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        // resizable: true,
        centered: true,
        // maximized: true,
        // decorated: true,
        // not exactly hd, because i dont want it to clip into something
        // initial_window_size: Some(egui::Vec2 {
        //     x: 1900.0,
        //     y: 1000.0,
        // }),
        // min_window_size: Some(egui::Vec2 {
        //     x: 1900.0,
        //     y: 1000.0,
        // }),
        // icon_data: Some(load_icon()),
        viewport: ViewportBuilder::default().with_title("Phoenix")
            .with_icon(load_icon())
            .with_resizable(true)
            .with_decorations(true)
            .with_inner_size(egui::Vec2 {
                x: 1900.0,
                y: 1000.0,
            }),
        ..Default::default()
    };
    eframe::run_native(
        "Phoenix",
        options,
        Box::new(|cc| Box::new(PhoenixGUI::new(cc))),
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        // convert the ICON bytes to an image
        let image = image::load_from_memory(ICON)
            .expect("Failed to load icon bytes")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}


// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
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