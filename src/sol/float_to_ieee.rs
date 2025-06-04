#![allow(dead_code)]

pub fn run() {
    let rez = f32_to_ieee_754(15.875);
    println!("> {rez:?}");

    let rez = f64_to_ieee_754(15.875);
    println!("> {rez:?}");

    let a = 265.365656564;
    print!("> {a} - {a:#e}")
}

fn f32_to_ieee_754(num: f32) -> String {
    let bytes = num.to_be_bytes();
    let bits: String = bytes.iter().map(|byte| format!("{:08b}", byte)).collect();

    let sign = &bits[0..1];
    let exponent = &bits[1..9];
    let mantissa = &bits[9..];

    format!("{} {} {}", sign, exponent, mantissa)
}

fn f64_to_ieee_754(num: f64) -> String {
    let bytes = num.to_be_bytes();
    let bits: String = bytes.iter().map(|byte| format!("{:08b}", byte)).collect();

    let sign = &bits[0..1];
    let exponent = &bits[1..12];
    let mantissa = &bits[12..];

    format!("{} {} {}", sign, exponent, mantissa)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_f32() {
        assert_eq!(
            f32_to_ieee_754(15.875),
            "0 10000010 11111100000000000000000"
        );
        assert_eq!(f32_to_ieee_754(-1.0), "1 01111111 00000000000000000000000");
    }

    #[test]
    fn test_sample_f64() {
        assert_eq!(
            f64_to_ieee_754(15.875),
            "0 10000000010 1111110000000000000000000000000000000000000000000000"
        );
        assert_eq!(
            f64_to_ieee_754(-1.0),
            "1 01111111111 0000000000000000000000000000000000000000000000000000"
        );
    }
}
