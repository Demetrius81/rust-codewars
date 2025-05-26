#[allow(dead_code)]

pub fn run() {
    let rez = remove_every_other(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    // rez.iter().for_each(|vec| println!("{vec:?}"));
    println!("{rez:?}")
}

pub fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    arr.iter()
        .enumerate()
        .filter_map(|(i, v)| -> Option<u8> {
            if i % 2 != 0 { None } else { Some(*v) }
        })
        .collect()
}
pub fn remove_every_other_2(arr: &[u8]) -> Vec<u8> {
    arr.iter().step_by(2).copied().collect()
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::remove_every_other;

    #[test]
    fn sample_test() {
        assert_eq!(
            remove_every_other(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            &[1, 3, 5, 7, 9]
        );
    }
}
