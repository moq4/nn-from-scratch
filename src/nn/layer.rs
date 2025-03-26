use crate::Matrix;

use super::activation_functions::sigmoid;

pub struct Layer {
    weights: Matrix,
    bias: Matrix,
}

impl Layer {
    pub fn new(input_size: usize, output_size: usize) -> Layer {
        let mut weights = Matrix::new(output_size, input_size);
        let bias = Matrix::new(output_size, 1);

        for row in &mut weights.data {
            for value in row {
                *value = 0.5; // ideally this would be random, just do 0.5 for now
            }
        }

        Layer { weights, bias }
    }

    pub fn forward(&self, input: &Matrix) -> Matrix {
        let z = self.weights.multiply(input).add_scalar(0.5);
        z.apply(sigmoid)
    }
}
