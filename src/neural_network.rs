//! Neural network module
// todo: add more comments and maybe merge the 2 structs

use crate::matrix::Matrix;

use rand::prelude::SliceRandom;

use std::fs::File;
use std::sync::atomic::{AtomicUsize, Ordering};

use std::sync::mpsc::Receiver;

use std::sync::mpsc::Sender;


use std::time::Instant;

/// Represents a neural network (but is used internally for the training, because the weights and biases are not mutable)
/// Use this when you want to predict a value. Note that this struct is immutable
///
/// Explanation why there are two structs:
/// - This struct is only used to predict the values and then in the training function do the backpropagation and return the corrections
///   And because it needs to be shared between threads, it needs to be immutable (its also better for ram usage, because the weights and biases are not copied)
/// - The other struct is used for the training and is mutable, because it needs to be changed
#[derive(Debug)]
pub struct NeuralNetworkWorker<'a> {
    /// Weights of the neural network
    pub weights: &'a Vec<Matrix>,
    /// Biases of the neural network
    pub biases: &'a Vec<Matrix>,
    /// Learning Rate of the neural network
    pub learning_rate: f32,
    /// List of the layer sized (first one should be the same size as the input and last one should be as big as thee output vector)
    pub layer_sizes: &'a Vec<u32>,
}

impl<'a> NeuralNetworkWorker<'a> {
    /// Construct a new neural network with given sizes<br>
    /// **Important:** The size of the first layer has to be the same as the input size and same for the last layer and output
    pub fn new(
        layer_sizes: &'a Vec<u32>,
        weights: &'a Vec<Matrix>,
        biases: &'a Vec<Matrix>,
    ) -> NeuralNetworkWorker<'a> {
        NeuralNetworkWorker {
            learning_rate: 0.005,
            layer_sizes,
            weights,
            biases,
        }
    }

    pub fn convert_to_matrix(data: &Vec<Vec<Vec<f32>>>) -> Vec<Matrix> {
        let mut m: Vec<Matrix> = vec![];
        for item in data {
            m.push(Matrix::from_2d_vec(item));
        }
        m
    }

    /// Forward propagation
    pub fn predict(&self, inputs: Vec<f32>) -> Vec<f32> {
        let mut layers: Vec<Matrix> = vec![Matrix::from_vec(&inputs)];

        for i in 1..self.layer_sizes.len() {
            // print!("{}", i);
            layers.push(Matrix::multiply_two(&self.weights[i - 1], &layers[i - 1]));
            layers[i].add_matrix(&self.biases[i - 1]);
            layers[i].sigmoid();
            // println!("layers[{}]: {:?}", i, layers[i].data);
        }
        // println!();
        layers[layers.len() - 1].to_vec()
    }

    /// One Training step
    /// Returns the corrections for the weights and biases
    /// The bool is true if the prediction was correct
    pub fn train(&self, input: &Vec<f32>, target: &Vec<f32>) -> (Vec<(Matrix, Matrix)>, bool) {
        // predict the output
        // convert the input to a matrix
        let mut layers: Vec<Matrix> = Vec::new();
        layers.push(Matrix::from_vec(input));

        for i in 1..self.layer_sizes.len() {
            layers.push(Matrix::multiply_two(&self.weights[i - 1], &layers[i - 1]));
            layers[i].add_matrix(&self.biases[i - 1]);
            layers[i].sigmoid();
        }

        // error detection and correction
        let target = Matrix::from_vec(target);

        // calculate the error between the output and the correct output (target)
        let mut error = Matrix::subtract_two(&target, &layers[layers.len() - 1]);
        // println!("output: {:?}, target: {:?} => error: {:?}", layers[layers.len() - 1].data, target.data, error.data);
        // let avg_error = error.average();
        // println!("avg_error: {}", avg_error);
        let mut transposed;

        // use the derivative sigmoid function to correct the error
        let mut corrections = Vec::new();
        // println!("layers #{}: {:?}", layers.len() - 1, layers[layers.len() - 1].data);
        corrections.push(self.correct_error(layers.len() - 1, &layers, &error));
        // for (int i = layers.size() - 2; i > 0; i--) {
        for i in (1..layers.len() - 1).rev() {
            // println!("i: {}", i);
            // todo fix this
            let mut w = self.weights[i].clone();
            w.add_matrix(&corrections[0].1);
            transposed = Matrix::transpose(&w);
            error = Matrix::multiply_two(&transposed, &error);
            // println!("error: {:?}", error.data);
            // println!("layers #{}: {:?}", i, layers[i].data);
            corrections.insert(0, self.correct_error(i, &layers, &error));
        }
        // println!("layers #{}: {:?}", 0, layers[0].data);
        // println!();
        // println!("corrections: {:?}", corrections);
        (
            corrections,
            NeuralNetworkWorker::check(&layers[layers.len() - 1], &target),
        )
    }

    /// Correct the error
    /// ## Returns
    /// * The gradient of the hidden layer -> bias
    /// * The delta of the weights between the hidden and input layer -> weights
    fn correct_error(&self, i: usize, layers: &[Matrix], error: &Matrix) -> (Matrix, Matrix) {
        // todo: optimize this (maybe dont clone the matrix)
        let mut gradient = layers[i].clone().sigmoid_derivative();
        gradient.multiply_with_matrix(error);
        gradient.multiply_with_f32(self.learning_rate);

        let delta = Matrix::multiply_two(&gradient, &Matrix::transpose(&layers[i - 1]));
        (gradient, delta)
    }

    /// Train one batch and return the avg weights and biases correction
    /// Format: list of nns (list of layers (matrix of biases and weights))
    pub fn train_batch(
        &self,
        batch: Vec<(Vec<f32>, Vec<f32>)>,
    ) -> (Vec<Vec<(Matrix, Matrix)>>, usize) {
        let mut res = vec![];
        let mut correct = 0;
        for (input, target) in &batch {
            let corrections = self.train(input, target);
            if corrections.1 {
                correct += 1;
            }
            res.push(corrections.0);
        }
        (res, correct)
    }

    /// Check if the prediction is correct
    pub fn check(prediction: &Matrix, target: &Matrix) -> bool {
        // get the highest value of the prediction
        let mut max = 0.0;
        let mut max_index = 0;
        for i in 0..prediction.data.len() {
            if prediction.data[i] > max {
                max = prediction.data[i];
                max_index = i;
            }
        }
        let mut m = Matrix::from_vec(&vec![0.0; prediction.data.len()]);
        m.data[max_index] = 1.0;

        // check if the prediction is correct
        let mut correct = true;
        for i in 0..prediction.data.len() {
            if m.data[i] != target.data[i] {
                correct = false;
            }
        }
        correct
    }
}

pub fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// Represents the configuration of a Neural Network
pub struct NNConfig {
    pub learning_rate: f32,
    pub layer_sizes: Vec<u32>,
    pub epochs: usize,
    pub batch_number: usize,


    pub command_receiver: Option<Receiver<NNCommand>>,


    pub command_sender: Option<Sender<NNCommand>>,
    pub update_interval: usize,
}

impl Default for NNConfig {
    fn default() -> Self {
        NNConfig {
            learning_rate: 0.005,
            layer_sizes: vec![4, 80, 86, 7],
            epochs: 1000,
            batch_number: 100,


            command_receiver: None,


            command_sender: None,
            update_interval: 100,
        }
    }
}



use crate::gui::tab_types::neural_network::NNCommand;

use serde::{Deserialize, Serialize};


/// A Neural Network, use this one for saving and loading
#[derive(Debug, Deserialize, Serialize)]
pub struct WeightsBiases {
    weights: Vec<Vec<Vec<f32>>>,
    biases: Vec<Vec<Vec<f32>>>,
    layer_sizes: Vec<u32>,
}


/// A Neural Network, use this one for training and the other one for predicting
#[derive(Debug)]
pub struct NeuralNetwork {
    pub learning_rate: f32,
    pub layer_sizes: Vec<u32>,
    pub weights: Vec<Matrix>,
    pub biases: Vec<Matrix>,


    pub command_receiver: Option<Receiver<NNCommand>>,


    pub command_sender: Option<Sender<NNCommand>>,
    pub update_interval: usize,
}



impl NeuralNetwork {
    pub fn new(layer_sizes: Vec<u32>) -> NeuralNetwork {
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        for i in 0..layer_sizes.len() - 1 {
            weights.push(Matrix::random(
                layer_sizes[i + 1] as usize,
                layer_sizes[i] as usize,
            ));
            biases.push(Matrix::random(layer_sizes[i + 1] as usize, 1));
        }
        NeuralNetwork {
            learning_rate: 0.005,
            layer_sizes,
            weights,
            biases,

            command_receiver: None,

            command_sender: None,
            update_interval: 100,
        }
    }

    /// One Epoch:
    /// Create a batch of n neural networks, and divide the training data for those n networks
    /// Let them train and then average the weights and biases of the networks
    /// Return the averaged weights and biases and then average the weights and bias correction over all the networks and apply them to the neural network
    /// Run each batch in a thread
    /// Returns the amount of correct predictions
    pub fn train_batches(
        &mut self,
        input: Vec<Vec<f32>>,
        target: Vec<Vec<f32>>,
        batch_number: usize,
    ) -> usize {
        // first we divide the training data into batches
        let mut batches = Vec::new();
        for _ in 0..batch_number {
            batches.push(vec![]);
        }
        // shuffle the data
        let mut input = input;
        let mut target = target;
        let mut rng = rand::thread_rng();
        let mut indices = (0..input.len()).collect::<Vec<_>>();
        indices.shuffle(&mut rng);
        input = indices.iter().map(|&i| input[i].clone()).collect();
        target = indices.iter().map(|&i| target[i].clone()).collect();
        for i in 0..input.len() {
            batches[i % batch_number].push((input[i].clone(), target[i].clone()));
        }
        // now we average the weights and biases correction over all the batches
        let mut weights_correction = vec![];
        let mut biases_correction = vec![];
        let mut correct = 0;
        crossbeam::scope(|scope| {
            let mut corrections = Vec::new();
            let mut handles = Vec::new();

            for (i, current_batch) in batches.iter().enumerate().take(batch_number) {
                // create thread for each batch and train it, without cloning the neural network because we correct the weights and biases of the neural network after the training
                let cloned_network = NeuralNetworkWorker {
                    learning_rate: self.learning_rate / input.len() as f32,
                    layer_sizes: &self.layer_sizes,
                    weights: &self.weights,
                    biases: &self.biases,
                };

                scope.builder().name(format!("thread #{}", i));
                let handler = scope.spawn(move |_| {
                    // let handle = thread::current();
                    // println!("Thread started");
                    cloned_network.train_batch(current_batch.clone())
                    // println!("Thread {} finished", handle.name().unwrap());
                });
                handles.push(handler);
            }
            for handle in handles {
                let res = handle.join().unwrap();
                for r in res.0 {
                    corrections.push(r);
                }
                correct += res.1;
            }
            // init the weights and biases correction
            for i in 0..self.layer_sizes.len() - 1 {
                weights_correction.push(Matrix::new(
                    self.layer_sizes[i + 1] as usize,
                    self.layer_sizes[i] as usize,
                ));
                biases_correction.push(Matrix::new(self.layer_sizes[i + 1] as usize, 1));
            }
            // println!("correction lens: {} [0]:{}, [0][0].0:{:?}", corrections.len(), corrections[0].len(), corrections[0][0].0);
            // println!("biases_correction #{}: {:?}", 0, corrections[0][0].0);
            for correction in corrections {
                for i in 0..correction.len() {
                    // println!("correction #{}: {:?}", i, correction[i]);
                    biases_correction[i].add_matrix(&correction[i].0);
                    weights_correction[i].add_matrix(&correction[i].1);
                }
            }
            // for i in 0..weights_correction.len() {
            //     weights_correction[i].multiply_with_f32(1.0 / input.len() as f32);
            //     biases_correction[i].multiply_with_f32(1.0 / input.len() as f32);
            // }
        })
        .expect("Error while training batches");
        // now we apply the weights and biases correction to the neural network
        for i in 0..weights_correction.len() {
            // println!("weights_correction #{}: {:?}", i, weights_correction[i]);
            self.weights[i].add_matrix(&weights_correction[i]);
            self.biases[i].add_matrix(&biases_correction[i]);
        }
        correct
    }

    /// Train the neural network with the given input and target for x epochs
    pub fn train_epochs(
        &mut self,
        input: Vec<Vec<f32>>,
        target: Vec<Vec<f32>>,
        batch_number: usize,
        epochs: usize,
    ) {
        println!("input len: {}", input.len());
        let output_file = File::create("output.csv").unwrap();
        let mut writer = csv::Writer::from_writer(output_file);
        let time1 = Instant::now();
        for i in 0..epochs {
            let t1 = Instant::now();
            let correct = self.train_batches(input.clone(), target.clone(), batch_number);
            println!("Training took: {:?}", t1.elapsed());
            if i % self.update_interval == 0 || i == epochs - 1 {
                println!("Epoch: {}", i);
                if cfg!(feature = "gui") {
                    // send command to update plot
                    self.command_sender
                        .as_ref()
                        .unwrap()
                        .send(NNCommand::UpdatePlot(
                            i as f32,
                            vec![correct as f32 / input.len() as f32 * 100.0],
                        ))
                        .unwrap();
                } else {
                    // lets save it to the output file
                    writer
                        .write_record(&[i.to_string(), (correct).to_string()])
                        .unwrap();
                    println!("Epoch: {}, Correct: {}", i, correct);
                }
            }
            if cfg!(feature = "gui") && self.check_command() {
                return;
            }
        }
        if !cfg!(feature = "gui") {
            writer.flush().unwrap();
        }
        let time2 = Instant::now();
        println!("Training took: {:?}", time2.duration_since(time1));
        if cfg!(feature = "gui") && self.command_receiver.is_some() {
            // send command to say how long the training took
            self.command_sender
                .as_ref()
                .unwrap()
                .send(NNCommand::SendTime(time2.duration_since(time1)))
                .unwrap();
            loop {
                if self.check_command() {
                    break;
                }
            }
        }
    }

    fn check_command(&self) -> bool {
        let mut stop = false;
        if self.command_receiver.is_some() {
            let command_receiver = self.command_receiver.as_ref().unwrap();
            if let Ok(command) = command_receiver.try_recv() {
                match command {
                    NNCommand::Stop => {
                        stop = true;
                    }
                    NNCommand::Pause => loop {
                        if let Ok(command) = command_receiver.try_recv() {
                            match command {
                                NNCommand::Stop => {
                                    stop = true;
                                    break;
                                }
                                NNCommand::Resume => {
                                    break;
                                }
                                _ => {}
                            }
                        }
                    },
                    NNCommand::SaveWeightsBiases(file_name) => {
                        self.save_weights_biases(file_name);
                    }
                    _ => {
                        println!("Unknown command");
                    }
                }
            }
        }
        stop
    }

    /// Save weights and biases to a file in the current directory called
    pub fn save_weights_biases(&self, file_name: String) {
        println!("Saving to file: {}", file_name);
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        for i in 0..self.weights.len() {
            weights.push(self.weights[i].clone().to_2d_vec());
            biases.push(self.biases[i].clone().to_2d_vec());
        }

        let weights_biases = WeightsBiases {
            weights,
            biases,
            layer_sizes: self.layer_sizes.clone(),
        };
        let file = File::create(file_name).unwrap();
        ron::ser::to_writer(file, &weights_biases).unwrap();
    }

    /// Load weights and biases from a file in the current directory
    /// todo: tests this
    pub fn load_from_save(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let weights_biases: WeightsBiases = ron::de::from_reader(file).unwrap();
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        for i in 0..weights_biases.weights.len() {
            weights.push(Matrix::from_2d_vec(&weights_biases.weights[i]));
            biases.push(Matrix::from_2d_vec(&weights_biases.biases[i]));
        }
        NeuralNetwork {
            learning_rate: 0.1,
            layer_sizes: weights_biases.layer_sizes,
            weights,
            biases,

            command_receiver: None,

            command_sender: None,
            update_interval: 100,
        }
    }

    /// Train the neural network with the given input and target for x epochs
    pub fn train_epochs_m(
        &mut self,
        input_m: Vec<Matrix>,
        target_m: Vec<Matrix>,
        batch_number: usize,
        epochs: usize,
    ) {
        // convert to Vec<Vec<f32>>
        let mut input = Vec::new();
        let mut target = Vec::new();
        for i in 0..input_m.len() {
            input.push(input_m[i].clone().to_vec());
            target.push(target_m[i].clone().to_vec());
        }
        self.train_epochs(input, target, batch_number, epochs);
    }

    /// Predict an output
    pub fn predict(&self, input: Vec<f32>) -> Vec<f32> {
        let nn = NeuralNetworkWorker {
            learning_rate: self.learning_rate,
            layer_sizes: &self.layer_sizes,
            weights: &self.weights,
            biases: &self.biases,
        };
        nn.predict(input)
    }
}
