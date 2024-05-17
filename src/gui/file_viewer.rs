use egui::{CollapsingHeader, Ui};
use std::env;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct FileViewer {
    pub curr_dir: PathBuf,
    #[serde(skip)]
    pub error: Option<String>,
    sort_by: SortBy,
    order: Order,
}

#[derive(
    serde::Deserialize, serde::Serialize, EnumIter, PartialEq, Eq, Hash, Debug, Clone, Copy, Display,
)]
pub enum SortBy {
    Name,
    Date,
    Size,
}

#[derive(
    serde::Deserialize, serde::Serialize, EnumIter, PartialEq, Eq, Hash, Debug, Clone, Copy, Display,
)]
pub enum Order {
    Ascending,
    Descending,
}
#[cfg(not(target_arch = "wasm32"))]
fn get_cur_dir() -> PathBuf {
    match env::current_dir() {
        Ok(path) => path,
        Err(_) => PathBuf::new(),
    }
}

#[cfg(target_arch = "wasm32")]
fn get_cur_dir() -> PathBuf {
    PathBuf::new()
}

impl Default for FileViewer {
    fn default() -> Self {
        Self {
            curr_dir: get_cur_dir(),
            error: None,
            sort_by: SortBy::Name,
            order: Order::Ascending,
        }
    }
}

impl FileViewer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Option<PathBuf> {
        let mut res = None;
        // create drop down menu for sort by
        // todo: create a macro for this
        ui.horizontal(|ui| {
            ui.label("Sort by:");
            egui::ComboBox::from_id_source("sort_by")
                .selected_text(format!("{}", self.sort_by))
                .show_ui(ui, |ui| {
                    for sort_by in SortBy::iter() {
                        if ui
                            .selectable_label(self.sort_by == sort_by, format!("{}", sort_by))
                            .clicked()
                        {
                            self.sort_by = sort_by;
                        }
                    }
                });
        });
        ui.horizontal(|ui| {
            ui.label("Order:");
            egui::ComboBox::from_id_source("order")
                .selected_text(format!("{}", self.order))
                .show_ui(ui, |ui| {
                    for order in Order::iter() {
                        if ui
                            .selectable_label(self.order == order, format!("{}", order))
                            .clicked()
                        {
                            self.order = order;
                        }
                    }
                });
        });
        if self.curr_dir.is_dir() {
            res = Self::create_sub_label(ui, &self.curr_dir, self.sort_by, self.order);
        }
        res
    }

    fn create_sub_label(
        ui: &mut Ui,
        path: &Path,
        sort_by: SortBy,
        order: Order,
    ) -> Option<PathBuf> {
        let sub_dirs = read_dir(path).unwrap();
        // println!("current path: {:?}", path);
        path.file_name()?;
        let file_name = match path.file_name().unwrap().to_str() {
            Some(name) => name,
            None => return None,
        };
        let mut res = None;
        CollapsingHeader::new(file_name)
            .default_open(false)
            .show(ui, |ui| {
                let mut sub_dirs = sub_dirs
                    .map(|x| x.unwrap())
                    .collect::<Vec<std::fs::DirEntry>>();
                let sub_dirs = match sort_by {
                    SortBy::Name => {
                        // sort the sub_dirs in a way that if 2 have the same name except a number that the one with the higher number is first
                        // todo fix this
                        sub_dirs.sort_by(|a, b| {
                            let a_name = a.file_name().to_ascii_lowercase();
                            let b_name = b.file_name().to_ascii_lowercase();
                            let a_name = match a_name.to_str() {
                                Some(name) => name,
                                None => return std::cmp::Ordering::Equal,
                            };
                            let b_name = match b_name.to_str() {
                                Some(name) => name,
                                None => return std::cmp::Ordering::Equal,
                            };
                            let a_num = a_name
                                .chars()
                                .rev()
                                .filter(|x| x.is_ascii_digit())
                                .collect::<String>()
                                .chars()
                                .rev()
                                .collect::<String>()
                                .parse::<i32>()
                                .unwrap_or(0);
                            let b_num = b_name
                                .chars()
                                .rev()
                                .filter(|x| x.is_ascii_digit())
                                .collect::<String>()
                                .chars()
                                .rev()
                                .collect::<String>()
                                .parse::<i32>()
                                .unwrap_or(0);
                            if a_name
                                .to_string()
                                .chars()
                                .filter(|x| x.is_alphabetic())
                                .collect::<Vec<_>>()
                                == b_name
                                    .to_string()
                                    .chars()
                                    .filter(|x| x.is_alphabetic())
                                    .collect::<Vec<_>>()
                            {
                                a_num.cmp(&b_num)
                            } else {
                                a_name.cmp(b_name)
                            }
                        });
                        sub_dirs
                    }
                    SortBy::Date => {
                        sub_dirs.sort_by(|a, b| {
                            let a_time = a.metadata().unwrap().modified().unwrap();
                            let b_time = b.metadata().unwrap().modified().unwrap();
                            a_time.cmp(&b_time)
                        });
                        sub_dirs
                    }
                    SortBy::Size => {
                        sub_dirs.sort_by(|a, b| {
                            let a_size = a.metadata().unwrap().len();
                            let b_size = b.metadata().unwrap().len();
                            a_size.cmp(&b_size)
                        });
                        sub_dirs
                    }
                };
                let sub_dirs = match order {
                    Order::Ascending => sub_dirs,
                    Order::Descending => sub_dirs.into_iter().rev().collect(),
                };
                for sub_file in sub_dirs {
                    ui.horizontal(|ui| {
                        let sub_path = sub_file.path();
                        let sub_file_name = sub_path.file_name().unwrap().to_str().unwrap();
                        if sub_path.is_dir() {
                            if let Some(sub_res) =
                                Self::create_sub_label(ui, &sub_path, sort_by, order)
                            {
                                res = Some(sub_res);
                            }
                        } else if ui.button(sub_file_name).clicked() {
                            res = Some(sub_path);
                        }
                    });
                }
            });
        res
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.curr_dir = path;
    }
}
