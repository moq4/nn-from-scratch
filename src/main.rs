mod matrix;

use crate::matrix::matrix::Matrix;

fn main() {
    let mut m1 = Matrix::new(2, 2);
    m1.data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];

    let mut m2 = Matrix::new(2, 2);
    m2.data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];

    let add_result = m1.add(&m2);
    let sub_result = m1.subtract(&m2);

    println!("Addition results");
    add_result.print();

    println!("Subtraction results");
    sub_result.print();
}
