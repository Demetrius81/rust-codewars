#![allow(dead_code)]

pub fn run() {
    let a = [1, 2, 0, 1, 0, 1, 0, 3, 0, 1];
    let rez = move_zeros(&a);
    println!("{:?}", a);
    println!("{rez:?}");
}

fn move_zeros(arr: &[u8]) -> Vec<u8> {
    arr.iter()
        .copied()
        .filter(|&x| x > 0)
        .chain(vec![0; arr.len()])
        .take(arr.len())
        .collect::<Vec<u8>>()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::move_zeros;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[u8], expected: &[u8]) {
        let actual = move_zeros(a);
        assert!(
            actual == expected,
            "With arr = {a:?}\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest(
            &[1, 2, 0, 1, 0, 1, 0, 3, 0, 1],
            &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0],
        );
        dotest(
            &[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9],
            &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );
        dotest(&[0, 0], &[0, 0]);
        dotest(&[0], &[0]);
        dotest(&[], &[]);
    }
}
