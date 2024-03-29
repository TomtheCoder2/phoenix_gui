use crate::matrix::Matrix;

pub mod blue_green;
pub mod mnist;

pub trait TestSet {
    // reads all the data from the file
    fn read(&mut self) {}
    // returns the inputs and targets
    fn get_data(&self) -> (Vec<Matrix>, Vec<Matrix>, f32) {
        (vec![], vec![], 0.0)
    }
}
