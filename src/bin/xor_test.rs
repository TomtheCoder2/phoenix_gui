// #[cfg(not(feature = "nn"))]
// #[cfg(not(feature = "gui"))]
// use ai::matrix::Matrix;
// #[cfg(not(feature = "nn"))]
// #[cfg(not(feature = "gui"))]
// use ai::neural_network::NeuralNetwork;
//
// #[cfg(not(feature = "nn"))]
// #[cfg(not(feature = "gui"))]
// fn main() {
//     let inputs = vec![
//         vec![0.0, 0.0],
//         vec![0.0, 1.0],
//         vec![1.0, 0.0],
//         vec![1.0, 1.0],
//     ];
//     let targets = vec![
//         vec![0.0, 1.0],
//         vec![1.0, 0.0],
//         vec![1.0, 0.0],
//         vec![0.0, 1.0],
//     ];
//     let mut nn = NeuralNetwork::new(vec![2, 4, 2]);
//     nn.learning_rate = 1.5;
//     nn.train_epochs(inputs.clone(), targets.clone(), 16, 500);
//     // check the predictions
//     for i in 0..inputs.len() {
//         let prediction = nn.predict(inputs[i].to_vec());
//         println!(
//             "predict the input {:?}, target: {:?}, prediction: {:?}, error: {:?}",
//             inputs[i],
//             targets[i],
//             prediction,
//             Matrix::subtract_two(
//                 &Matrix::from_vec(&targets[i]),
//                 &Matrix::from_vec(&prediction)
//             )
//             .data
//         );
//     }
//     println!("weights: {:?}", nn.weights);
//     println!("biases: {:?}", nn.biases);
// }
//
// #[cfg(any(not(feature = "nn"), not(feature = "gui")))]
// fn main() {
//     println!("This example requires the nn and gui features to be enabled.");
// }

fn main() {
    println!("This example requires the nn and gui features to be enabled.");
}
