#![allow(dead_code)]

pub fn run() {
    let a = 3599u32;
    let rez = make_readable(a);
    println!("{:?}", a);
    println!("00:59:59");
    println!("{rez}");
}

fn make_readable(seconds: u32) -> String {
    format!(
        "{2:02.0}:{1:02.0}:{0:02.0}",
        seconds % 60,
        (seconds / 60) % 60,
        seconds / 3600
    )
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::make_readable;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: u32, expected: &str) {
        assert_eq!(make_readable(s), expected, "{ERR_MSG} with seconds = {s}")
    }

    #[test]
    fn fixed_tests() {
        dotest(0, "00:00:00");
        dotest(59, "00:00:59");
        dotest(60, "00:01:00");
        dotest(3599, "00:59:59");
        dotest(3600, "01:00:00");
        dotest(86399, "23:59:59");
        dotest(86400, "24:00:00");
        dotest(359999, "99:59:59");
    }
}
