#![allow(dead_code)]

pub fn run() {
    let rez = prime_bef_aft(100);

    println!("> {rez:?}");
}

fn prime_bef_aft(n: u32) -> (u32, u32) {
    (
        n - (1..).find(|v| is_prime(n - v)).unwrap(),
        n + (1..).find(|v| is_prime(n + v)).unwrap(),
    )
}

fn is_prime(n: u32) -> bool {
    (2..=(n as f32).sqrt() as u32).all(|v| n % v != 0)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::prime_bef_aft;

    #[test]
    fn sample_tests() {
        assert_eq!(prime_bef_aft(100), (97, 101));
        assert_eq!(prime_bef_aft(97), (89, 101));
        assert_eq!(prime_bef_aft(101), (97, 103));
        assert_eq!(prime_bef_aft(120), (113, 127));
        assert_eq!(prime_bef_aft(130), (127, 131));
    }
}
