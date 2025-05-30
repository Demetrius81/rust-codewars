#[allow(dead_code)]

pub fn run() {
	let rez = two_sort(&["bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"]);
	println!("{rez}")
}


fn two_sort(arr: &[&str]) -> String {
	arr
			.iter()
			.min()
			.unwrap()
			.chars()
			.map(|c| c.to_string())
			.collect::<Vec<_>>().join("***")
}
	


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample_test_cases() {
			assert_eq!(two_sort(&["bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"]), "b***i***t***c***o***i***n");
			assert_eq!(two_sort(&["turns", "out", "random", "test", "cases", "are", "easier", "than", "writing", "out", "basic", "ones"]), "a***r***e");
	}
}