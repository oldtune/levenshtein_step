mod matrix;
use matrix::Matrix;

use crate::matrix::find_minimum_distance;

fn main() {
    let string_a = "ha".to_string();
    let string_b = "ha".to_string();

    let distance = find_minimum_distance(&string_a, &string_b);
    println!("{}", distance);
}
