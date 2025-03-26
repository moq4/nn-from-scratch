mod nn;

use crate::nn::matrix::*;
use crate::nn::activation_functions::*;
use crate::nn::layer::*;

fn main() {
    let mut m1 = Matrix::new(2, 2);
    m1.data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];

    let mut m2 = Matrix::new(2, 2);
    m2.data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];

    let add_result = m1.add(&m2);
    let sub_result = m1.subtract(&m2);

    println!("----Addition results----");
    add_result.print();
    println!("----Subtraction results----");
    sub_result.print();

    let mut m4 = Matrix::new(2, 3);
    m4.data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];

    let m_transposed = m4.transpose();
    let multiply_result = m4.multiply(&m_transposed);

    println!("----Transposed results----");
    m_transposed.print();
    println!("----Multiply results----");
    multiply_result.print();

    println!("----Test Sigmoid----");
    let mut m_sigmoid = Matrix::new(2, 2);
    m_sigmoid.data = vec![vec![0.5, -1.0], vec![2.0, -3.0]];

    let activated = m_sigmoid.apply(sigmoid);
    activated.print();

    println!("----Test Relu----");
    let mut m_relu = Matrix::new(2, 2);
    m_relu.data = vec![vec![0.5, -1.0], vec![2.0, -3.0]];

    let activated = m_relu.apply(relu);
    activated.print();

    println!("----Test Forward Pass----");
    let mut m_forward_pass = Matrix::new(2, 1);
    m_relu.data = vec![vec![0.1], vec![0.8]];
    let layer = Layer::new(2, 1);

    let foward_pass_result = layer.forward(&m_forward_pass);
    foward_pass_result.print();
}
