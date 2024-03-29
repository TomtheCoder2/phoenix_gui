use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;

use lz4_compression::compress::compress;

use phoenix_gui::data_sets::mnist::MNist;

fn main() {
    let mut data_set = MNist::default();
    data_set.read_files();
    let encoded: Vec<u8> = bincode::serialize(&data_set).unwrap();
    println!("Size before compression: {} bytes", encoded.len());
    let compressed_data = compress(&encoded);
    // let decompressed_data = decompress(&compressed_data).unwrap();
    println!("Size after compression: {} bytes", compressed_data.len());
    // write to file
    write_to_file(
        &"./src/resources/mnist_data_compressed.bin".to_string(),
        compressed_data,
    );
    write_to_file(&"./src/resources/mnist_data.bin".to_string(), encoded);
    // MNist::print_data(data_set.train_input);
}

pub fn write_to_file(filename: &String, content: Vec<u8>) {
    let path = Path::new(&filename);
    let path_display = path.display();

    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(why) => {
            eprintln!("Failed to create {}: {}", path_display, why);
            exit(1);
        }
    };

    match file.write_all(&content) {
        Ok(_) => {
            println!("Successfully wrote to {}", path_display);
        }
        Err(why) => {
            println!("Failed to write to {}: {}", path_display, why);
            exit(1);
        }
    };
}
