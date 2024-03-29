use phoenix_gui::matrix::Matrix;

use phoenix_gui::neural_network::NeuralNetwork;
use rand::Rng;

fn main() {
    let layer_sizes = vec![2, 400, 20, 4000, 3, 2];
    println!("layer_sizes: {:?}", layer_sizes);
    let mut weights: Vec<Matrix> = vec![];
    let mut biases: Vec<Matrix> = vec![];
    // let seed = 1234;
    let mut rng = rand::thread_rng();
    // rng.seed(seed);
    for i in 1..layer_sizes.len() {
        weights.push(Matrix::new(
            layer_sizes[i] as usize,
            layer_sizes[i - 1] as usize,
        ));
        // set the weights to random values
        for j in 0..weights[i - 1].rows {
            for k in 0..weights[i - 1].cols {
                weights[i - 1].set(j, k, rng.gen_range(-1.0..1.0));
            }
        }
        biases.push(Matrix::new(layer_sizes[i] as usize, 1));
        // set the biases to random values
        for j in 0..biases[i - 1].rows {
            for k in 0..biases[i - 1].cols {
                biases[i - 1].set(j, k, rng.gen_range(-1.0..1.0));
            }
        }
    }
    let mut nn = NeuralNetwork {
        weights: weights.clone(),
        biases: biases.clone(),
        command_receiver: None,
        command_sender: None,
        layer_sizes,
        learning_rate: 0.005,
        update_interval: 0,
    };
    // generate some random input and target data
    let mut rng = rand::thread_rng();
    let mut inputs: Vec<Vec<f32>> = vec![];
    let mut targets: Vec<Vec<f32>> = vec![];
    for _ in 0..1000 {
        let mut input: Vec<f32> = vec![];
        let mut target: Vec<f32> = vec![];
        for _ in 0..2 {
            input.push(rng.gen_range(0.0..1.0));
        }
        for _ in 0..2 {
            target.push(rng.gen_range(0.0..1.0));
        }
        inputs.push(input);
        targets.push(target);
    }
    // println!("inputs: {:?}", inputs);
    // println!("targets: {:?}", targets);
    // train the network
    let time1 = std::time::Instant::now();
    nn.train_epochs(inputs, targets, 100, 100);
    let time2 = std::time::Instant::now();
    println!(
        "training took {} ms ({} s)",
        (time2 - time1).as_millis(),
        (time2 - time1).as_secs()
    );
}
