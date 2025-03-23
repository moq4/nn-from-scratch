mod matrix;

use crate::matrix::matrix::Matrix;

fn main() {
    let m = Matrix::new(3, 3);
    m.print();
}
