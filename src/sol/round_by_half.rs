#![allow(dead_code)]

use std::time::Instant;

use rand::Rng;

pub fn run() {
    let arr = generate_random_floats(100_000_000);
    let start = Instant::now();
    let _ = arr.iter().map(|&x| solution(x)).collect::<Vec<f64>>();
    let duration = start.elapsed();

    println!("time  solution: {:?}", duration);

    let start = Instant::now();
    let _ = arr.iter().map(|&x| solution2(x)).collect::<Vec<f64>>();
    let duration = start.elapsed();

    println!("time solution2: {:?}", duration);
}

fn generate_random_floats(length: usize) -> Vec<f64> {
    let mut rng = rand::rng();
    (0..length).map(|_| rng.random::<f64>()).collect()
}

fn solution(n: f64) -> f64 {
    n.floor()
        + match n.fract() {
            i if i < 0.25 => 0.0,
            i if i >= 0.25 && i < 0.75 => 0.5,
            i if i >= 0.75 => 1.0,
            _ => panic!("Something wrong!!!"),
        }
}

fn solution2(n: f64) -> f64 {
    (n * 2.0).round() / 2.0
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn sample_tests() {
        assert_eq!(solution(4.2), 4.0);
        assert_eq!(solution(4.4), 4.5);
        assert_eq!(solution(4.6), 4.5);
        assert_eq!(solution(4.8), 5.0);
    }
}
