use std::f64;

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn print(&self) {
        for row in &self.data {
            println!("{:?}", row);
        }
    }

    // Basic math functions

    pub fn add(&self, other: &Matrix) -> Matrix {
        assert!(
            self.rows == other.rows && self.cols == other.cols,
            "Matrix dimensions must match for addition"
        );

        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }

    pub fn subtract(&self, other: &Matrix) -> Matrix {
        assert!(
            self.rows == other.rows && self.cols == other.cols,
            "Matrix dimensions must match for subtraction"
        );

        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }

        result
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert!(
            self.cols == other.rows,
            "# of cols in first matrix must match # of rows in second"
        );

        let mut result = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }

    // Activation functions

    pub fn apply<F: Fn(f64) -> f64>(&self, func: F) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = func(self.data[i][j]);
            }
        }
        result
    }

    // Bias
    pub fn add_scalar(&self, scalar: f64) -> Matrix {
        self.apply(|x| x + scalar)
    }
}
