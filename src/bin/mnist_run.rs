use phoenix_gui::data_sets::mnist::MNist;
use phoenix_gui::neural_network::NNConfig;

fn main() {
    let config = NNConfig {
        learning_rate: 0.0015,
        epochs: 100,
        layer_sizes: vec![784, 1100, 80, 10],
        batch_number: 100,
        command_receiver: None,
        command_sender: None,
        update_interval: 2,
    };
    let mut mnist = MNist::default();
    mnist.run(config);
}