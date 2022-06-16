use crate::min;
pub struct Matrix {
    a: String,
    b: String,
    matrix: Vec<Vec<usize>>,
}

impl Matrix {
    pub fn new(a: String, b: String) -> Self {
        let matrix = Self::init_matrix(a.len(), b.len());
        Self { a, b, matrix }
    }

    fn init_matrix(row: usize, col: usize) -> Vec<Vec<usize>> {
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

    // fn fill_the_matrix(&self) {
    //     for i in 1..self.a.len() {
    //         for j in 1..self.b.len() {
    //             self.calculate_distance(
    //                 i,
    //                 j,
    //                 self.a.chars().into_iter().collect(),
    //                 self.b.chars().into_iter().collect(),
    //             );
    //         }
    //     }
    // }

    // fn calculate_distance(&self, i: usize, j: usize, a: Vec<char>, b: Vec<char>) -> usize {}

    //for debugging purpose
    pub fn print(&self) {
        for child in &self.matrix {
            println!("{:?}", child);
        }
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
