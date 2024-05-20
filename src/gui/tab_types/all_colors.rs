use crate::data::get_data;
use crate::gui::tab_types::TabStruct;
use crate::matrix::Matrix;
use egui_plot::{Legend, Plot, PlotPoints, Points};
use egui::{Color32, Ui};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum ValueType {
    Red,
    Green,
    Blue,
    Alpha,
    AVGRGB,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AllColorsPlot {
    #[serde(skip)]
    pub plot_data: (Vec<Matrix>, Vec<Matrix>, f32),
    pub x_value: ValueType,
    pub y_value: ValueType,
}

impl Default for AllColorsPlot {
    fn default() -> Self {
        AllColorsPlot {
            plot_data: get_data(),
            x_value: ValueType::Red,
            y_value: ValueType::Green,
        }
    }
}

#[typetag::serde]
impl TabStruct for AllColorsPlot {
    fn show_interface(&mut self) -> bool {
        false
    }
    fn plot(&mut self, ui: &mut Ui) {
        // interface, but because its so small well put it above the plot
        ui.horizontal(|ui| {
            egui::ComboBox::new(ui.next_auto_id(), "X Value")
                .selected_text(format!("{:?}", self.x_value))
                .show_ui(ui, |ui| {
                    for option in [
                        ValueType::Red,
                        ValueType::Green,
                        ValueType::Blue,
                        ValueType::Alpha,
                        ValueType::AVGRGB,
                    ] {
                        ui.selectable_value(&mut self.x_value, option, option.to_string());
                    }
                });
            egui::ComboBox::new(ui.next_auto_id(), "Y Value")
                .selected_text(format!("{:?}", self.y_value))
                .show_ui(ui, |ui| {
                    for option in [
                        ValueType::Red,
                        ValueType::Green,
                        ValueType::Blue,
                        ValueType::Alpha,
                        ValueType::AVGRGB,
                    ] {
                        ui.selectable_value(&mut self.y_value, option, option.to_string());
                    }
                });
        });
        let data = &self.plot_data;
        let mut data_to_plot = vec![];
        for _ in 0..data.1[0].data.len() {
            data_to_plot.push(vec![])
        }
        for i in 0..data.0.len() {
            let r = (data.0[i].data[0] + 1.0) * data.2 / 2.0;
            let g = (data.0[i].data[1] + 1.0) * data.2 / 2.0;
            let b = (data.0[i].data[2] + 1.0) * data.2 / 2.0;
            let a = (data.0[i].data[3] + 1.0) * data.2 / 2.0;
            // println!("x: {}, y: {}, z: {}", x, y, z);
            // get index of the max value of data.1[i]
            let mut max_index = 0;
            let mut max_value = data.1[i].data[0];
            for j in 1..data.1[i].rows {
                if data.1[i].data[j] > max_value {
                    max_value = data.1[i].data[j];
                    max_index = j;
                }
            }
            let x = match self.x_value {
                ValueType::Red => r,
                ValueType::Green => g,
                ValueType::Blue => b,
                ValueType::Alpha => a,
                ValueType::AVGRGB => (r + g + b) / 3.0,
            };
            let y = match self.y_value {
                ValueType::Red => r,
                ValueType::Green => g,
                ValueType::Blue => b,
                ValueType::Alpha => a,
                ValueType::AVGRGB => (r + g + b) / 3.0,
            };
            data_to_plot[max_index].push((x as f64, y as f64));
        }
        // convert to PlotPoints
        // let data_to_plot: Vec<PlotPoints> = data_to_plot.iter().map(|x| x.iter().map(|(x, y)| [*x, *y]).collect()).collect();
        // * 0 - black
        // * 1 - white
        // * 2 - blue
        // * 3 - green
        // * 4 - yellow
        // * 5 - red
        // * 6 - nothing
        let colors = vec![
            "Black", "White", "Blue", "Green", "Yellow", "Red", "Nothing",
        ];
        Plot::new(ui.next_auto_id())
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                for i in 0..data_to_plot.len() {
                    let points = Points::new(
                        data_to_plot[i]
                            .iter()
                            .map(|(x, y)| [*x, *y])
                            .collect::<PlotPoints>(),
                    )
                    .color(match i {
                        0 => Color32::GOLD,
                        1 => Color32::WHITE,
                        2 => Color32::BLUE,
                        3 => Color32::GREEN,
                        4 => Color32::YELLOW,
                        5 => Color32::RED,
                        _ => Color32::from_rgb(159, 43, 104),
                    })
                    .radius(1.0)
                    .highlight(true)
                    .name(colors[i]);
                    plot_ui.points(points);
                }
            });
    }

    fn title(&self) -> String {
        "All Colors".to_string()
    }
}
