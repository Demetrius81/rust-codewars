#![allow(dead_code)]

pub fn run() {
    let rez = array_leaders(&[-1, -29, -26, -2]);

    println!("{rez:?}");
}

fn array_leaders(arr: &[i32]) -> Vec<i32> {
    let ints = arr.to_vec();
    ints.iter()
        .enumerate()
        .filter_map(|(i, x)| match i + 1 {
            idx if idx < ints.len() => match ints.iter().skip(idx).copied().sum::<i32>() {
                rez if rez < *x => Some(*x),
                _ => None,
            },
            idx if idx == ints.len() && *x > 0 => Some(*x),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Positive values
        assert_eq!(array_leaders(&[1, 2, 3, 4, 0]), [4]);
        assert_eq!(array_leaders(&[16, 17, 4, 3, 5, 2]), [17, 5, 2]);

        // Negative values
        assert_eq!(array_leaders(&[-1, -29, -26, -2]), [-1]);
        assert_eq!(array_leaders(&[-36, -12, -27]), [-36, -12]);

        // Mixed values
        assert_eq!(array_leaders(&[5, -2, 2]), [5, 2]);
        assert_eq!(array_leaders(&[0, -1, -29, 3, 2]), [0, -1, 3, 2]);
    }
}
