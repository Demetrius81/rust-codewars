#![allow(dead_code)]

pub fn run() {
    let good_sudoku_1 = Sudoku {
        data: vec![
            vec![4, 1, 5, 3, 2, 6],
            vec![5, 3, 6, 2, 4, 1],
            vec![6, 1, 2, 4, 3, 5],
            vec![2, 1, 5, 4, 6, 3],
            vec![3, 5, 4, 6, 1, 2],
            vec![4, 6, 1, 2, 3, 5],
        ],
    };

    let rez = good_sudoku_1.is_valid();

    println!("> {:?}", rez);
}

struct Sudoku {
    data: Vec<Vec<u32>>,
}

impl Sudoku {
    fn is_valid(&self) -> bool {
        let size = self.data.len();
        if size > 0 {
            if self
                .data
                .iter()
                .all(|x| x.len() == size && x.iter().all(|&n| n <= size as u32 && n > 0))
            {
                if size > 3 {
                    if is_perfect_square(size) && !all_rows_equal(&self.data) {
                        return true;
                    }
                } else {
                    return true;
                }
            }
        }

        false
    }
}

fn all_rows_equal<T>(matrix: &Vec<Vec<T>>) -> bool
where
    T: PartialEq,
{
    if matrix.is_empty() {
        return true;
    }

    let first_row = &matrix[0];

    for row in matrix.iter().skip(1) {
        if row != first_row {
            return false;
        }
    }
    true
}

fn is_perfect_square(n: usize) -> bool {
    let sqrt = (n as f64).sqrt();
    let sqrt_int = sqrt as usize;
    sqrt_int * sqrt_int == n as usize
}

#[test]
fn good_sudoku() {
    let good_sudoku_1 = Sudoku {
        data: vec![
            vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
            vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
            vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
            vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
            vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
            vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
            vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
            vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
            vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
        ],
    };

    let good_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 4, 2, 3],
            vec![3, 2, 4, 1],
            vec![4, 1, 3, 2],
            vec![2, 3, 1, 4],
        ],
    };
    assert!(good_sudoku_1.is_valid());
    assert!(good_sudoku_2.is_valid());
}

#[test]
fn bad_sudoku() {
    let bad_sudoku_1 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        ],
    };

    let bad_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1],
        ],
    };
    assert!(!bad_sudoku_1.is_valid());
    assert!(!bad_sudoku_2.is_valid());
}
