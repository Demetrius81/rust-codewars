#![allow(dead_code)]

pub fn run() {
    let rez = tower_builder(3, (4, 2));
    for i in rez {
        println!("> {i}");
    }
}

fn tower_builder(n_floors: usize, block_size: (usize, usize)) -> Vec<String> {
    // let (block_width, block_height) = block_size;
    // let total_width = (2 * n_floors - 1) * block_size.0;

    (1..=n_floors)
        .map(|i| {
            (0..block_size.1)
                .map(|_| {
                    format!(
                        "{:^width$}",
                        "*".repeat((2 * i - 1) * block_size.0),
                        width = (2 * n_floors - 1) * block_size.0
                    )
                })
                .collect::<Vec<String>>()
        })
        .flatten()
        .collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(tower_builder(1, (1, 1)), vec!["*"]);
        assert_eq!(
            tower_builder(3, (4, 2)),
            vec![
                "        ****        ",
                "        ****        ",
                "    ************    ",
                "    ************    ",
                "********************",
                "********************"
            ]
        );
    }
}
