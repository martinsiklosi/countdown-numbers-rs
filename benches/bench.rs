#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
	use countdown::find_combination;
	use test::Bencher;

	#[bench]
	fn bench_find_combinations(b: &mut Bencher) {
		let numbers = vec![25, 50, 75, 100, 3, 6];
		let target = 952;
		b.iter(|| find_combination(&numbers, &target));
	}
}
