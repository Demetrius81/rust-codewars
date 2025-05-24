use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark_make_latin_square(c: &mut Criterion) {
	c.bench_function("make_latin_square", |b| b.iter(|| temp::make_latin_square(black_box(20))));
}

pub fn benchmark_make_latin_square_unsafe(c: &mut Criterion) {
	c.bench_function("make_latin_square_unsafe", |b| b.iter(|| temp::make_latin_square_unsafe(black_box(20))));
}

criterion_group!(benches, benchmark_make_latin_square, benchmark_make_latin_square_unsafe);
criterion_main!(benches);

mod temp {
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
		// vec![]
	}
}