use crate::gui::tab_types::PlotStruct;
use eframe::epaint::Color32;
use egui::plot::{Legend, Line, Plot, PlotPoints};
use egui::{CollapsingHeader, ScrollArea, Ui};
#[cfg(target_arch = "wasm32")]
use egui_file::FileDialog;
use std::fs::File;
use std::io::Read;

/// Colors for the plot
pub const INDEX_COLORS: [&str; 128] = [
    "#ffffff", "#FFFF00", "#1CE6FF", "#FF34FF", "#FF4A46", "#008941", "#006FA6", "#A30059",
    "#FFDBE5", "#7A4900", "#0000A6", "#63FFAC", "#B79762", "#004D43", "#8FB0FF", "#997D87",
    "#5A0007", "#809693", "#FEFFE6", "#1B4400", "#4FC601", "#3B5DFF", "#4A3B53", "#FF2F80",
    "#61615A", "#BA0900", "#6B7900", "#00C2A0", "#FFAA92", "#FF90C9", "#B903AA", "#D16100",
    "#DDEFFF", "#000035", "#7B4F4B", "#A1C299", "#300018", "#0AA6D8", "#013349", "#00846F",
    "#372101", "#FFB500", "#C2FFED", "#A079BF", "#CC0744", "#C0B9B2", "#C2FF99", "#001E09",
    "#00489C", "#6F0062", "#0CBD66", "#EEC3FF", "#456D75", "#B77B68", "#7A87A1", "#788D66",
    "#885578", "#FAD09F", "#FF8A9A", "#D157A0", "#BEC459", "#456648", "#0086ED", "#886F4C",
    "#34362D", "#B4A8BD", "#00A6AA", "#452C2C", "#636375", "#A3C8C9", "#FF913F", "#938A81",
    "#575329", "#00FECF", "#B05B6F", "#8CD0FF", "#3B9700", "#04F757", "#C8A1A1", "#1E6E00",
    "#7900D7", "#A77500", "#6367A9", "#A05837", "#6B002C", "#772600", "#D790FF", "#9B9700",
    "#549E79", "#FFF69F", "#201625", "#72418F", "#BC23FF", "#99ADC0", "#3A2465", "#922329",
    "#5B4534", "#FDE8DC", "#404E55", "#0089A3", "#CB7E98", "#A4E804", "#324E72", "#6A3A4C",
    "#83AB58", "#001C1E", "#D1F7CE", "#004B28", "#C8D0F6", "#A3A489", "#806C66", "#222800",
    "#BF5650", "#E83000", "#66796D", "#DA007C", "#FF1A59", "#8ADBB4", "#1E0200", "#5B4E51",
    "#C895C5", "#320033", "#FF6832", "#66E1D3", "#CFCDAC", "#D0AC94", "#7ED379", "#012C58",
];

type PlotData = Vec<(Vec<(f32, f32)>, Color32, String)>;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct PlotFile {
    #[serde(skip)]
    // list of lines
    pub data: PlotData,
    #[serde(skip)]
    raw_data: Vec<Vec<Option<f32>>>,
    #[serde(skip)]
    header: Vec<String>,
    pub load_file_name: String,
    #[serde(skip)]
    pub loading_error: Option<String>,
    #[serde(skip)]
    pub load_data_message: Option<String>,
    /// use the first value for the x coordinate of the "other" plots
    pub x_axis: i16,
    /// List of scaling factors for each line, maybe some plot is always 10x as big as the other lines
    #[serde(skip)]
    pub scaling_factors: Vec<f64>,
    #[serde(skip)]
    pub comments: Vec<String>,
    // #[serde(skip)]
    // #[cfg(target_arch = "wasm32")]
    // open_file_dialog: Option<FileDialog>,
}

impl Default for PlotFile {
    fn default() -> Self {
        PlotFile {
            data: vec![],
            raw_data: vec![],
            header: vec![],
            load_file_name: "data.csv".to_owned(),
            loading_error: None,
            load_data_message: None,
            x_axis: -1,
            scaling_factors: vec![],
            comments: vec![],
        }
    }
}

#[typetag::serde]
impl PlotStruct for PlotFile {
    fn interface(&mut self, ui: &mut Ui) {
        // choose file to load
        ui.horizontal(|ui| {
            ui.label("File Name ");
            // ui.text_edit_singleline(&mut self.load_file_name).on_hover_text("File Format: .csv");
            if ui.button("Open file…").clicked() {
                if cfg!(not(target_arch = "wasm32")) {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.load_file_name = path.display().to_string();
                        }
                    }
                } else {
                    // if (ui.button("Open")).clicked() {
                    //     let mut dialog = FileDialog::open_file(
                    //         match File::open(self.load_file_name.clone()).unwrap() {
                    //             Ok(file) => Some(file),
                    //             Err(_) => None,
                    //         });
                    //     dialog.open();
                    //     self.open_file_dialog = Some(dialog);
                    // }
                    //
                    // if let Some(dialog) = &mut self.open_file_dialog {
                    //     if dialog.show(ctx).selected() {
                    //         if let Some(file) = dialog.path() {
                    //             self.opened_file = Some(file);
                    //         }
                    //     }
                    // }
                }
                self.load_data();
            }
            if ui.button("Reload").clicked() {
                self.load_data();
            }
        });
        ui.label(format!("File: {}", self.load_file_name));
        // ui.checkbox(&mut self.x_axis, "Use first variable for the X axis");
        // add menu to select the variable for the x axis default is index of the variables
        ui.horizontal(|ui| {
            ui.label("X axis: ");
            egui::ComboBox::from_label("X axis")
                .selected_text(
                    if self.x_axis == -1 || self.x_axis as usize >= self.data.len() {
                        "index".to_string()
                    } else {
                        self.data[self.x_axis as usize].2.clone()
                    },
                )
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.x_axis, -1, "index");
                    for i in 0..self.scaling_factors.len() {
                        if i >= self.data.len() {
                            // idk why this is needed but it is
                            break;
                        }
                        ui.selectable_value(
                            &mut self.x_axis,
                            i as i16,
                            self.data[i].2.clone().to_string(),
                        );
                    }
                });
        });
        // todo add option to scroll through the data
        CollapsingHeader::new("Scaling factors")
            .default_open(false)
            .show(ui, |ui| {
                for i in 0..self.scaling_factors.len() {
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut self.scaling_factors[i]).speed(0.01));
                        ui.label(format!(
                            "{}{}",
                            self.data[i].2.clone(),
                            if self.x_axis == i as i16 {
                                " (X axis)"
                            } else {
                                ""
                            }
                        ));
                    });
                }
            });
        if let Some(error) = self.loading_error.clone() {
            ui.colored_label(Color32::RED, error);
            self.load_data_message = None;
        }
        if let Some(message) = self.load_data_message.clone() {
            ui.colored_label(Color32::GREEN, message);
        }
        if !self.comments.is_empty() {
            ui.separator();
            CollapsingHeader::new("Comments")
                .default_open(true)
                .show(ui, |ui| {
                    ScrollArea::both().show(ui, |ui| {
                        for comment in self.comments.clone() {
                            ui.label(comment);
                        }
                    });
                });
        }
    }

    fn title(&self) -> String {
        format!("Plot File: {}", self.load_file_name)
    }

    fn name(&self) -> String {
        format!(
            "Plot File: {}",
            self.load_file_name
                .replace('\\', "/")
                .split('/')
                .last()
                .unwrap_or("")
        )
    }

    fn plot(&mut self, ui: &mut Ui) {
        // render self.other_data
        let data = self.data.clone();
        // this time we want lines
        Plot::new("my_plot")
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                self.update_data();
                let mut i = 0;
                for line in data {
                    if i == self.x_axis as usize {
                        i += 1;
                        continue;
                    }
                    let points: PlotPoints = line
                        .0
                        .iter()
                        .map(|(x, y)| {
                            [
                                (*x as f64) * self.scaling_factors[0],
                                *y as f64 * self.scaling_factors[i],
                            ]
                        })
                        .collect();
                    let line = Line::new(points).color(line.1).name(line.2);
                    plot_ui.line(line);
                    i += 1;
                }
            });
    }
}

impl PlotFile {
    pub(crate) fn load_data(&mut self) {
        self.loading_error = None;
        self.load_data_message = None;
        self.comments = Vec::new();
        // we try to load the file, using the first line as the header
        let mut data = Vec::new();
        let header;
        let file = match File::open(&self.load_file_name) {
            Ok(f) => Some(f),
            Err(_) => {
                self.loading_error =
                    Some(format!("Could not open file \"{}\"", self.load_file_name));
                None
            }
        };
        if let Some(..) = file {
            let mut file = file.unwrap();
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    let mut lines = contents.lines();
                    header = match lines.next() {
                        Some(h) => h
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect::<Vec<String>>(),
                        None => {
                            self.loading_error =
                                Some(format!("File \"{}\" is empty", self.load_file_name));
                            return;
                        }
                    };
                    let mut i = 2;
                    for line in lines {
                        if line.is_empty() {
                            continue;
                        }
                        if line.starts_with('#') {
                            // this is a comment
                            self.comments.push(format!(
                                "{}: {}",
                                i,
                                line.trim_start_matches('#').trim()
                            ));
                            continue;
                        }
                        let mut row = Vec::new();
                        for value in line.split(',') {
                            // strip whitespaces
                            let value = value.trim();
                            row.push(match value.parse::<f32>() {
                                Ok(v) => Some(v),
                                Err(_) => match &*value.to_ascii_lowercase() {
                                    "null" | "none" => None,
                                    _ => {
                                        self.loading_error = Some(format!(
                                            "Could not parse value \"{}\" (line {})",
                                            value, i
                                        ));
                                        None
                                    }
                                },
                            });
                        }
                        data.push(row);
                        i += 1;
                    }
                    self.header = header;
                    self.raw_data = data;
                    self.update_data();
                }
                Err(_) => {
                    self.loading_error =
                        Some(format!("Could not read file \"{}\"", self.load_file_name));
                }
            }
        }
    }

    pub fn update_data(&mut self) {
        if self.loading_error.is_none() {
            self.data.clear();
            // self.scaling_factors.clear();
            // we assume that the first column is the x axisx
            // let start = if self.x_axis != -1 { 1 } else { 0 };
            let header = self.header.clone();
            let data = self.raw_data.clone();
            for i in 0..header.len() {
                // if i == self.x_axis as usize {
                //     continue;
                // }
                let mut column = Vec::new();
                let mut j = 0;
                for row in &data {
                    if row.len() <= i {
                        continue;
                    }
                    if self.x_axis != -1 {
                        if row[0].is_some() && row[i].is_some() {
                            column.push((row[self.x_axis as usize].unwrap(), row[i].unwrap()));
                        }
                    } else {
                        if row[i].is_some() {
                            column.push((j as f32, row[i].unwrap()));
                        }
                        j += 1;
                    }
                }
                let (r, g, b) = convert_hex_to_rgb(INDEX_COLORS[i % 128]);
                self.data
                    .push((column, Color32::from_rgb(r, g, b), header[i].clone()));
            }
            for i in 0..header.len() {
                // if the self.scaling_factors is empty, we fill it with 1.0
                if self.scaling_factors.len() <= i {
                    self.scaling_factors.push(1.0);
                }
            }
            while self.scaling_factors.len() > header.len() {
                self.scaling_factors.pop();
            }
            self.load_data_message = Some(format!(
                "Loaded {} rows with format {}",
                data.len(),
                header.join(", ")
            ));
        }
    }
}

/// Converts strings like #00ff00 to rgb values
pub fn convert_hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let mut hex = hex.to_string();
    if hex.starts_with('#') {
        hex.remove(0);
    }
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}

/// Returns a color from the INDEX_COLORS array, which should be different for each index
pub fn get_color(index: usize) -> Color32 {
    let (r, g, b) = convert_hex_to_rgb(INDEX_COLORS[index % 128]);
    Color32::from_rgb(r, g, b)
}
