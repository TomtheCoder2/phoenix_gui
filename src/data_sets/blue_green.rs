use crate::data::get_data;
use crate::matrix::Matrix;
use crate::neural_network::{NNConfig, NeuralNetwork};

pub fn run(config: NNConfig) {
    // * 0 - black
    // * 1 - white
    // * 2 - blue
    // * 3 - green
    // * 4 - yellow
    // * 5 - red
    // * 6 - nothing
    let (inputs, targets, _max_value) = get_data();
    // we only want blue and green
    let mut new_inputs = vec![];
    let mut new_targets = vec![];
    for i in 0..inputs.len() {
        // if targets[i].data[0] == 0.0
        //     && targets[i].data[3] == 0.0
        //     && targets[i].data[4] == 0.0
        //     && targets[i].data[5] == 0.0
        //     && targets[i].data[6] == 0.0
        // {
        new_inputs.push(inputs[i].clone());
        if targets[i].data[0] == 1.0 {
            new_targets.push(Matrix::from_vec(&vec![1.0, 0.0]));
        } else if targets[i].data[1] == 1.0 {
            new_targets.push(Matrix::from_vec(&vec![0.0, 1.0]));
        } else {
            panic!("targets[i]: {:?}", targets[i]);
        }
        // }
    }
    let inputs = new_inputs;
    let targets = new_targets;

    // println!("inputs: {:?}", inputs);
    // println!("targets: {:?}", targets);
    println!("input.len(): {:?}", inputs.len());
    println!("targets.len(): {:?}", targets.len());
    let mut nn = NeuralNetwork::new(config.layer_sizes);
    nn.learning_rate = config.learning_rate;
    if cfg!(feature = "gui") {
        nn.command_receiver = config.command_receiver;
        nn.command_sender = config.command_sender;
    }
    nn.update_interval = config.update_interval;
    nn.train_epochs_m(inputs, targets, config.batch_number, config.epochs + 1);
    // println!("predict the first input \n{:?}, \ntarget: \n{:?}", nn.predict(inputs[0].to_vec()), targets[0].to_vec());
    // println!("weights: {:?}", nn.weights);
    // check(&nn, inputs[0].to_vec(), targets[0].to_vec());
    // let mut correct = 0;
    // for i in 0..inputs.len() {
    //     if crate::check(&nn, inputs[i].to_vec(), targets[i].to_vec(), max_value) {
    //         correct += 1;
    //     }
    // }
    // println!(
    //     "correct: {}/{} -> {}%",
    //     correct,
    //     inputs.len(),
    //     correct as f32 / inputs.len() as f32 * 100.0
    // );
}
