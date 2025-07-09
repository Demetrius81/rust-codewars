#![allow(dead_code)]

use Direction::*;

pub fn run() {
    let a = [North, West, South, East];
    let rez = dir_reduc(&a);
    println!("{:?}", a);
    println!("{:?}", [West]);
    println!("{rez:?}");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut stack = vec![];

    arr.iter().for_each(|&d| match d {
        North => {
            if let Some(&last) = stack.last() {
                if last == South {
                    _ = stack.pop();
                } else {
                    stack.push(d);
                }
            } else {
                stack.push(d);
            }
        }
        East => {
            if let Some(&last) = stack.last() {
                if last == West {
                    _ = stack.pop();
                } else {
                    stack.push(d);
                }
            } else {
                stack.push(d);
            }
        }
        West => {
            if let Some(&last) = stack.last() {
                if last == East {
                    _ = stack.pop();
                } else {
                    stack.push(d);
                }
            } else {
                stack.push(d);
            }
        }
        South => {
            if let Some(&last) = stack.last() {
                if last == North {
                    _ = stack.pop();
                } else {
                    stack.push(d);
                }
            } else {
                stack.push(d);
            }
        }
    });
    
    stack
}

#[cfg(test)]
mod tests {
    use super::{Direction::*, dir_reduc};

    #[test]
    fn basic() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);

        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
    }
}
