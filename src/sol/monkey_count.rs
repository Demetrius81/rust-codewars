#![allow(dead_code)]

pub fn run() {
    let rez = monkey_count(15);
    println!("{rez:?}")
}

fn monkey_count(n: i32) -> Vec<i32> {
    (1..=n).collect()
}

fn monkey_count_2(n: i32) -> Vec<i32> {
    std::iter::successors(Some(1), |x| Some(x + 1))
        .take(n as usize)
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(monkey_count(5), vec![1, 2, 3, 4, 5]);
        assert_eq!(monkey_count(1), vec![1]);
        // assert_eq!(monkey_count(0), vec![]);
        assert_eq!(
            monkey_count(12),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
        );
    }
}
