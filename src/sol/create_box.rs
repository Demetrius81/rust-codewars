#[allow(dead_code)]

pub fn run() {
	let rez = create_box(7,	8);
	rez.iter().for_each(|vec| println!("{vec:?}"));
}

fn create_box(m: u32, n: u32) -> Vec<Vec<u32>> {
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
