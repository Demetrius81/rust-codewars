#![allow(dead_code)]

use itertools::Itertools;

pub fn run() {
    let a = "test1234";

    let rez = increment_string(a);

    println!("> {}", rez);
}

fn increment_string(s: &str) -> String {
    let mut digits: Vec<char> = vec![];
    for c in s.chars().rev() {
        if c.is_digit(10) {
            digits.push(c);
        } else {
            break;
        }
    }

    digits.reverse();

    let alphabetic = &s[0..s.len() - digits.len()];

    let num: String;

    if digits.len() > 0 {
        if let Ok(n) = (digits.iter().join("")).parse::<i64>() {
            num = format!("{:0width$}", n + 1, width = digits.len());
        } else {
            increment_large_number(&mut digits);
            num = digits.iter().join("");
        }
    } else {
        num = "1".to_string();
    }

    format!("{}{}", alphabetic, num)
}

fn increment_large_number(digits: &mut Vec<char>) {
    let mut carry = 1;

    for i in (0..digits.len()).rev() {
        if digits[i] == '9' && carry == 1 {
            digits[i] = '0';
        } else {
            if carry == 1 {
                let digit_value = digits[i].to_digit(10).unwrap() + 1;
                digits[i] = std::char::from_digit(digit_value, 10).unwrap();
                carry = 0;
                break;
            }
        }
    }

    if carry == 1 {
        digits.insert(0, '1');
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::increment_string;

    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert!(
            actual == expected,
            "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\""
        )
    }

    #[test]
    fn sample_tests() {
        dotest("foo", "foo1");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
        dotest("", "1");
    }
}
