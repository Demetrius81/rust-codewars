#![allow(dead_code)]

pub fn run() {
    let rez = is_valid_walk(&['n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's']);

    println!("> {rez:?}");
}

fn is_valid_walk(walk: &[char]) -> bool {
    walk.len() == 10
        && walk.iter().fold((0, 0), |(y, x), dir| match dir {
            'n' => (y + 1, x),
            's' => (y - 1, x),
            'w' => (y, x + 1),
            'e' => (y, x - 1),
            _ => panic!("Error!"),
        }) == (0, 0)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::is_valid_walk;

    #[test]
    fn sample_tests() {
        assert!(is_valid_walk(&[
            'n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
        ]));
        assert!(!is_valid_walk(&[
            'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e'
        ]));
        assert!(!is_valid_walk(&['w']));
        assert!(!is_valid_walk(&[
            'n', 'n', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
        ]));
        assert!(!is_valid_walk(&[
            'e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's'
        ]))
    }
}
