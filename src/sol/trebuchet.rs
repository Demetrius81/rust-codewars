#[allow(dead_code)]
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run() {
    let rez = match read_file_to_vec("./calibration_data.txt") {
        Ok(x) => Some(get_vec_from_page(x).into_iter().sum::<i32>()),
        Err(_) => None,
    };

    println!("Result: {:?}", rez.unwrap());
}

fn get_vec_from_page(lines: Vec<String>) -> Vec<i32> {
    lines
        .into_par_iter()
        .filter_map(|s| {
            if let Some(num) = get_first_last_digit(&s) {
                Some(num)
            } else {
                None
            }
        })
        .collect()
}

fn read_file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;

    Ok(lines)
}

fn get_first_last_digit(s: &str) -> Option<i32> {
    let digits: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.is_empty() {
        None
    } else {
        Some(
            format!("{}{}", digits[0], digits.last().unwrap())
                .parse()
                .unwrap(),
        )
    }
}
