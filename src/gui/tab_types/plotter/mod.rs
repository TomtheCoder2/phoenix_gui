pub mod compiler;
mod functions;
pub mod parser;
pub mod precedence;
pub mod stack;
pub mod vm;

use crate::gui::tab_types::plot_file::get_color;
use crate::gui::tab_types::plotter::compiler::Compiler;
use crate::gui::tab_types::plotter::parser::Operation;
use crate::gui::tab_types::plotter::vm::VM;
use crate::gui::tab_types::PlotStruct;
use egui::plot::Legend;
use egui::plot::Line;
use egui::plot::Plot;
use egui::plot::PlotPoints;
use egui::{Color32, Ui};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
struct Parameters {
    min: f64,
    max: f64,
    amount: usize,
    // first the string and then the compiled number or an error as a string
    values: HashMap<String, (String, Result<f64, String>)>,
    x_index: Vec<usize>,
    extras: Vec<Extra>,
}

fn euqal(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

impl Eq for Parameters {}

impl PartialEq for Parameters {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min
            && self.max == other.max
            && self.amount == other.amount
            && self.values == other.values
            && self.x_index == other.x_index
            && self.extras == other.extras
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 1.0,
            amount: 1000,
            values: Default::default(),
            x_index: vec![],
            extras: vec![],
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
struct Extra {
    integral: bool,
    derivative: bool,
    integral_start: f64,
}

impl Eq for Extra {}

impl PartialEq for Extra {
    fn eq(&self, other: &Self) -> bool {
        self.integral == other.integral
            && self.derivative == other.derivative
            && euqal(self.integral_start, other.integral_start)
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Input {
    pub function_string: String,
    #[serde(skip)]
    pub last_function_string: String,
    #[serde(skip)]
    pub error: Option<String>,
    #[serde(skip)]
    pub time: Option<Duration>,
    pub instructions: Option<(Vec<Operation>, Vec<String>)>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            function_string: "0".to_string(),
            last_function_string: "0".to_string(),
            error: None,
            time: None,
            instructions: None,
        }
    }
}

struct PlotData {
    x_y: Vec<(f64, f64)>,
    derivative: Option<Vec<(f64, f64)>>,
    integral: Option<Vec<(f64, f64)>>,
    name: String,
}

impl Default for PlotData {
    fn default() -> Self {
        Self {
            x_y: vec![],
            derivative: None,
            integral: None,
            name: "".to_string(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Plotter {
    inputs: Vec<Input>,
    #[serde(skip)]
    last_input_len: usize,
    current_parameters: Parameters,
    #[serde(skip)]
    last_parameters: Parameters,
    #[serde(skip)]
    plot_data: Vec<PlotData>,
}

impl Default for Plotter {
    fn default() -> Self {
        Self {
            inputs: vec![Input::default()],
            last_input_len: 1,
            current_parameters: Parameters::default(),
            last_parameters: Parameters::default(),
            plot_data: Vec::new(),
        }
    }
}

#[typetag::serde]
impl PlotStruct for Plotter {
    fn interface(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Min: ");
            ui.add(egui::DragValue::new(&mut self.current_parameters.min).speed(0.01));
        });
        ui.horizontal(|ui| {
            ui.label("Max: ");
            ui.add(egui::DragValue::new(&mut self.current_parameters.max).speed(0.01));
        });
        ui.horizontal(|ui| {
            ui.label("Amount: ");
            ui.add(egui::DragValue::new(&mut self.current_parameters.amount).speed(1.0));
        });
        ui.label("Values: ");
        let mut added = Vec::new();
        let mut to_add = Vec::new();
        for instr_index in 0..self.inputs.len() {
            let instr = if let Some(instr) = &self.inputs[instr_index].instructions {
                instr
            } else {
                continue;
            };
            for i in 0..instr.1.len() {
                if instr.1[i] == "x" {
                    if i >= self.current_parameters.x_index.len() {
                        self.current_parameters.x_index.push(instr_index);
                    } else {
                        self.current_parameters.x_index[i] = instr_index;
                    }
                    continue;
                }
                if added.contains(&instr.1[i]) {
                    continue;
                }
                added.push(instr.1[i].clone());
                if !self.current_parameters.values.contains_key(&instr.1[i]) {
                    self.current_parameters
                        .values
                        .insert(instr.1[i].clone(), ("0".to_string(), Ok(0.0)));
                }
                to_add.push(instr.1[i].clone());
            }
        }
        // sort the values
        to_add.sort();
        for add in to_add {
            ui.horizontal(|ui| {
                ui.label(&add);
                // ui.add(
                //     egui::DragValue::new(self.current_parameters.values.get_mut(&add).unwrap())
                //         .speed(0.01),
                // );
                ui.text_edit_singleline(
                    &mut self.current_parameters.values.get_mut(&add).unwrap().0,
                );
            });
            match &self.current_parameters.values.get_mut(&add).unwrap().1 {
                Ok(v) => {
                    ui.label(format!("= {:.3}", v));
                }
                Err(e) => {
                    ui.colored_label(Color32::RED, format!("= 0 ({})", e));
                }
            }
        }
    }

    fn plot(&mut self, ui: &mut Ui) {
        // interface, but because its so small well put it above the plot
        for i in 0..self.inputs.len() {
            ui.horizontal(|ui| {
                if ui.button("-").clicked() {
                    self.inputs.remove(i);
                    if i < self.current_parameters.x_index.len() {
                        self.current_parameters.x_index.remove(i);
                    }
                    return;
                }
                if i >= self.inputs.len() {
                    return;
                }
                ui.label(format!("#{}: f(x) = ", i + 1));
                ui.text_edit_singleline(&mut self.inputs[i].function_string);
                if ui.button("Compile").clicked() {
                    self.gen_data();
                }
                if i >= self.current_parameters.extras.len() {
                    self.current_parameters.extras.push(Extra::default());
                }
                ui.checkbox(&mut self.current_parameters.extras[i].integral, "Integral");
                ui.checkbox(
                    &mut self.current_parameters.extras[i].derivative,
                    "Derivative",
                );
                if self.current_parameters.extras[i].integral {
                    ui.horizontal(|ui| {
                        ui.label("Start: ");
                        ui.add(
                            egui::DragValue::new(
                                &mut self.current_parameters.extras[i].integral_start,
                            )
                            .speed(0.01),
                        );
                    });
                }
                if self.inputs.len() > i {
                    if let Some(t) = &self.inputs[i].time {
                        ui.colored_label(
                            Color32::GREEN,
                            format!(
                                "(tot: {:?}, per: {:?})",
                                t,
                                Duration::from_nanos(
                                    (t.as_nanos() / self.current_parameters.amount as u128) as u64
                                )
                            ),
                        );
                    }
                }
            });
            if self.inputs.len() > i {
                if let Some(e) = &self.inputs[i].error {
                    ui.colored_label(Color32::RED, e);
                }
            }
        }
        if ui.button("+").clicked() {
            self.inputs.push(Input::default());
        }
        // check if self.current_parameters is equal self.last_parameters and if so run self.compile();
        if self.current_parameters != self.last_parameters
            || self
                .inputs
                .iter()
                .any(|x| x.function_string != x.last_function_string)
            || self.inputs.len() != self.last_input_len
        {
            self.gen_data();
            self.inputs.iter_mut().for_each(|x| {
                x.last_function_string = x.function_string.clone();
            });
            self.last_parameters = self.current_parameters.clone();
            self.last_input_len = self.inputs.len();
        }
        // plot
        Plot::new(ui.next_auto_id())
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                // convert all plot_data to one long array and look if derivative and integral are non None
                let data = self.plot_data.iter().fold(Vec::new(), |mut acc, x| {
                    acc.push((&x.x_y, x.name.clone()));
                    if let Some(derivative) = &x.derivative {
                        acc.push((derivative, format!("{}'", x.name)));
                    }
                    if let Some(integral) = &x.integral {
                        acc.push((integral, format!("I{}", x.name)));
                    }
                    acc
                });
                for (i, data) in data.iter().enumerate() {
                    plot_ui.line(
                        Line::new(data.0.iter().map(|x| [x.0, x.1]).collect::<PlotPoints>())
                            .color(get_color(i))
                            .name(data.1.clone()),
                    );
                }
            });
    }

    fn title(&self) -> String {
        "Plotter".to_string()
    }
}

impl Plotter {
    fn gen_data(&mut self) {
        // self.plot_data.clear();
        // compile all the parameters and check for errors
        for param in &mut self.current_parameters.values {
            match Compiler::new().optimized_compile(param.1 .0.clone()) {
                Ok(instructions) => {
                    let value = VM::run((&*instructions.0, &*instructions.1), &[]);
                    param.1 .1 = value;
                }
                Err(e) => {
                    param.1 .1 = Err(e);
                }
            }
        }
        for i in 0..self.inputs.len() {
            // check if something changed
            if self.inputs[i].function_string.is_empty()
                || (self.inputs[i].function_string == self.inputs[i].last_function_string
                    && self.current_parameters.extras.len() > i
                    && self.last_parameters.extras.len() > i
                    && self.current_parameters.extras[i] == self.last_parameters.extras[i]
                    && self.current_parameters.min == self.last_parameters.min
                    && self.current_parameters.max == self.last_parameters.max
                    && self.current_parameters.amount == self.last_parameters.amount
                    && self.current_parameters == self.last_parameters)
            {
                continue;
            }
            if self.plot_data.len() <= i {
                self.plot_data.push(PlotData::default());
            } else {
                self.plot_data[i].x_y.clear();
                self.plot_data[i].derivative = None;
                self.plot_data[i].integral = None;
            }
            // compile the code to instructions
            match Compiler::new().optimized_compile(self.inputs[i].function_string.clone()) {
                Ok(instructions) => {
                    self.inputs[i].instructions = Some(instructions);
                    self.inputs[i].error = None;
                }
                Err(e) => {
                    self.inputs[i].error = Some(e);
                    self.inputs[i].instructions = None;
                }
            }
            if self.inputs[i].instructions.is_none() {
                continue;
            }
            let instructions = self.inputs[i].instructions.as_ref().unwrap();
            let mut values = Vec::new();
            let mut x_index = None;
            for j in 0..instructions.1.len() {
                if instructions.1[j] == "x" {
                    values.push(0.0);
                    x_index = Some(j);
                } else {
                    let default: (String, Result<f64, String>) = ("0".to_string(), Ok(0.0));
                    values.push(
                        *self
                            .current_parameters
                            .values
                            .get(&instructions.1[j])
                            .unwrap_or(&default)
                            .1
                            .as_ref()
                            .unwrap_or(&0.0),
                    );
                }
            }
            while values.len() < instructions.1.len() {
                values.push(0.0);
            }
            // do one tests with the vm
            match VM::run((&*instructions.0, &*instructions.1), &values) {
                Ok(_) => {}
                Err(e) => {
                    self.inputs[i].error = Some(e);
                    continue;
                }
            }
            let mut error = None;
            let start_time = Instant::now();
            let data_to_plot = (0..self.current_parameters.amount + 1)
                .map(|x| {
                    // we want to map x between self.current_parameters.min and self.current_parameters.max with self.current_parameters.amount steps
                    let x = self.current_parameters.min
                        + (self.current_parameters.max - self.current_parameters.min) * x as f64
                            / (self.current_parameters.amount + 1) as f64;
                    if let Some(x_index) = x_index {
                        values[x_index] = x;
                    }
                    (
                        x,
                        match VM::run((&*instructions.0, &*instructions.1), &values) {
                            Ok(x) => x,
                            Err(e) => {
                                error = Some(e);
                                0.0
                            }
                        },
                    )
                })
                .collect::<Vec<(f64, f64)>>();
            if let Some(e) = error {
                self.inputs[i].error = Some(e);
                self.inputs[i].time = None;
            } else {
                let time = Instant::now().duration_since(start_time);
                self.inputs[i].time = Some(time);
            }
            let mut p_integral = None;
            let mut p_derivative = None;
            if self.current_parameters.extras.get(i).is_some()
                && self.current_parameters.extras[i].integral
            {
                let mut integral = self.current_parameters.extras[i].integral_start;
                let mut last_x = data_to_plot[0].0;
                let mut last_y = data_to_plot[0].1;
                let mut integrals = Vec::new();
                for (x, y) in data_to_plot.clone() {
                    integral += (y + last_y) * (x - last_x) / 2.0;
                    last_x = x;
                    last_y = y;
                    integrals.push((x, integral));
                }
                p_integral = Some(integrals);
            }
            if self.current_parameters.extras.get(i).is_some()
                && self.current_parameters.extras[i].derivative
            {
                let mut last_x = data_to_plot[0].0;
                let mut last_y = data_to_plot[0].1;
                let mut derivatives = Vec::new();
                for (x, y) in data_to_plot.clone() {
                    derivatives.push((x, (y - last_y) / (x - last_x)));
                    last_x = x;
                    last_y = y;
                }
                p_derivative = Some(derivatives);
            }
            // check if self.plot_data is long enough, else fill with PlotData::default
            while self.plot_data.len() <= i {
                self.plot_data.push(PlotData::default());
            }
            self.plot_data[i] = PlotData {
                x_y: data_to_plot,
                name: format!("{}", i + 1),
                integral: p_integral,
                derivative: p_derivative,
            };
        }
        if self.plot_data.len() > self.inputs.len() {
            self.plot_data.truncate(self.inputs.len());
        }
    }
}
