use egui::ScrollArea;
use egui_dock::NodeIndex;

use crate::gui::tab_types::PlotType;
use egui::Context;

use crate::gui::file_viewer::FileViewer;
use crate::gui::tab_types::image::ImageTab;
use crate::gui::tab_types::plot_file::PlotFile;
#[cfg(not(target_arch = "wasm32"))]
use sysinfo::{System, SystemExt};

use crate::gui::tab_types::PlotType::AllColors;
use crate::gui::tabs::{MyTabs, Tab};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct PhoenixGUI {
    version: String,
    selected: PlotType,
    #[serde(skip)]
    #[cfg(not(target_arch = "wasm32"))]
    sys: System,
    tabs: MyTabs,
    file_viewer: FileViewer,
}

impl Default for PhoenixGUI {
    fn default() -> Self {
        Self {
            version: VERSION.to_owned(),
            selected: AllColors,
            // all_colors: AllColorsPlot::default(),
            // nn: NeuralNetworkPlot::default(),
            #[cfg(not(target_arch = "wasm32"))]
            sys: {
                let mut sys = System::new_all();
                // First we update all information of our `System` struct.
                sys.refresh_all();
                sys
            },
            // other: PlotFile::default(),
            tabs: MyTabs::new(),
            file_viewer: FileViewer::default(),
        }
    }
}

impl PhoenixGUI {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for PhoenixGUI {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                    if ui.button("Open File").clicked() {
                        let mut plot = PlotFile::default();
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            plot.load_file_name = path.display().to_string();
                        }
                        plot.load_data();
                        let mut tab = Tab::new(PlotType::Other, 0);
                        tab.plot = Box::new(plot);
                        self.tabs.tree.set_focused_node(NodeIndex(0));
                        self.tabs.tree.push_to_focused_leaf(tab);
                    }
                    if ui.button("Open Folder").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.file_viewer.set_path(path);
                        }
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show_animated(ctx, true, |ui| {
            ui.heading("Files");
            ui.separator();
            ScrollArea::both().show(ui, |ui| {
                if let Some(path) = self.file_viewer.ui(ui) {
                    println!("Loading file: {}", path.display());
                    if path.extension().is_none() || path.extension().unwrap().to_str().is_none() {
                        println!("Unsupported file type: {}", path.display());
                        self.file_viewer.error =
                            Some(format!("Unsupported file type: {}", path.display()));
                        return;
                    }
                    match path.extension().unwrap().to_str().unwrap() {
                        "csv" => {
                            let mut plot = PlotFile::default();
                            plot.load_file_name = path.display().to_string();
                            plot.load_data();
                            let mut tab = Tab::new(PlotType::Other, 0);
                            tab.plot = Box::new(plot);
                            self.tabs.tree.set_focused_node(NodeIndex(0));
                            self.tabs.tree.push_to_focused_leaf(tab);
                        }
                        "png" | "jpeg" => {
                            let mut plot = ImageTab::default();
                            plot.file_path = path.display().to_string();
                            let mut tab = Tab::new(PlotType::Image, 0);
                            tab.plot = Box::new(plot);
                            self.tabs.tree.set_focused_node(NodeIndex(0));
                            self.tabs.tree.push_to_focused_leaf(tab);
                        }
                        _ => {
                            println!("Unsupported file type: {}", path.display());
                            self.file_viewer.error =
                                Some(format!("Unsupported file type: {}", path.display()));
                        }
                    }
                }
                if self.file_viewer.error.is_some() {
                    self.show_error_window(ctx);
                }
            });
            // ui.label("Made by Nautilus");

            // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            //     ui.horizontal(|ui| {
            //         ui.spacing_mut().item_spacing.x = 0.0;
            //         ui.label("powered by ");
            //         ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            //         ui.label(" and ");
            //         ui.hyperlink_to(
            //             "eframe",
            //             "https://github.com/emilk/egui/tree/master/crates/eframe",
            //         );
            //         ui.label(".");
            //     });
            //     ui.label("Made by Nautilus");
            //     if cfg!(not(target_arch = "wasm32")) {
            //         // display system information:
            //         ui.separator();
            //         // RAM and swap information:
            //         ui.label(format!(
            //             "Total memory: {:.3} GB",
            //             self.sys.total_memory() as f32 / 1024.0 / 1024.0 / 1024.0
            //         ));
            //         ui.label(format!(
            //             "Total swap: {:.3} GB",
            //             self.sys.total_swap() as f32 / 1024.0 / 1024.0 / 1024.0
            //         ));
            //
            //         // Display system information:
            //         ui.label(format!(
            //             "System name:             {}",
            //             self.sys
            //                 .name()
            //                 .unwrap_or("unknown".to_string())
            //                 .replace("\"", "")
            //         ));
            //         ui.label(format!(
            //             "System kernel version:   {}",
            //             self.sys
            //                 .kernel_version()
            //                 .unwrap_or("unknown".to_string())
            //                 .replace("\"", "")
            //         ));
            //         ui.label(format!(
            //             "System OS version:       {}",
            //             self.sys
            //                 .os_version()
            //                 .unwrap_or("unknown".to_string())
            //                 .replace("\"", "")
            //         ));
            //         ui.label(format!(
            //             "System host name:        {}",
            //             self.sys
            //                 .host_name()
            //                 .unwrap_or("unknown".to_string())
            //                 .replace("\"", "")
            //         ));
            //
            //         // Number of CPUs:
            //         ui.label(format!("CPUs #{}", self.sys.cpus().len()));
            //         ui.heading("System Info");
            //         ui.separator();
            //     }
            // });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            // here we show the tabs
            self.tabs.ui(ctx);
        });

        // egui::Window::new("Window").show(ctx, |ui| {
        //     ui.label("Windows can be moved by dragging them.");
        //     ui.label("They are automatically sized based on contents.");
        //     ui.label("You can turn on resizing and scrolling if you like.");
        //     ui.label("You would normally choose either panels OR windows.");
        // });
    }

    /// Called by the frame work to save state before shutdown.
    /// On Windows its saved here: C:\Users\UserName\AppData\Roaming\Phoenix\data\app.ron
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.version = VERSION.to_string();
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl PhoenixGUI {
    /// call this only if [self.file_viewer.error] is Some
    fn show_error_window(&mut self, ctx: &Context) {
        egui::Window::new("Error").show(ctx, |ui| {
            ui.heading("Error");
            ui.separator();
            ui.label(self.file_viewer.error.clone().unwrap());
            // center button
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    if ui.button("Close").clicked() {
                        self.file_viewer.error = None;
                    }
                });
            });
        });
    }
}
