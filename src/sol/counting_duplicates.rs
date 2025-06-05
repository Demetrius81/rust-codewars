#![allow(dead_code)]

use itertools::Itertools;
use std::collections::HashMap;

pub fn run() {
    let rez = count_duplicates("abcBea");
    println!("{:?}", rez)
}

fn count_duplicates(text: &str) -> u32 {
    let mut counts = HashMap::new();

    text.chars().for_each(|ch| {
        *counts.entry(ch.to_ascii_lowercase()).or_insert(0) += 1;
    });

    counts.values().filter(|&&count| count > 1).count() as u32
}

fn count_duplicates_2(text: &str) -> u32 {
    text.to_lowercase()
        .chars()
        .counts()
        .values()
        .filter(|&&i| i > 1)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}
