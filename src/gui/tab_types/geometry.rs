use crate::gui::tab_types::PlotStruct;
use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct Geometry {
    a: f32,
    b: f32,
    c: f32,
    alpha: f32,
    beta: f32,
    gamma: f32,
}

#[typetag::serde]
impl PlotStruct for Geometry {
    fn interface(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("A: ");
            ui.add(egui::DragValue::new(&mut self.a).speed(1.0));
        });
        ui.horizontal(|ui| {
            ui.label("B: ");
            ui.add(egui::DragValue::new(&mut self.b).speed(1.0));
        });
        ui.horizontal(|ui| {
            ui.label("C: ");
            ui.add(egui::DragValue::new(&mut self.c).speed(1.0));
        });
        ui.horizontal(|ui| {
            ui.label("Alpha: ");
            ui.add(egui::DragValue::new(&mut self.alpha).speed(1.0));
        });
        ui.horizontal(|ui| {
            ui.label("Beta: ");
            ui.add(egui::DragValue::new(&mut self.beta).speed(1.0));
        });
        ui.horizontal(|ui| {
            ui.label("Gamma: ");
            ui.add(egui::DragValue::new(&mut self.gamma).speed(1.0));
        });

        // update all the values
        let mut a = self.a.abs();
        let mut b = self.b.abs();
        let mut c = self.c.abs();
        let mut alpha = self.alpha.abs();
        let mut beta = self.beta.abs();
        let mut gamma = self.gamma.abs();
        // check if we have enough values to make a unique triangle
        let mut number_of_givens = 0;
        if a > 0.0 {
            number_of_givens += 1;
        }
        if b > 0.0 {
            number_of_givens += 1;
        }
        if c > 0.0 {
            number_of_givens += 1;
        }
        if alpha > 0.0 {
            number_of_givens += 1;
        }
        if beta > 0.0 {
            number_of_givens += 1;
        }
        if gamma > 0.0 {
            number_of_givens += 1;
        }
        if number_of_givens < 3 {
            ui.colored_label(
                egui::Color32::RED,
                "Not enough values to make a unique triangle",
            );
            return;
        }
        // calculate all the values
        if a == 0.0 {
            if b != 0.0 && c != 0.0 && alpha != 0.0 {
                a = (b.powi(2) + c.powi(2) - 2.0 * b * c * alpha.to_radians().cos()).sqrt();
            } else if b != 0.0 && c != 0.0 && beta != 0.0 {
                a = (b.powi(2) + c.powi(2) - 2.0 * b * c * beta.to_radians().cos()).sqrt();
            } else if b != 0.0 && c != 0.0 && gamma != 0.0 {
                a = (b.powi(2) + c.powi(2) - 2.0 * b * c * gamma.to_radians().cos()).sqrt();
            } else {
                ui.colored_label(egui::Color32::RED, "Not enough values to calculate A");
                return;
            }
        }
        // now have a for sure
        if b != 0.0 {
            if c != 0.0 && alpha != 0.0 {
                b = (a.powi(2) + c.powi(2) - 2.0 * a * c * alpha.to_radians().cos()).sqrt();
            } else if c != 0.0 && beta != 0.0 {
                b = (a.powi(2) + c.powi(2) - 2.0 * a * c * beta.to_radians().cos()).sqrt();
            } else if c != 0.0 && gamma != 0.0 {
                b = (a.powi(2) + c.powi(2) - 2.0 * a * c * gamma.to_radians().cos()).sqrt();
            } else {
                ui.colored_label(egui::Color32::RED, "Not enough values to calculate B");
                return;
            }
        }
        // now have b for sure
        if c != 0.0 {
            if alpha != 0.0 {
                c = (a.powi(2) + b.powi(2) - 2.0 * a * b * alpha.to_radians().cos()).sqrt();
            } else if beta != 0.0 {
                c = (a.powi(2) + b.powi(2) - 2.0 * a * b * beta.to_radians().cos()).sqrt();
            } else if gamma != 0.0 {
                c = (a.powi(2) + b.powi(2) - 2.0 * a * b * gamma.to_radians().cos()).sqrt();
            } else {
                ui.colored_label(egui::Color32::RED, "Not enough values to calculate C");
                return;
            }
        }
        if alpha == 0.0 {
            if beta != 0.0 && gamma != 0.0 {
                alpha = (b.powi(2) + c.powi(2) - a.powi(2)) / (2.0 * b * c);
                alpha = alpha.acos().to_degrees();
            } else {
                ui.colored_label(egui::Color32::RED, "Not enough values to calculate Alpha");
                return;
            }
        }
        if beta == 0.0 {
            if alpha != 0.0 && gamma != 0.0 {
                beta = (a.powi(2) + c.powi(2) - b.powi(2)) / (2.0 * a * c);
                beta = beta.acos().to_degrees();
            } else {
                ui.colored_label(egui::Color32::RED, "Not enough values to calculate Beta");
                return;
            }
        }
        if gamma == 0.0 {
            if alpha != 0.0 && beta != 0.0 {
                gamma = (a.powi(2) + b.powi(2) - c.powi(2)) / (2.0 * a * b);
                gamma = gamma.acos().to_degrees();
            } else {
                ui.colored_label(egui::Color32::RED, "Not enough values to calculate Gamma");
                return;
            }
        }
        // now we have all the values
        ui.label(format!("A: {}", a));
        ui.label(format!("B: {}", b));
        ui.label(format!("C: {}", c));
        ui.label(format!("Alpha: {}", alpha));
        ui.label(format!("Beta: {}", beta));
        ui.label(format!("Gamma: {}", gamma));
    }
    fn title(&self) -> String {
        "Geometry".to_string()
    }
}
