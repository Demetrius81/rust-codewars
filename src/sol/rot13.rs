#![allow(dead_code)]

use itertools::Itertools;

const SHIFT: u32 = 13;

pub fn run() {
    let a = "Test";
    // println!("> {}", "grfg");

    let rez = rot13(a);

    // println!("> {}", rez);
}

fn rot13(message: &str) -> String {
    message
        .chars()
        .map(|c| match c {
            'A'..='M' | 'a'..='m' => char::from_u32(c as u32 + SHIFT).unwrap(),
            'N'..='Z' | 'n'..='z' => char::from_u32(c as u32 - SHIFT).unwrap(),
            _ => c,
        })
        .join("")
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::rot13;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: &str, expected: &str) {
        assert_eq!(rot13(s), expected, "{ERR_MSG} with message = \"{s}\"")
    }

    #[test]
    fn sample_tests() {
        dotest("test", "grfg");
        dotest("Test", "Grfg");
    }
}
