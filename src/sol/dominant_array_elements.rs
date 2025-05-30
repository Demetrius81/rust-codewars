#![allow(dead_code)]

pub fn run() {
    let rez = solve(&[16, 17, 14, 3, 14, 5, 2]);

    println!("{rez:?}");
}

fn solve(arr: &[u32]) -> Vec<u32> {
    let ints = arr.to_vec();
    ints.iter()
        .enumerate()
        .filter_map(|(i, x)| match i + 1 {
            idx if idx < ints.len() => match ints.iter().skip(idx).copied().max() {
                Some(rez) if rez < *x => Some(*x),
                _ => None,
            },
            idx if idx == ints.len() => Some(*x),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solve(&[16, 17, 14, 3, 14, 5, 2]), [17, 14, 5, 2]);
        assert_eq!(solve(&[92, 52, 93, 31, 89, 87, 77, 105]), [105]);
        assert_eq!(solve(&[75, 47, 42, 56, 13, 55]), [75, 56, 55]);
        assert_eq!(solve(&[67, 54, 27, 85, 66, 88, 31, 24, 49]), [88, 49]);
        assert_eq!(solve(&[76, 17, 25, 36, 29]), [76, 36, 29]);
        assert_eq!(solve(&[104, 18, 37, 9, 36, 47, 28]), [104, 47, 28]);
    }
}
