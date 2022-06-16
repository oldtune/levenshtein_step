mod matrix;
use matrix::Matrix;

fn main() {
    let a = String::from("honda");
    let b = String::from("hyundai");
    let matrix = Matrix::new(a, b);
    matrix.print();
}
