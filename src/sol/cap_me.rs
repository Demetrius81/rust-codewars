#![allow(dead_code)]

pub fn run() {
    let rez = cap_me(vec!["".to_string(), "".to_string(), "".to_string()]);

    println!("{rez:?}")
}

fn cap_me(arr: Vec<String>) -> Vec<String> {
    if arr.is_empty() {
        vec![]
    } else {
        arr.iter()
            .map(|x| {
                if x.is_empty() {
                    return x.to_owned();
                } else {
                    let mut v: Vec<char> = x.to_ascii_lowercase().chars().collect();
                    v[0] = v[0].to_uppercase().nth(0).unwrap();
                    v.into_iter().collect::<String>()
                }
            })
            .collect::<Vec<String>>()
    }
}

fn cap_me_2(arr: Vec<String>) -> Vec<String> {
    if arr.is_empty() {
        vec![]
    } else {
        arr.iter()
            .map(|x| {
                if x.is_empty() {
                    return x.to_owned();
                } else {
                    if x.len() > 1 {
                        format!("{}{}", x[0..1].to_uppercase(), x[1..].to_lowercase())
                    } else {
                        x.to_uppercase()
                    }
                }
            })
            .collect::<Vec<String>>()
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::cap_me;

    #[test]
    fn fixed_tests() {
        let test_cases = vec![
            (vec!["jo", "nelson", "jurie"], vec!["Jo", "Nelson", "Jurie"]),
            (vec!["OZZA", "ARRA", "AZZA"], vec!["Ozza", "Arra", "Azza"]),
            (vec![], vec![]),
            (vec!["", "", ""], vec!["", "", ""]),
        ];

        for (test_case, expected) in test_cases {
            let owned_test_case: Vec<String> = test_case.iter().map(|s| String::from(*s)).collect();
            let owned_expected: Vec<String> = expected.iter().map(|s| String::from(*s)).collect();

            assert_eq!(cap_me(owned_test_case), owned_expected);
        }
    }
}
