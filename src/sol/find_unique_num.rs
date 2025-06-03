#![allow(dead_code)]

pub fn run() {
    let rez = find_uniq(&[0.0, 1.0, 0.0]);

    println!("{rez:?}");

    let a = [1, 1, 1, 1, 1, 1].iter();
    let b = [2].iter();
    let r = a.partial_cmp(b);

    println!("> {:?}", r)
}

fn find_uniq_0(arr: &[f64]) -> f64 {
    if arr[0] != arr[1] {
        if arr[2] == arr[0] {
            return arr[1];
        } else {
            return arr[0];
        }
    }

    let reference = arr[0];
    let epsilon = 1e-12;

    for &x in arr {
        if (x - reference).abs() > epsilon {
            return x;
        }
    }

    reference
}

fn find_uniq(arr: &[f64]) -> f64 {
    if arr[0] != arr[1] {
        if arr[2] == arr[0] {
            return arr[1];
        } else {
            return arr[0];
        }
    }

    *arr.iter().find(|&&x| x != arr[0]).unwrap()
}

#[cfg(test)]
mod tests {
    use super::find_uniq;

    fn dotest(arr: &[f64], expected: f64) {
        assert_eq!(
            find_uniq(arr),
            expected,
            "\nleft is your output. right is expected output.\n input: `{:?}`\n",
            arr
        );
    }

    #[test]
    fn examples() {
        dotest(&[0.0, 1.0, 0.0], 1.0);
        dotest(&[1.0, 1.0, 1.0, 2.0, 1.0, 1.0], 2.0);
        dotest(&[3.0, 10.0, 3.0, 3.0, 3.0], 10.0);
    }
}
