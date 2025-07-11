#![allow(dead_code)]

// ======================
// |   Density Chart    |
// ======================
// | Honey   | H | 1.36 |
// | Water   | W | 1.00 |
// | Alcohol | A | 0.87 |
// | Oil     | O | 0.80 |
// ----------------------

use std::cmp::Ordering;
use itertools::Itertools;

pub fn run() {
    let a = [
        vec!['H', 'H', 'W', 'O'],
        vec!['W', 'W', 'O', 'W'],
        vec!['H', 'H', 'O', 'O'],
    ];

    let rez = separate_liquids(&a);

    println!("> {:?}", rez);
}

fn separate_liquids(glass: &[Vec<char>]) -> Vec<Vec<char>> {
    if glass.len() == 0 {
        return vec![];
    }

    let width = glass[0].len();
    let mut res: Vec<Vec<char>> = vec![];

    let a: Vec<char> = glass
        .iter()
        .cloned()
        .flatten()
        .sorted_by(|a, b| compare(a, b))
        .collect();

    if glass.len() == 1 {
        res.push(a);
    } else {
        let mut temp: &[char];
        let mut rest: &[char] = &a;

        for i in 1..glass.len() {
            (temp, rest) = rest.split_at(width);
            res.push(temp.iter().cloned().collect::<Vec<char>>());
            if i == glass.len() - 1 {
                res.push(rest.iter().cloned().collect::<Vec<char>>());
            }
        }
    }

    res
}

fn separate_liquids_n(glass: &[Vec<char>]) -> Vec<Vec<char>> {
    glass
        .iter()
        .flatten()
        .sorted_by(|&a, &b| compare(a, b))
        .chunks(if glass.is_empty() {1} else {glass[0].len()})
        .into_iter()
        .map(|c| c.copied().collect())
        .collect()
}

fn translate(a: &char) -> f64 {
    match a {
        'H' => 1.36,
        'W' => 1.00,
        'A' => 0.87,
        'O' => 0.80,
        _ => panic!("Wrong!!!"),
    }
}

fn compare(a: &char, b: &char) -> Ordering {
    let cmp = translate(a) - translate(b);

    if cmp == 0.0 {
        Ordering::Equal
    } else if cmp < 0.0 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::separate_liquids;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[Vec<char>], expected: &[Vec<char>]) {
        assert_eq!(
            separate_liquids(a),
            expected,
            "{ERR_MSG} with glass = {a:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(
            &[
                vec!['H', 'H', 'W', 'O'],
                vec!['W', 'W', 'O', 'W'],
                vec!['H', 'H', 'O', 'O'],
            ],
            &[
                vec!['O', 'O', 'O', 'O'],
                vec!['W', 'W', 'W', 'W'],
                vec!['H', 'H', 'H', 'H'],
            ],
        );
        dotest(
            &[
                vec!['A', 'A', 'O', 'H'],
                vec!['A', 'H', 'W', 'O'],
                vec!['W', 'W', 'A', 'W'],
                vec!['H', 'H', 'O', 'O'],
            ],
            &[
                vec!['O', 'O', 'O', 'O'],
                vec!['A', 'A', 'A', 'A'],
                vec!['W', 'W', 'W', 'W'],
                vec!['H', 'H', 'H', 'H'],
            ],
        );
        dotest(&[vec!['A', 'H', 'W', 'O']], &[vec!['O', 'A', 'W', 'H']]);
        dotest(
            &[vec!['A'], vec!['H'], vec!['W'], vec!['O']],
            &[vec!['O'], vec!['A'], vec!['W'], vec!['H']],
        );
        dotest(&[], &[]);
    }
}
