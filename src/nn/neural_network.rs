use crate::Layer;
use crate::Matrix;

pub struct NeuralNetwork {
    layers: Vec<Layer>,
}

impl NeuralNetwork {
    pub fn new(layer_sizes: Vec<usize>) -> NeuralNetwork {
        let mut layers = Vec::new();
        for i in 0..layer_sizes.len() - 1 {
            layers.push(Layer::new(layer_sizes[i], layer_sizes[i + 1]));
        }
        NeuralNetwork { layers }
    }

    pub fn forward(&self, input: &Matrix) -> Matrix {
        let mut output = input.clone();
        for layer in &self.layers {
            output = layer.forward(&output);
        }
        output
    }
}
