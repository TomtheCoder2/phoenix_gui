use crate::gui::tab_types::all_colors::AllColorsPlot;
use crate::gui::tab_types::geometry::Geometry;
use crate::gui::tab_types::image::ImageTab;

use crate::gui::tab_types::neural_network::NeuralNetworkPlot;
use crate::gui::tab_types::plot_file::PlotFile;
use crate::gui::tab_types::plotter::Plotter;
use egui::Ui;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use strum_macros::EnumIter;

pub mod all_colors;
pub mod geometry;
pub mod image;

pub mod neural_network;
pub mod plot_file;
pub mod plotter;
pub mod tcp_client;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, EnumIter)]
pub enum PlotType {
    AllColors,
    NeuralNetwork,
    Other,
    Image,
    Geometry,
    Plotter,
    TCPClient,
}

impl Display for PlotType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// Represents a plot type
// todo: maybe add serde via :erased_serde::Serialize + erased_serde::Deserializer<'a> + Default
#[typetag::serde(tag = "type")]
pub trait PlotStruct {
    fn interface(&mut self, _ui: &mut Ui) {}
    fn show_interface(&mut self) -> bool {
        true
    }
    fn plot(&mut self, _ui: &mut Ui) {}
    fn title(&self) -> String;
    fn name(&self) -> String {
        self.title()
    }
    fn update(&mut self, _ui: &mut Ui, _frame: &mut eframe::Frame) {}
}

pub fn default_plot(plot_type: PlotType) -> Box<dyn PlotStruct> {
    match plot_type {
        PlotType::AllColors => Box::<AllColorsPlot>::default(),
        PlotType::NeuralNetwork => Box::<NeuralNetworkPlot>::default(),
        PlotType::Other => Box::<PlotFile>::default(),
        PlotType::Image => Box::<ImageTab>::default(),
        PlotType::Geometry => Box::<Geometry>::default(),
        PlotType::Plotter => Box::<Plotter>::default(),
        PlotType::TCPClient => Box::<tcp_client::TCPClient>::default(),
    }
}
