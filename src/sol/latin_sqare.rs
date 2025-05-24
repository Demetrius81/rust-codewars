#[allow(dead_code)]

pub fn run() {
	let rez = make_latin_square(4);
	rez.iter().for_each(|vec| println!("{vec:?}"));
}

pub fn make_latin_square(n: i32) -> Vec<Vec<i32>> {
	(1..=n).map(|i| (i..=n).chain(1..i).collect()).collect()
}

pub fn make_latin_square_unsafe(n: i32) -> Vec<Vec<i32>> {
	if n == 0 {
		return vec![];
	}
	if n == 1 {
		return vec![vec![1]];
	}
	if n == 2 {
		return vec![vec![1, 2], vec![2, 1]];
	}

	unsafe {
		let mut vec_ptr: *mut i32 = std::ptr::null_mut();

		(0..n)
			.map(|i| {
				let mut temp_start: *mut i32 = std::ptr::null_mut();
				let mut temp_vec = (0..n)
				.map(|j| {
					if i == 0 {
						j + 1
					} else {
						if j == n - 1 {
							*temp_start
						} else {
							if j == 0 {
								temp_start = vec_ptr.add(0);
							}
							*vec_ptr.add((j + 1).try_into().unwrap())
						}
					}
				})
				.collect::<Vec<i32>>();
				vec_ptr = temp_vec.as_mut_ptr();
				temp_vec
			})
			.collect()
	}
}

#[cfg(test)]
mod tests {
    use super::make_latin_square;

    #[test]
    fn small_tests() {
        for i in 1..11 {
            let square = make_latin_square(i);
            if let Err(e) = verify_latin_square(&square, i) {
                panic!("Verification failed: {e}. Square: \n {square:?}");
            }
        }
    }
    
    fn verify_latin_square(square: &[Vec<i32>], len: i32) -> Result<(), String> {
        use std::collections::HashSet;
        
        if len<=0 {
            Err(String::from("Square should have positive size"))?
        }
        
        let ulen = len as usize;
        if square.len() != ulen {
            Err(format!("Square should have {len} rows, but it has {}", square.len()))?
        }

        for (i, row) in square.iter().enumerate() {
            if row.len() != ulen {
                Err(format!("Square should have {len} columns, but row {} has {} element(s)", i+1, row.len()))?
            }
        }
        
        let numbers:HashSet<i32> = (1..=len as i32).collect();
        
        for i in 0..ulen {
            let mut row = numbers.clone();
            for &number in &square[i] {
                if number <=0 || len < number {
                    Err(format!("Each row should consist of numbers 1..n, but row {} has element {number}", i+1))?
                }
                if !row.remove(&number) {
                    Err(format!("Each row should consist of unique numbers, but row {} has element {number} duplicated", i+1))?
                }
            }
            if let Some(number) = row.iter().next() {
                Err(format!("Each row should have all number 1..n, but row {} has no element {number}", i+1))?
            }

            let mut column = numbers.clone();
            for j in 0..ulen {
                let number = square[j][i];
                if number <=0 || len < number {
                    Err(format!("Each column should consist of numbers 1..n, but column {} has element {number}", i+1))?
                }
                if !column.remove(&number) {
                    Err(format!("Each column should consist of unique numbers, but column {} has element {number} duplicated", i+1))?
                }
            }
            if let Some(number) = column.iter().next() {
                Err(format!("Each column should have all number 1..n, but column {} has no element {number}", i+1))?
            }
        }
        Ok(())
    }

}
