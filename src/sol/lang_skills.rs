#![allow(dead_code)]

use std::collections::HashMap;

pub fn run() {
    let rez = my_languages(HashMap::from([
        ("Hindi", 60),
        ("Greek", 71),
        ("Dutch", 93),
        ("C++", 50),
        ("ASM", 10),
        ("Haskell", 20),
    ]));

    println!("{rez:?}");
}

fn my_languages_2(results: HashMap<&str, i32>) -> Vec<&str> {
    let mut v: Vec<_> = results.iter().collect();
    v.sort_by(|(_, va), (_, vb)| va.cmp(vb));
    v.iter()
        .filter_map(|(k, v)| match **v {
            rez if rez >= 60 => Some(**k),
            _ => None,
        })
        .rev()
        .collect::<Vec<_>>()
    // vec![""]
}

#[cfg(test)]
mod tests {
    use super::my_languages;
    use std::collections::HashMap;

    fn do_test(dict: &HashMap<&str, i32>, expected: Vec<&str>) {
        let actual = my_languages(dict.clone());
        assert_eq!(
            actual, expected,
            "with dict {:?}, expected {:?}, got {:?}",
            &dict, expected, actual
        );
    }

    #[test]
    fn fixed_tests() {
        do_test(
            &HashMap::from([("Hindi", 60), ("Greek", 71), ("Dutch", 93)]),
            vec!["Dutch", "Greek", "Hindi"],
        );
        do_test(
            &HashMap::from([("C++", 50), ("ASM", 10), ("Haskell", 20)]),
            vec![],
        );
        do_test(
            &HashMap::from([
                ("Hindi", 60),
                ("Greek", 71),
                ("Dutch", 93),
                ("C++", 50),
                ("ASM", 10),
                ("Haskell", 20),
            ]),
            vec!["Dutch", "Greek", "Hindi"],
        );
    }

    #[test]
    fn empty() {
        do_test(&HashMap::new(), Vec::<&str>::new());
    }
}
