# Phoenix GUI

Phoenix GUI is a versatile graphical user interface application designed to simplify the process of parsing CSV file data, plotting mathematical functions, and exploring other advanced features such as neural networks and TCP data transmission using the `phoenix-rec` crate from crates.io.

## Features

- **CSV Data Parsing**: Effortlessly parse and visualize data from CSV files.
- **Plotting Functions**: Plot mathematical functions, such as \( y = x^2 + 1 \).
- **Neural Networks (WIP)**: Experimental neural network functionality.
- **TCP Data Transmission (WIP)**: Send data over TCP using the `phoenix-rec` crate.

## Installation

To install Phoenix GUI, ensure you have Rust and Cargo installed. Then, clone the repository and build the project:

```sh
git clone https://github.com/yourusername/phoenix-gui.git
cd phoenix-gui
cargo build
```

## Usage

### Running the Application

To start the application, run:

```sh
cargo run
```

### CSV Data Parsing

1. Open Phoenix GUI.
2. Navigate to File -> Open Folder.
3. Select a CSV file to parse.
4. View and analyze the parsed data in the interface.

### Plotting Functions

1. Open Phoenix GUI.
2. Navigate to the + icon.
3. Select the Plotter tab.
4. Enter a mathematical function (e.g., `x^2 + 1`).
5. Click "Compile" to visualize the function.

### Neural Networks (WIP)

1. Open Phoenix GUI.
2. Navigate to the + icon.
3. Select the Neural Networks tab.
4. Use the interface to experiment with neural networks (functionality may be limited).

### TCP Data Transmission (WIP)

1. Open Phoenix GUI.
2. Navigate to the + icon.
3. Select the TCP tab.
4. Configure the TCP settings and send data over TCP (functionality may be limited).

## Dependencies

Phoenix GUI relies on the following dependencies:

- [phoenix-rec](https://crates.io/crates/phoenix-rec) crate for TCP data transmission.
- Other dependencies as listed in `Cargo.toml`.

## Contributing

Contributions are welcome! Please fork the repository and submit pull requests.

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/YourFeature`).
3. Commit your changes (`git commit -am 'Add some feature'`).
4. Push to the branch (`git push origin feature/YourFeature`).
5. Create a new Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For any questions or suggestions, please open an issue or contact the project maintainer.