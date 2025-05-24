#[allow(dead_code)]

pub fn run() {
	let rez = create_box(7,	8);
	println!("{rez:?}")
}

fn create_box(m: u32, n: u32) -> Vec<Vec<u32>> {
	// let mut result = Vec::with_capacity(n as usize);
    // for i in 0..n {
    //     let mut row = Vec::with_capacity(m as usize);
    //     for j in 0..m {
    //         // Вычисляем уровень слоя
    //         let layer = std::cmp::min(
    //             std::cmp::min(i, n - 1 - i),
    //             std::cmp::min(j, m - 1 - j)
    //         ) + 1;
    //         row.push(layer + 0); // Можно оставить так или просто row.push(layer);
    //     }
    //     result.push(row);
    // }
    // result

    (0..n)
        .map(|i| (0..m)
            .map(|j| std::cmp::min(
                std::cmp::min(i, n - 1 - i),
                std::cmp::min(j, m - 1 - j)
                ) + 1)
            .collect::<Vec<u32>>()
            )
        .collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::create_box;
    
    fn dotest(m: u32, n: u32, expected: &[&[u32]]) {
        let actual = create_box(m, n);
        assert!(actual == expected,
                "With m = {m}, n = {n}\nExpected {expected:?}\nBut got {actual:?}")
    }

    #[test]
    fn example_tests() {
        dotest(7, 8, &[&[1, 1, 1, 1, 1, 1, 1], 
                       &[1, 2, 2, 2, 2, 2, 1], 
                       &[1, 2, 3, 3, 3, 2, 1], 
                       &[1, 2, 3, 4, 3, 2, 1], 
                       &[1, 2, 3, 4, 3, 2, 1], 
                       &[1, 2, 3, 3, 3, 2, 1], 
                       &[1, 2, 2, 2, 2, 2, 1], 
                       &[1, 1, 1, 1, 1, 1, 1]]);
        dotest(6, 4, &[&[1, 1, 1, 1, 1, 1], 
                       &[1, 2, 2, 2, 2, 1], 
                       &[1, 2, 2, 2, 2, 1], 
                       &[1, 1, 1, 1, 1, 1]]);
    }
}
