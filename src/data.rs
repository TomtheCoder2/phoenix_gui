use crate::matrix::Matrix;

static DATA_FILE: &str = include_str!("resources/data3.txt");

/// * 0 - black
/// * 1 - white
/// * 2 - blue
/// * 3 - green
/// * 4 - yellow
/// * 5 - red
/// * 6 - nothing
pub fn get_data() -> (Vec<Matrix>, Vec<Matrix>, f32) {
    // format of data: 7x{ 100x{x, x, x, x} }
    let mut input: Vec<Matrix> = vec![];
    // these are just 7x1 matrices
    // for the first 100 targets its [1, 0, 0, 0, 0, 0, 0]
    // then for the next 100 its [0, 1, 0, 0, 0, 0, 0]
    // etc
    let mut targets: Vec<Matrix> = vec![];
    let mut index = 0;
    let data_string = DATA_FILE.replace('\n', "");
    let data_string = data_string.replace(' ', "");
    for _color_index in 0..7 {
        // println!("color_index: {}", color_index);
        // consume the first {
        index = consume(&data_string[index..], '{') + index + 1;
        for _ in 0..100 {
            // println!("i: {}", i);
            // println!("curr_code: {}", data_string[index..index + 10].to_string());
            // consume the first {
            index = consume(&data_string[index..], '{') + index + 1;
            // println!("curr_code: {}", data_string[index..index + 10].to_string());
            let mut matrix = Matrix::new(4, 1);
            for row in 0..4 {
                let col = 0;
                let start = index;
                index = consume(&data_string[index..], ',') + index;
                let mut end = index;
                if &data_string[end - 1..end] == "}" {
                    end -= 1;
                    index -= 2;
                }
                // println!("number: {}", data_string[start..end].to_string());
                let value = data_string[start..end].parse::<f32>().unwrap();
                matrix.set(row, col, value);
                index += 1;
            }
            // println!("curr_code: {}", data_string[index..index + 10].to_string());
            // consume the last }
            index = consume(&data_string[index..], '}') + index + 1;
            input.push(matrix);
        }
        // consume the last }
        index = consume(&data_string[index..], '}') + index + 1;
    }
    let max_value = input
        .iter()
        .flat_map(|matrix| matrix.data.iter())
        .fold(0.0, |acc, &x| if x > acc { x } else { acc });
    // println!("max_value: {}", max_value);
    for matrix in &mut input {
        for i in 0..matrix.data.len() {
            matrix.data[i] /= max_value / 2.0;
            matrix.data[i] -= 1.0;
        }
    }
    // create targets
    for color_index in 0..7 {
        for _ in 0..100 {
            let mut matrix = Matrix::new(7, 1);
            matrix.set(color_index, 0, 1.0);
            targets.push(matrix);
        }
    }

    (input, targets, max_value)
}

// returns the index of the next occurrence of c
#[allow(dead_code)]
fn consume(code: &str, c: char) -> usize {
    for (index, char) in code.chars().enumerate() {
        if char == c {
            return index;
        }
    }
    panic!("Could not find {} in {}", c, code);
}
