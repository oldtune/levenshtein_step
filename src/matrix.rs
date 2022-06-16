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

    fn fill_matrix(&self) {}

    fn calculate_distance(&self, i: usize, j: usize, a: Vec<usize>, b: Vec<usize>) -> usize {
        if a[i] == 0 {
            return b[j];
        } else if b[j] == 0 {
            return a[i];
        } else {
            let insertion = self.matrix[i][j - 1] + 1;
            let deletion = self.matrix[i - 1][j] + 1;
            let replacement = match a[i] == b[j] {
                true => self.matrix[i - 1][j - 1],
                false => self.matrix[i - 1][j - 1] + 1,
            };

            return Self::min_of_3(insertion, deletion, replacement);
        }
    }

    fn min_of_3(first: usize, second: usize, last: usize) -> usize {
        if first < second && first < last {
            return first;
        }

        if second < first && second < last {
            return second;
        }

        last
    }

    //for debugging purpose
    pub fn print(&self) {
        for child in &self.matrix {
            println!("{:?}", child);
        }
    }
}
