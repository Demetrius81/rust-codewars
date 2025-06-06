#![allow(dead_code)]

use std::collections::HashSet;

pub fn run() {
    let rez = compute_depth(25);
    println!("36 == {rez}");
}

fn compute_depth(n: u16) -> u8 {
    let mut digits = HashSet::new();
    let mut count = 0u16;

    while digits.len() < 10 {
        count += 1;
        (n * count).to_string().chars().for_each(|ch| {
            if ch.is_digit(10) {
                digits.insert(ch);
            }
        });
    }

    count as u8
}

fn compute_depth_2(n: u16) -> u8 {
    let mut digits = HashSet::new();
    (1..)
        .find(|count: &u16| {
            (n * count).to_string().chars().for_each(|ch| {
                digits.insert(ch);
            });
            digits.len() == 10
        })
        .unwrap() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(compute_depth(1), 10);
        assert_eq!(compute_depth(42), 9);
        assert_eq!(compute_depth(8), 12);
        assert_eq!(compute_depth(13), 8);
        assert_eq!(compute_depth(7), 10);
        assert_eq!(compute_depth(25), 36);
    }
}
