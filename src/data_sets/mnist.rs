use std::process::exit;
use crate::data_sets::TestSet;
use crate::matrix::Matrix;
use image::RgbImage;

extern crate image;

use image::{ImageBuffer, Rgb};
use lz4_compression::decompress::decompress;
use serde::{Deserialize, Serialize};
use crate::neural_network::{NeuralNetwork, NNConfig};

#[derive(Deserialize, Serialize, Debug)]
pub struct MNist {
    loaded: bool,
    pub train_input: Vec<Matrix>,
    pub train_target: Vec<Matrix>,
    pub test_input: Vec<Matrix>,
    pub test_target: Vec<Matrix>,
}

impl Default for MNist {
    fn default() -> Self {
        Self {
            loaded: false,
            train_input: vec![],
            train_target: vec![],
            test_input: vec![],
            test_target: vec![],
        }
    }
}

static TRAIN_DATA: &str = include_str!("..\\resources/mnist_train.csv");
static TEST_DATA: &str = include_str!("..\\resources/mnist_test.csv");

static TEST_SET: &'static [u8; 32240033] = include_bytes!("..\\resources/mnist_data.bin");

impl TestSet for MNist {
    fn read(&mut self) {
        if self.loaded {
            return;
        }
        // let decompressed_data = match decompress(&TEST_SET.to_vec()) {
        //     Ok(s) => s,
        //     Err(e) => {
        //         panic!("Error decompressing file: {:?}", e);
        //     }
        // };
        let data_set: MNist = bincode::deserialize(TEST_SET).unwrap();
        self.train_input = data_set.train_input;
        self.train_target = data_set.train_target;
        self.test_input = data_set.test_input;
        self.test_target = data_set.test_target;
        self.loaded = true;
    }
}

impl MNist {
    pub fn read_files(&mut self) {
        // read data from train_data
        println!("Reading train data...");
        // skip first line
        let mut lines = TRAIN_DATA.lines();
        lines.next();
        let (train_input, train_target) = Self::read_lines(lines);
        self.train_input = train_input;
        self.train_target = train_target;
    }
    fn read_lines(lines: core::str::Lines) -> (Vec<Matrix>, Vec<Matrix>) {
        let mut input: Vec<Matrix> = vec![];
        let mut target: Vec<Matrix> = vec![];
        const LIMIT: usize = 1000000;
        for (i, line) in lines.enumerate() {
            let mut parts = line.split(",");
            let mut target_matrix = Matrix::new(10, 1);
            let mut input_matrix = Matrix::new(784, 1);
            let value = parts.next().unwrap().parse::<usize>().unwrap();
            target_matrix.set(value, 0, 1.0);
            for i in 0..784 {
                let value = parts.next().unwrap().parse::<f32>().unwrap();
                input_matrix.set(i, 0, value / 255.0);
            }
            input.push(input_matrix);
            target.push(target_matrix);
            if i > LIMIT {
                break;
            }
        }
        (input, target)
    }

    pub fn print_data(data: Vec<Matrix>) {
        println!("Getting image...");
        let len = data.len() as f64;
        let width: u32 = 28 * len.sqrt() as u32;
        let height: u32 = 28 * len.sqrt() as u32;
        let mut image: RgbImage = ImageBuffer::new(width, height);
        let mut counter = 0;
        for col in 0..len.sqrt() as u32 {
            for row in 0..len.sqrt() as u32 {
                for i in 0..28 {
                    for j in 0..28 {
                        if let Some(matrix) = data.get(counter) {
                            let value = matrix.get((i * 28 + j) as usize, 0);
                            let value = (value * 255.0) as u8;
                            // add col and row to the i j variable
                            *image.get_pixel_mut(i + col * 28, j + row * 28) = Rgb([value, value, value]);
                        }
                    }
                }
                counter += 1;
            }
        }
        // *image.get_pixel_mut(5, 5) = image::Rgb([255, 255, 255]);
        image.save("output.png").unwrap();
    }

    pub fn run(&mut self, config: NNConfig) {
        if config.epochs == 0 {
            return;
        }
        if !self.loaded {
            let mut data_set = MNist::default();
            // let t1 = std::time::Instant::now();
            // data_set.read();
            // println!("Time to read bin: {:?}", t1.elapsed());
            let t1 = std::time::Instant::now();
            data_set.read_files();
            println!("Time to read files: {:?}", t1.elapsed());
            self.train_input = data_set.train_input;
            self.train_target = data_set.train_target;
            // check if they contain NaNs
            for matrix in self.train_input.iter() {
                if matrix.contains_nan() {
                    panic!("Train input contains NaNs");
                }
                if matrix.max() > 1.0 {
                    panic!("Train input contains values greater than 1.0");
                }
            }
            for matrix in self.train_target.iter() {
                if matrix.contains_nan() {
                    panic!("Train target contains NaNs");
                }
                if matrix.max() > 1.0 {
                    panic!("Train input contains values greater than 1.0");
                }
            }
        }
        let mut nn = NeuralNetwork::new(config.layer_sizes);
        nn.learning_rate = config.learning_rate;
        // nn.command_receiver = config.command_receiver;
        nn.command_sender = config.command_sender;
        nn.update_interval = config.update_interval;
        // todo: dont clone
        nn.train_epochs_m(self.train_input.clone(), self.train_target.clone(), config.batch_number, config.epochs);
    }
}
