#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// hide console window on Windows in release

use phoenix_gui::gui::app::PhoenixGUI;

const ICON: &[u8; 13450] = include_bytes!("../resources/phoenix_icon.png");

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    println!("Starting...");
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        resizable: true,
        centered: true,
        // maximized: true,
        decorated: true,
        // not exactly hd, because i dont want it to clip into something
        initial_window_size: Some(egui::Vec2 {
            x: 1900.0,
            y: 1000.0,
        }),
        // min_window_size: Some(egui::Vec2 {
        //     x: 1900.0,
        //     y: 1000.0,
        // }),
        icon_data: Some(load_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "Phoenix",
        options,
        Box::new(|cc| Box::new(PhoenixGUI::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    println!("Phoenix is not compiled with GUI support. Try to compile with --all-features.");
}

#[cfg(not(target_arch = "wasm32"))]
fn load_icon() -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        // convert the ICON bytes to an image
        let image = image::load_from_memory(ICON)
            .expect("Failed to load icon bytes")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
