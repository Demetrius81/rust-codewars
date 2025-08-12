#[allow(dead_code)]
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[allow(dead_code)]

pub fn run() {
    let rez = match read_file_to_vec("./calibration_data.txt") {
        // Ok(x) => Some(get_first_last_digit(&create_digit_string(&x[2]))),
        Ok(x) => Some(get_vec_from_page(x).into_iter().sum::<i32>()),
        Err(_) => None,
    };

    println!("Result: {:?}", rez.unwrap());
}

fn get_vec_from_page(lines: Vec<String>) -> Vec<i32> {
    lines
        // .into_par_iter()
        .into_iter()
        .filter_map(|s| {
            if let Some(num) = get_first_last_digit(&create_digit_string(&s)) {
                _ = write_to_file(
                    "output.txt",
                    &create_digit_string(&s)
                        .chars()
                        .filter(|c| c.is_ascii_digit())
                        .collect::<String>(),
                );
                Some(num)
            } else {
                println!("No digits in line");
                None
            }
        })
        .collect()
}

fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;

    writeln!(file, "{}", content)?;
    Ok(())
}

fn read_file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;

    Ok(lines)
}

fn create_digit_string(s: &str) -> String {
    let mut map = HashMap::new();
    map.insert("zero", 0);
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);

    let mut s = s.to_lowercase();

    for (word, &num) in &map {
        while s.contains(word) {
            s = s.replace(word, &num.to_string());
        }
    }

    s
}

fn get_first_last_digit(s: &str) -> Option<i32> {
    let digits: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.is_empty() {
        println!("No digits in line");
        None
    } else {
        Some(
            format!("{}{}", digits[0], digits.last().unwrap())
                .parse()
                .unwrap(),
        )
    }
}
