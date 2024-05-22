use egui::{Color32, Context, Key, Stroke, warn_if_debug_build};
use egui::ScrollArea;
use egui_dock::{NodeIndex, SurfaceIndex, TabIndex};
#[cfg(not(target_arch = "wasm32"))]
use sysinfo::System;

use crate::gui::file_viewer::FileViewer;
use crate::gui::tab_types::image::ImageTab;
use crate::gui::tab_types::plot_file::PlotFile;
use crate::gui::tab_types::PlotType;
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
    show_file_viewer: bool,
    #[serde(skip)]
    plot_rect: Option<egui::Rect>,
    screenshot_file: String,
    #[serde(skip)]
    do_screenshot: bool,
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
            show_file_viewer: true,
            plot_rect: None,
            screenshot_file: "screenshot.png".to_string(),
            do_screenshot: false,
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

pub fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    // Widget code can be broken up in four steps:
    //  1. Decide a size for the widget
    //  2. Allocate space for it
    //  3. Handle interactions with the widget (if any)
    //  4. Paint the widget

    // 1. Deciding widget size:
    // You can query the `ui` how much space is available,
    // but in this example we have a fixed size widget based on the height of a standard button:
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 2.0);

    // 2. Allocating space:
    // This is where we get a region of the screen assigned.
    // We also tell the Ui to sense clicks in the allocated region.
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    // 3. Interact: Time to check for clicks!
    if response.clicked() {
        *on = !*on;
        response.mark_changed(); // report back that the value changed
    }

    // Attach some meta-data to the response which can be used by screen readers:
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    // 4. Paint!
    // Make sure we need to paint:
    if ui.is_rect_visible(rect) {
        // // Let's ask for a simple animation from egui.
        // // egui keeps track of changes in the boolean associated with the id and
        // // returns an animated value in the 0-1 range for how much "on" we are.
        // let how_on = ui.ctx().animate_bool(response.id, *on);
        // // We will follow the current style by asking
        // // "how should something that is being interacted with be painted?".
        // // This will, for instance, give us different colors when the widget is hovered or clicked.
        // let visuals = ui.style().interact_selectable(&response, *on);
        // // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
        // let rect = rect.expand(visuals.expansion);
        // let radius = 0.5 * rect.height();
        // ui.painter()
        //     .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        // // Paint the circle, animating it from left to right with `how_on`:
        // let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        // let center = egui::pos2(circle_x, rect.center().y);
        // ui.painter()
        //     .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
        egui::Image::new(egui::include_image!("../resources/icons/folder.png"))
            .rounding(5.0)
            .tint(egui::Color32::LIGHT_BLUE)
            .paint_at(ui, rect);
    }

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, ...) and maybe show a tooltip:

    // add a border around the image when hovered
    if response.hovered() {
        let visuals = ui.style().interact_selectable(&response, *on);
        ui.painter().rect(rect, 5.0, Color32::from_black_alpha(0), visuals.bg_stroke);
    }
    response
}

impl eframe::App for PhoenixGUI {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // sysinfo is not available on web
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    // if ui.button("Quit").clicked() {
                    //     _frame.close();
                    // }
                    if ui.button("Open File").clicked() {
                        let mut plot = PlotFile::default();
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            plot.load_file_name = path.display().to_string();
                        }
                        plot.load_data();
                        let current_tab = self.tabs.tree.focused_leaf().unwrap_or((SurfaceIndex::main(), NodeIndex(0)));
                        let mut tab = Tab::new(PlotType::Other, 0);
                        tab.plot = Box::new(plot);
                        self.tabs.tree.set_focused_node_and_surface(current_tab);
                        self.tabs.tree.push_to_focused_leaf(tab);
                    }
                    if ui.button("Open Folder").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.file_viewer.set_path(path);
                        }
                    }

                    if ui.button("Screenshot File").clicked() {
                        if let Some(path) = rfd::FileDialog::new().set_file_name("screenshot.png").add_filter("image", &["png", "jpg"]).save_file() {
                            self.screenshot_file = path.display().to_string();
                            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
                        }
                    }

                    if ui.button("Save Screenshot").clicked() {
                        ui.close_menu();
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
                    }
                });
            });
            // if ctrl+shift+s is pressed, save screenshot
            // Check for returned screenshot:
            ui.input(|i| {
                let keys = &i.keys_down;
                let modifiers = i.modifiers;
                let ctrl_shift_s = keys.contains(&Key::S) && modifiers.ctrl && modifiers.shift;
                if ctrl_shift_s {
                    self.do_screenshot = true;
                }
                for event in &i.raw.events {
                    if let egui::Event::Screenshot { image, .. } = event {
                        let pixels_per_point = i.pixels_per_point();
                        let region = egui::Rect::from_two_pos(
                            egui::Pos2::ZERO,
                            egui::Pos2 { x: 100., y: 100. },
                        );
                        // let top_left_corner = image.region(&region, Some(pixels_per_point));
                        let rect = self.plot_rect.unwrap_or(egui::Rect::EVERYTHING);
                        let top_left_corner = image.region(&rect, Some(pixels_per_point));
                        image::save_buffer(
                            self.screenshot_file.clone(),
                            top_left_corner.as_raw(),
                            top_left_corner.width() as u32,
                            top_left_corner.height() as u32,
                            image::ColorType::Rgba8,
                        )
                            .unwrap();
                        println!("Screenshot saved to: {}", self.screenshot_file);
                    }
                }
            });
            if self.do_screenshot {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
                self.do_screenshot = false;
            }
        });
        egui::SidePanel::left("tabs_panel")
            .max_width(10.0)
            .resizable(false)
            .show_animated(ctx, true, |ui| {
                egui_extras::install_image_loaders(ctx);
                // if ui.add(Button::image(include_image!("../resources/icons/folder.png"))).clicked() {
                //     self.show_file_viewer = !self.show_file_viewer;
                // }
                toggle_ui(ui, &mut self.show_file_viewer);
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    warn_if_debug_build(ui);
                    ui.label(format!("{}", VERSION));
                });
            });
        if self.show_file_viewer {
            egui::SidePanel::left("files_panel").show_animated(ctx, true, |ui| {
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
                                // first check if there's already a tab with this file
                                let mut to_focus = None;
                                for (surface_index, surface) in self.tabs.tree.iter_surfaces().enumerate() {
                                    for (node_index, node) in surface.iter_nodes().enumerate() {
                                        if node.tabs().is_none() {
                                            continue;
                                        }
                                        for (tab_index, tab) in node.tabs().unwrap().iter().enumerate() {
                                            if tab.plot.get_file_path().is_none() {
                                                continue;
                                            }
                                            if tab.plot.get_file_path().unwrap() == path.display().to_string() {
                                                to_focus = Some((SurfaceIndex(surface_index), NodeIndex(node_index), TabIndex(tab_index)));
                                                break;
                                            }
                                        }
                                    }
                                }
                                if let Some(to_focus) = to_focus {
                                    println!("File already open: {}, to_focus: {:?}", path.display(), to_focus);
                                    self.tabs.tree.set_active_tab(to_focus);
                                    return;
                                }
                                let current_tab = self.tabs.tree.focused_leaf();
                                let mut plot = PlotFile::default();
                                plot.load_file_name = path.display().to_string();
                                plot.load_data();
                                let mut tab = Tab::new(PlotType::Other, 0);
                                tab.plot = Box::new(plot);
                                // get focused surface
                                // self.tabs.tree.set_focused_node_and_surface((NodeIndex(0), 0));
                                self.tabs.tree.push_to_focused_leaf(tab);
                            }
                            "png" | "jpeg" => {
                                let mut plot = ImageTab::default();
                                plot.file_path = path.display().to_string();
                                let mut tab = Tab::new(PlotType::Image, 0);
                                tab.plot = Box::new(plot);
                                // self.tabs.tree.set_focused_node_and_surface(NodeIndex(0));
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
        }

        self.plot_rect = Some(egui::CentralPanel::default().show(ctx, |_ui| {
            // here we show the tabs
            self.tabs.ui(ctx);
        }).response.rect);

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
