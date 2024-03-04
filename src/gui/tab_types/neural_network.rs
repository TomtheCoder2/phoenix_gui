//! Not to confuse with the other neural_network.rs file in the root directory. This one is for plotting data in the GUI

use crate::gui::tab_types::plot_file::get_color;
use crate::gui::tab_types::PlotStruct;
use crate::neural_network::NNConfig;
use eframe::epaint::Color32;
use egui::plot::Legend;
use egui::plot::{Line, Plot, PlotPoints};
use egui::Ui;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::data_sets::mnist::MNist;
use crate::data_sets::TestSet;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct NeuralNetworkPlot {
    pub learning_rate: f32,
    pub layer_sizes_string: String,
    pub save_file_name: String,
    pub epochs: usize,
    pub batch_number: usize,
    pub update_interval: usize,
    #[serde(skip)]
    pub command_output: Option<(String, Color32)>,
    #[serde(skip)]
    pub nn_thread: JoinHandle<()>,
    #[serde(skip)]
    pub command_sender: Option<Sender<NNCommand>>,
    #[serde(skip)]
    pub command_receiver: Option<Receiver<NNCommand>>,
    #[serde(skip)]
    // cache for the data so we dont have to calculate it every frame
    pub nn_data_cache: Vec<(f32, Vec<f32>)>,
    #[serde(skip)]
    pub nn_start_training: u64,
    #[serde(skip)]
    pub nn_training_took: Duration,
}

impl Default for NeuralNetworkPlot {
    fn default() -> Self {
        let curr_time: u64 = get_time() as u64;
        NeuralNetworkPlot {
            learning_rate: 1.5,
            layer_sizes_string: "4, 40, 80, 2".to_owned(),
            save_file_name: "weights_biases.ron".to_owned(),
            nn_thread: std::thread::spawn(|| {}),
            epochs: 100,
            batch_number: 8,
            command_output: None,
            command_sender: None,
            command_receiver: None,
            nn_data_cache: vec![],
            update_interval: 100,
            nn_start_training: curr_time,
            nn_training_took: Duration::from_secs(0),
        }
    }
}

#[typetag::serde]
impl PlotStruct for NeuralNetworkPlot {
    fn interface(&mut self, ui: &mut Ui) {
        // add some space and a new heading called Neural Network
        ui.separator();
        ui.heading("Neural Network");

        ui.horizontal(|ui| {
            ui.label("Learning Rate ");
            ui.add(egui::DragValue::new(&mut self.learning_rate).speed(0.01));
        });
        ui.horizontal(|ui| {
            ui.label("Epochs ");
            ui.add(egui::DragValue::new(&mut self.epochs).speed(10.0));
        });
        ui.horizontal(|ui| {
            ui.label("Amount of Batches ");
            ui.add(egui::DragValue::new(&mut self.batch_number).speed(1.0));
        });
        ui.horizontal(|ui| {
            ui.label("Update Interval ");
            ui.add(egui::DragValue::new(&mut self.update_interval).speed(1.0));
        });
        ui.horizontal(|ui| {
            ui.label("Layer Sizes ");
            ui.text_edit_singleline(&mut self.layer_sizes_string)
                .on_hover_text("Format: 4, 40, 80, 2");
        });
        ui.horizontal(|ui| {
            ui.label("Save File Name ");
            ui.text_edit_singleline(&mut self.save_file_name)
                .on_hover_text("File Format: .ron");
        });
        // prepare the config for the neural network, to check if there are any errors
        let mut had_error_layer_sizes = None;
        let mut config = NNConfig {
            learning_rate: self.learning_rate,
            epochs: self.epochs,
            layer_sizes: self
                .layer_sizes_string
                .split(", ")
                .map(|s| match s.replace(' ', "").parse::<u32>() {
                    Ok(n) => n,
                    Err(_) => {
                        had_error_layer_sizes = Some(format!("Invalid number: \"{}\"", s));
                        0
                    }
                })
                .collect::<Vec<u32>>(),
            batch_number: self.batch_number,
            command_receiver: None,
            command_sender: None,
            update_interval: self.update_interval,
        };
        if had_error_layer_sizes.is_some() {
            ui.colored_label(Color32::RED, had_error_layer_sizes.clone().unwrap());
        }
        ui.horizontal(|ui| {
            if ui.button("Train").clicked() {
                if self.command_sender.is_some() {
                    println!("Sending stop command");
                    match self.command_sender
                        .as_ref()
                        .unwrap()
                        .send(NNCommand::Stop) {
                        Ok(_) => {

                        }
                        Err(e) => {
                            self.command_output = Some(("Error sending stop command".to_string(), Color32::RED));
                            println!("error: {}", e);
                        }
                    }
                }
                // configure the command sender and receiver
                // sending from the main thread to the nn thread
                let (sender, receiver) = mpsc::channel();
                self.command_sender = Some(sender);
                config.command_receiver = Some(receiver);

                // sending from the nn thread to the main thread
                let (sender2, receiver2) = mpsc::channel();
                self.command_receiver = Some(receiver2);
                config.command_sender = Some(sender2);

                if had_error_layer_sizes.is_none() {
                    // start nn in another thread
                    self.nn_data_cache = vec![];
                    self.nn_start_training = get_time() as u64;
                    self.nn_training_took = Duration::from_secs(0);
                    let t = std::thread::spawn(move || {
                        // crate::data_sets::blue_green::run(config);
                        // self.mnist.run(config);
                        // todo make this static
                        let mut mnist = MNist::default();
                        mnist.run(config);
                    });
                    self.nn_thread = t;
                    self.command_output = Some(("Started".to_string(), Color32::GREEN));
                } else {
                    println!("Error in layer sizes");
                    self.command_sender = None;
                    self.command_receiver = None;
                }
            }
            if ui.button("Pause").clicked() {
                if self.command_sender.is_some() {
                    println!("Sending pause command");
                    self.command_sender
                        .as_ref()
                        .unwrap()
                        .send(NNCommand::Pause)
                        .unwrap();
                    self.command_output = Some(("Paused".to_string(), Color32::YELLOW));
                } else {
                    self.command_output = Some(("No NN running".to_string(), Color32::RED));
                }
            }
            if ui.button("Resume").clicked() {
                if self.command_sender.is_some() {
                    println!("Sending resume command");
                    self.command_sender
                        .as_ref()
                        .unwrap()
                        .send(NNCommand::Resume)
                        .unwrap();
                    self.command_output = Some(("Resumed".to_string(), Color32::GREEN));
                } else {
                    self.command_output = Some(("No NN running".to_string(), Color32::RED));
                }
            }
            if ui.button("Stop").clicked() {
                if self.command_sender.is_some() {
                    println!("Sending stop command");
                    self.command_sender
                        .as_ref()
                        .unwrap()
                        .send(NNCommand::Stop)
                        .unwrap();
                    self.command_sender = None;
                    self.command_output = Some(("Stopped".to_string(), Color32::RED));
                } else {
                    self.command_output = Some(("No NN running".to_string(), Color32::RED));
                }
            }
            if ui.button("Save").clicked() {
                if self.command_sender.is_some() {
                    println!("Sending save command");
                    let mut file_name = self.save_file_name.clone();
                    if !file_name.ends_with(".ron") {
                        file_name.push_str(".ron");
                    }
                    self.command_sender
                        .as_ref()
                        .unwrap()
                        .send(NNCommand::SaveWeightsBiases(file_name.clone()))
                        .unwrap();
                    self.command_output =
                        Some((format!("Saved data to {}", file_name), Color32::GREEN));
                } else {
                    self.command_output = Some(("No NN running".to_string(), Color32::RED));
                }
            }
        });
        if let Some((text, color)) = self.command_output.clone() {
            ui.colored_label(color, text).id = ui.next_auto_id();
        }
        self.update_data();
        if !self.nn_data_cache.is_empty() {
            ui.label(format!(
                "Current Epoch: {}/{}",
                self.nn_data_cache.last().unwrap().0,
                self.epochs
            ));
            ui.label(format!(
                "Current Accuracy: {}%",
                self.nn_data_cache.last().unwrap().1[0]
            ));
            ui.label(format!(
                "Time: {}",
                if self.nn_training_took == Duration::from_secs(0) {
                    format!("{}ms", get_time() as u64 - self.nn_start_training)
                } else {
                    format!("{:#?}", self.nn_training_took)
                }
            ));
        }
    }

    fn plot(&mut self, ui: &mut Ui) {
        self.update_data();
        // render the NN_DATA
        Plot::new(format!(
            "my_plot_{}",
            ui.next_auto_id().short_debug_format()
        ))
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                let mut line_data: Vec<Vec<(f32, f32)>> = vec![];
                for line in &self.nn_data_cache {
                    let x = line.0;
                    for (i, y) in line.1.iter().enumerate() {
                        if line_data.len() <= i {
                            line_data.push(vec![]);
                        }
                        line_data[i].push((x, *y));
                    }
                }
                // collect the columns
                for (i, points) in line_data.into_iter().enumerate() {
                    let points: PlotPoints =
                        points.iter().map(|(x, y)| [*x as f64, *y as f64]).collect();
                    let line = Line::new(points).color(get_color(i));
                    plot_ui.line(line);
                }
            });
    }

    fn title(&self) -> String {
        "Neural Network".to_string()
    }
}

impl NeuralNetworkPlot {
    fn update_data(&mut self) {
        if self.command_receiver.is_some() {
            let mut cmd = self.command_receiver.as_ref().unwrap().try_recv();
            while cmd.is_ok() {
                // handle all commands here
                match cmd.unwrap() {
                    NNCommand::UpdatePlot(epoch, accuracy) => {
                        self.nn_data_cache.push((epoch, accuracy));
                    }
                    NNCommand::SendTime(time) => {
                        self.nn_training_took = time;
                    }
                    _ => {}
                }
                cmd = self.command_receiver.as_ref().unwrap().try_recv();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum NNCommand {
    Stop,
    Pause,
    Resume,
    /// Save the weights and biases to a file with the given name.
    SaveWeightsBiases(String),
    UpdatePlot(f32, Vec<f32>),
    /// says when the training was finished
    SendTime(Duration),
}

impl Eq for NNCommand {}

impl Display for NNCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[inline]
/// Get the current time in milliseconds
pub fn get_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}
