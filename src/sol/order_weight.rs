#![allow(dead_code)]

use itertools::Itertools;

pub fn run() {
    let rez = order_weight("2000 10003 1234000 44444444 9999 11 11 22 123");
    println!("Arrange:\t2000 10003 1234000 44444444 9999 11 11 22 123");
    println!("Act:\t\t11 11 2000 10003 22 123 1234000 44444444 9999");
    println!("Assert:\t\t{rez}");
}

fn order_weight(s: &str) -> String {
    s.split(" ")
        .sorted_by_key(|&x| (sum_of_digits(x), x))
        .join(" ")
}

fn sum_of_digits(n: &str) -> u64 {
    n.chars().map(|c| c.to_digit(10).unwrap() as u64).sum()
}

fn testing(s: &str, exp: &str) -> () {
    assert_eq!(order_weight(s), exp)
}

#[test]
fn basics_order_weight() {
    testing("103 123 4444 99 2000", "2000 103 123 4444 99");
    testing(
        "2000 10003 1234000 44444444 9999 11 11 22 123",
        "11 11 2000 10003 22 123 1234000 44444444 9999",
    );
}
