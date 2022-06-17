use crate::min;
pub struct Matrix {
    a: String,
    b: String,
    matrix: Vec<Vec<usize>>,
}

pub fn init_matrix(row: usize, col: usize) -> Vec<Vec<usize>> {
    let mut vec = vec![];

    for i in 0..=row {
        let mut child_vec: Vec<usize> = vec![];
        for j in 0..=col {
            let first_row = i == 0;
            if first_row {
                child_vec.push(j);
            } else {
                let first_col = j == 0;
                if first_col {
                    child_vec.push(i);
                } else {
                    child_vec.push(0);
                }
            }
        }
        vec.push(child_vec);
    }
    // print!("{:?}", vec);
    vec
}

pub fn find_minimum_distance(a: &String, b: &String) -> usize {
    //pre-condition
    if a.len() == 0 {
        return b.len();
    }

    if b.len() == 0 {
        return a.len();
    }

    if a == b {
        return 0;
    }

    let mut d = init_matrix(a.len(), b.len());

    for (i_index, i) in a.chars().enumerate() {
        for (j_index, j) in b.chars().enumerate() {
            let i_index = i_index + 1;
            let j_index = j_index + 1;

            if i == j {
                d[i_index][j_index] = d[i_index - 1][j_index - 1];
            } else {
                d[i_index][j_index] = 1 + min!(
                    d[i_index][j_index - 1],
                    d[i_index - 1][j_index - 1],
                    d[i_index - 1][j_index]
                );
            }
        }
    }

    d[a.len()][b.len()]
}

fn print_matrix(matrix: &Vec<Vec<usize>>) {
    for row in matrix.iter() {
        println!("{:?}", row);
    }
}

#[macro_export]
macro_rules! min {
    ($x: expr) => {
       $x
    };
    ($x: expr, $($y: expr), +) => {
        std::cmp::min($x, min!($($y), +))
    }
}
