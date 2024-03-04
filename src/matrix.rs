//! Matrix operations
use crate::SEED;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};
use unroll::unroll_for_loops;

/// Represents a Matrix
#[derive(Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Matrix {
    /// Number of rows
    pub rows: usize,
    /// Number of columns
    pub cols: usize,
    /// Actual data of the matrix (row-major order)
    pub data: Vec<f32>,
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // format:
        // ==== Matrix { rows, cols } ====
        // [ [ 1, 2, 3 ],
        //   [ 4, 5, 6 ],
        //   [ 7, 8, 9 ] ]
        writeln!(
            f,
            "==== Matrix {{ rows: {}, cols: {} }} ====",
            self.rows, self.cols
        )?;
        writeln!(f, "[")?;
        for row in 0..self.rows {
            write!(f, "  [ ")?;
            for col in 0..self.cols {
                write!(f, "{}, ", self.get(row, col))?;
            }
            writeln!(f, "],")?;
        }
        write!(f, "]")
    }
}

impl Matrix {
    /// Create a matrix with given size and fill it 0's
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut r = StdRng::seed_from_u64(SEED);
        let mut m = Matrix::new(rows, cols);
        for row in 0..rows {
            for col in 0..cols {
                m.set(row, col, r.gen_range(-1.0..1.0));
            }
        }
        m
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, val: f32) {
        self.data[row * self.cols + col] = val;
    }

    /// Add a scalar to every element
    pub fn add_f32(&mut self, scaler: f32) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.set(row, col, self.get(row, col) + scaler);
            }
        }
    }
    /// Add a Matrix to `self`
    pub fn add_matrix(&mut self, m: &Matrix) {
        if self.rows != m.rows || self.cols != m.cols {
            panic!(
                "Matrix add: matrices have different dimensions: {}x{} vs {}x{}",
                self.rows, self.cols, m.rows, m.cols
            );
        }
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.set(row, col, self.get(row, col) + m.get(row, col));
            }
        }
    }
    /// Subtract `m2` from `m1`
    /// <br>
    /// `C` = `M1` - `M2`
    pub fn subtract_two(m1: &Matrix, m2: &Matrix) -> Matrix {
        if m1.rows != m2.rows || m1.cols != m2.cols {
            panic!("Matrix subtract: matrices have different dimensions");
        }
        let mut m = Matrix::new(m1.rows, m1.cols);
        for row in 0..m1.rows {
            for col in 0..m1.cols {
                m.set(row, col, m1.get(row, col) - m2.get(row, col));
            }
        }
        m
    }

    /// Transpose a given matrix
    pub fn transpose(m1: &Matrix) -> Matrix {
        let mut m: Matrix = Matrix::new(m1.cols, m1.rows);
        for row in 0..m1.rows {
            for col in 0..m1.cols {
                m.set(col, row, m1.get(row, col));
            }
        }
        m
    }

    /// Multiply the two matrices
    #[unroll_for_loops]
    pub fn multiply_two_normal(m1: &Matrix, m2: &Matrix) -> Matrix {
        if m1.cols != m2.rows {
            panic!("Matrix multiply: matrices have different dimensions");
        }
        let mut m = Matrix::new(m1.rows, m2.cols);
        for row in 0..m1.rows {
            for col in 0..m2.cols {
                let mut sum: f32 = 0.0;
                for i in 0..m1.cols {
                    sum += m1.get(row, i) * m2.get(i, col);
                }
                m.set(row, col, sum);
            }
        }
        m
    }

    #[cfg(not(feature = "blas"))]
    /// Multiply the two matrices
    pub fn multiply_two(m1: &Matrix, m2: &Matrix) -> Matrix {
        Matrix::multiply_two_normal(m1, m2)
    }

    #[cfg(feature = "blas")]
    /// Multiply the two matrices
    pub fn multiply_two(m1: &Matrix, m2: &Matrix) -> Matrix {
        Matrix::multiply_two_blas(m1, m2)
    }

    // pub fn multiply_two_blas(m1: Matrix, m2: Matrix) -> Matrix {
    //     if m1.cols != m2.rows {
    //         panic!("Matrix multiply: matrices have different dimensions");
    //     }
    //     let mut m = Matrix::new(m1.rows, m2.cols);
    //     // first convert to rustml::Matrix<f32>
    //     let m1_rustml: rustml::Matrix<f32> = rustml::Matrix::from_vec(m1.data.clone().into_iter().flatten().collect(), m1.rows, m1.cols);
    //     let m2_rustml: rustml::Matrix<f32> = rustml::Matrix::from_vec(m2.data.clone().into_iter().flatten().collect(), m2.rows, m2.cols);
    //     // then multiply
    //     let m_rustml = m1_rustml * m2_rustml;
    //     // then convert back to Matrix
    //     m.data = m_rustml.buf().chunks(m.cols).map(|x| x.to_vec()).collect();
    //     m
    // }

    #[cfg(feature = "blas")]
    /// New optimized function for multiplying to matrices
    pub fn multiply_two_blas(m1: &Matrix, m2: &Matrix) -> Matrix {
        use matrixmultiply::sgemm;
        if m1.cols != m2.rows {
            panic!("Matrix multiply: matrices have different dimensions");
        }
        let mut result = Matrix::new(m1.rows, m2.cols);

        // doc of the blas function:
        // General matrix multiplication (f32)
        //
        // C ← α A B + β C
        //
        // + m, k, n: dimensions
        // + a, b, c: pointer to the first element in the matrix
        // + A: m by k matrix
        // + B: k by n matrix
        // + C: m by n matrix
        // + rs<em>x</em>: row stride of *x*
        // + cs<em>x</em>: col stride of *x*
        //
        // Strides for A and B may be arbitrary. Strides for C must not result in
        // elements that alias each other, for example they can not be zero.
        //
        // If β is zero, then C does not need to be initialized.
        // pub unsafe fn sgemm(
        //     m: usize,
        //     k: usize,
        //     n: usize,
        //     alpha: f32,
        //     a: *const f32,
        //     rsa: isize,
        //     csa: isize,
        //     b: *const f32,
        //     rsb: isize,
        //     csb: isize,
        //     beta: f32,
        //     c: *mut f32,
        //     rsc: isize,
        //     csc: isize
        // );

        // for us beta is always 0 and alpha is always 1
        // first we need to convert the matrices to arrays, we can do this by just flattening them
        let a = &m1.data;
        let b = &m2.data;
        let c = &mut result.data;
        // then we need to get the dimensions
        let m = m1.rows;
        let k = m1.cols;
        let n = m2.cols;
        // then we need to get the strides
        let rs_a = k;
        let cs_a = 1;
        let rs_b = n;
        let cs_b = 1;
        let rs_c = n;
        let cs_c = 1;
        // then we can call the blas function
        unsafe {
            sgemm(
                m,
                k,
                n,
                1.0,
                a.as_ptr(),
                rs_a as isize,
                cs_a,
                b.as_ptr(),
                rs_b as isize,
                cs_b,
                0.0,
                c.as_mut_ptr(),
                rs_c as isize,
                cs_c,
            );
        }
        // // then we need to convert the array back to a matrix
        // let mut result = Matrix::new(m, n);
        // for row in 0..m {
        //     for col in 0..n {
        //         result.set(row, col, c[row * n + col]);
        //     }
        // }

        result
    }

    /// Multiply `self` by another matrix<br>
    /// **IMPORTANT:** This function just multiplies each element with the other element, for "real" matrix multiplication use [fn@multiply_two]
    pub fn multiply_with_matrix(&mut self, m: &Matrix) {
        if self.cols != m.cols || self.rows != m.rows {
            panic!("Matrix multiply: matrices have different dimensions");
        }
        for row in 0..self.rows {
            for col in 0..m.cols {
                self.set(row, col, self.get(row, col) * m.get(row, col));
            }
        }
    }
    /// Multiply each element by a `f32`
    pub fn multiply_with_f32(&mut self, d: f32) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.set(row, col, self.get(row, col) * d);
            }
        }
    }
    /// Apply the sigmoid function to each element<br>
    /// `sigmoid(x)` = `1/(1 + e^x)`
    pub fn sigmoid(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.set(row, col, 1.0 / (1.0 + (-self.get(row, col)).exp()));
            }
        }
    }
    /// Apply the derivative sigmoid function on all elements
    /// `x = x * (1 - x)`
    pub fn sigmoid_derivative(&mut self) -> Matrix {
        let mut m = Matrix::new(self.rows, self.cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                m.set(row, col, self.get(row, col) * (1.0 - self.get(row, col)));
            }
        }
        m
    }
    /// Convert an array to a matrix with size: arr.len(), 1
    pub fn from_vec(arr: &Vec<f32>) -> Matrix {
        let mut m = Matrix::new(arr.len(), 1);
        for (i, item) in arr.iter().enumerate() {
            m.set(i, 0, *item);
        }
        m
    }

    /// Convert a 2d array to a matrix
    pub fn from_2d_vec(arr: &Vec<Vec<f32>>) -> Matrix {
        let mut m: Matrix = Matrix::new(arr.len(), arr[0].len());
        for (i, item) in arr.iter().enumerate() {
            for (j, jtem) in item.iter().enumerate() {
                m.set(i, j, *jtem);
            }
        }
        m
    }

    /// Convert a Matrix to a 1d array, row major (i think, idk lol)
    pub fn to_vec(&self) -> Vec<f32> {
        let mut arr = vec![];
        for i in 0..self.rows {
            for j in 0..self.cols {
                arr.push(self.get(i, j));
            }
        }
        arr
    }

    pub fn to_2d_vec(&self) -> Vec<Vec<f32>> {
        let mut arr = vec![];
        for i in 0..self.rows {
            let mut row = vec![];
            for j in 0..self.cols {
                row.push(self.get(i, j));
            }
            arr.push(row);
        }
        arr
    }

    /// Get the average of all values
    pub fn average(&self) -> f32 {
        let mut sum = 0.0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                sum += self.get(i, j);
            }
        }
        sum / (self.rows * self.cols) as f32
    }

    /// Check if the matrix contains any NaN values
    /// If yes, return true, else false
    pub fn contains_nan(&self) -> bool {
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.get(i, j).is_nan() {
                    return true;
                }
            }
        }
        false
    }

    /// Gets the largest value in the matrix
    pub fn max(&self) -> f32 {
        let mut max = self.get(0, 0);
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.get(i, j) > max {
                    max = self.get(i, j);
                }
            }
        }
        max
    }
}
