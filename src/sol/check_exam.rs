#![allow(dead_code)]

pub fn run() {
    let rez = check_exam(&["a", "a", "b", "b"], &["a", "c", "b", "d"]);

    println!("{rez:?}");
}

fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    match arr_a
        .iter()
        .zip(arr_b)
        .map(|(a, b)| match *b {
            answ if answ == *a => 4i64,
            answ if answ == "" => 0i64,
            _ => -1i64,
        })
        .sum()
    {
        res if res > 0 => res,
        _ => 0i64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(check_exam(&["a", "a", "b", "b"], &["a", "c", "b", "d"]), 6);
        assert_eq!(check_exam(&["a", "a", "c", "b"], &["a", "a", "b", ""]), 7);
        assert_eq!(check_exam(&["a", "a", "b", "c"], &["a", "a", "b", "c"]), 16);
        assert_eq!(check_exam(&["b", "c", "b", "a"], &["", "a", "a", "c"]), 0);
    }
}
