use eframe::epaint::Color32;
use egui::plot::{Legend, Line, Plot, PlotPoints};
use std::fmt::{Debug, Display};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::spawn;

use crate::gui::tab_types::plot_file::{get_color, INDEX_COLORS};
use egui::Ui;
use phoenix_rec::client::create_client;
use phoenix_rec::data_types::DataType;
use phoenix_rec::Data::{RecordData, RecordDataOption};
use phoenix_rec::{get_rec_data, get_rec_index, get_rec_len, Data, clear_data};
use phoenix_rec::client::client_alive;

use crate::gui::tab_types::PlotStruct;

pub type PlotData = Vec<(Vec<(f32, f32)>, Color32, String)>;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TCPClient {
    data: PlotData,
    data_len: usize,
    server_name: String,
    #[serde(skip)]
    main_sender: Option<Sender<String>>,
    #[serde(skip)]
    main_receiver: Option<Receiver<String>>,
    #[serde(skip)]
    message: String,
}

impl Default for TCPClient {
    fn default() -> Self {
        TCPClient {
            data: vec![],
            data_len: 0,
            server_name: "localhost".to_string(),
            main_sender: None,
            main_receiver: None,
            message: "".to_string(),
        }
    }
}

macro_rules! lm {
    ($mutex:expr) => {
        $mutex.lock().expect("Mutex poisoned")
    };
}

#[typetag::serde]
impl PlotStruct for TCPClient {
    fn show_interface(&mut self) -> bool {
        true
    }

    fn interface(&mut self, ui: &mut Ui) {
        ui.label("TCP Client");
        ui.text_edit_singleline(&mut self.server_name);
        if ui.button("Connect").clicked() {
            if !client_alive() {
                clear_data();
                self.message = "Connecting...".to_string();
                // create a tunnel for communication between the 2 threads
                let server_name = self.server_name.clone();
                let (main_sender, thread_receiver) = std::sync::mpsc::channel();
                let (thread_sender, main_receiver) = std::sync::mpsc::channel();
                self.main_sender = Some(main_sender);
                self.main_receiver = Some(main_receiver);
                // create new thread and start the client int here
                thread::spawn(move || {
                    create_client(server_name, thread_receiver, thread_sender);
                });
            } else {
                self.message = "Client already connected".to_string();
            }
        }
        // try to receive something on the main_receiver
        if let Some(main_receiver) = &self.main_receiver {
            if let Ok(msg) = main_receiver.try_recv() {
                self.message = msg;
                println!("Message: {}", self.message);
            }
        }
        ui.label(format!("{}", self.message));
    }

    fn plot(&mut self, ui: &mut Ui) {
        ui.label("TCP Client");
        if self.data_len != get_rec_len() {
            self.read();
            println!("Data len: {}", self.data.len());
            // println!("Data: {:?}", self.data);
        }
        // render self.other_data
        let data = self.data.clone();
        // this time we want lines
        Plot::new("my_plot")
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                // self.update_data();
                let mut i = 0;
                for line in data {
                    // if i == self.x_axis as usize {
                    //     i += 1;
                    //     continue;
                    // }
                    let points: PlotPoints = line
                        .0
                        .iter()
                        .map(|(x, y)| [(*x as f64), *y as f64])
                        .collect();
                    let line = Line::new(points).color(line.1).name(line.2);
                    plot_ui.line(line);
                    i += 1;
                }
            });
    }

    fn title(&self) -> String {
        "TCP Client".to_string()
    }
}

impl TCPClient {
    fn read(&mut self) {
        self.data.clear();
        println!("Reading data len: {}", get_rec_len());

        // println!("Data: {:?}", DATA);
        let mut data = vec![];
        let mut used = vec![];
        let rec_data = get_rec_data();
        self.data_len = rec_data.data.len();
        for dat in &rec_data.data {
            match dat {
                RecordData(t, d) => {
                    // todo: maybe index compression
                    let mut temp = vec![];
                    for i in d {
                        // convert to u8
                        let index = i.to_u8();
                        while temp.len() <= index as usize {
                            temp.push(None);
                        }
                        if !used.contains(&index) {
                            used.push(index);
                        }
                        temp[index as usize] = Some(*i);
                    }
                    data.push(RecordDataOption(*t, temp));
                }
                Data::Command(s) => {
                    data.push(Data::Command(s.clone()));
                }
                _ => {
                    panic!("Data is not RecordData or Command");
                }
            }
        }
        used.sort();
        for (i, name) in used
            .iter()
            .map(|i| {
                DataType::from_repr(*i)
                    .expect("DataType not found")
                    .write_description()
            })
            .filter(|x| x != &"".to_string())
            .collect::<Vec<String>>()
            .iter()
            .flat_map(|x| x.split(", "))
            .collect::<Vec<&str>>()
            .into_iter()
            .enumerate()
        {
            self.data.push((vec![], get_color(i), name.to_string()));
        }
        // println!("data: {:?}", self.data);
        // todo reimplement this
        // file.write_all(
        //     format!(
        //         "# k_p_drive: {}, k_i_drive: {}, k_d_drive: {}\n",
        //         KPDRIVE.load(SeqCst),
        //         KIDRIVE.load(SeqCst),
        //         KDDRIVE.load(SeqCst)
        //     )
        //     .as_bytes(),
        // )
        // .unwrap();
        // println!("Data: {:?}", data);
        for data in data {
            match data {
                RecordDataOption(t, d) => {
                    for (i, x) in d.iter().enumerate() {
                        if let Some(x) = x {
                            // DataType::Color(r, l) => format!("{}, {}", r, l),
                            //             DataType::Distance(d) => format!("{}", d),
                            //             DataType::CalcSpeed(r, l) => format!("{}, {}", r, l),
                            //             DataType::SyncSpeed(r, l) => format!("{}, {}", r, l),
                            //             DataType::RealSpeeds(r, l) => format!("{}, {}", r, l),
                            //             DataType::DrivenDistance(r, l) => format!("{}, {}", r, l),
                            //             DataType::SyncError(e) => format!("{}", e),
                            //             DataType::Correction(r, l) => format!("{}, {}", r, l),
                            match x {
                                DataType::Color(r, l)
                                | DataType::CalcSpeed(r, l)
                                | DataType::SyncSpeed(r, l)
                                | DataType::RealSpeeds(r, l) => {
                                    self.data[i].0.push((t as f32, *r as f32));
                                    self.data[i + 1].0.push((t as f32, *l as f32));
                                }
                                DataType::DrivenDistance(r, l) | DataType::Correction(r, l) => {
                                    self.data[i].0.push((t as f32, *r));
                                    self.data[i + 1].0.push((t as f32, *l));
                                }
                                DataType::Distance(d) => {
                                    self.data[i].0.push((t as f32, *d as f32));
                                }
                                DataType::SyncError(d) => {
                                    self.data[i].0.push((t as f32, *d));
                                }

                                DataType::None(_) => {}
                            }
                        }
                    }
                }
                Data::Command(s) => {}
                _ => {
                    panic!("Data::RecordDataOption or Data::Command expected");
                }
            }
        }
    }
}
