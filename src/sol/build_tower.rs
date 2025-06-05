#![allow(dead_code)]

pub fn run() {
    let rez = tower_builder(2);
    for i in rez {
        println!("> {i}");
    }
}

fn tower_builder(n_floors: usize) -> Vec<String> {
    (1..=n_floors)
        .map(|i| format!("{0}{1}{0}", " ".repeat(n_floors - i), "*".repeat(2 * i - 1)))
        .collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::tower_builder;

    #[test]
    fn fixed_tests() {
        assert_eq!(tower_builder(1), vec!["*"]);
        assert_eq!(tower_builder(2), vec![" * ", "***"]);
        assert_eq!(tower_builder(3), vec!["  *  ", " *** ", "*****"]);
    }
}
