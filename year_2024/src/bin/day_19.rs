fn main() {
	shared::print_answers(19, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let (towels, patterns) = input.split_once("\n\n").unwrap();
	let towels = towels.split(", ").map(Towel::from_str).collect::<Vec<_>>();

	let mut count = 0;
	for pattern in patterns.trim().split("\n").map(Pattern::from_str) {
		let mut reached = vec![false; pattern.stripes.len() + 1];
		reached[0] = true;
		for length in 0..pattern.stripes.len() {
			let remaining_pattern = pattern.slice(length);
			for &towel in &towels {
				if towel.fits_pattern(remaining_pattern) {
					reached[length + towel.stripes.len()] |= reached[length];
				}
			}
		}
		if *reached.last().unwrap() {
			count += 1;
		}
	}
	count
}

fn get_answer_2(input: &str) -> u64 {
	let (towels, patterns) = input.split_once("\n\n").unwrap();
	let towels = towels.split(", ").map(Towel::from_str).collect::<Vec<_>>();

	let mut sum = 0;
	for pattern in patterns.trim().split("\n").map(Pattern::from_str) {
		let mut paths = vec![0; pattern.stripes.len() + 1];
		paths[0] = 1;
		for length in 0..pattern.stripes.len() {
			let path_count = paths[length];
			for &towel in &towels {
				if towel.fits_pattern(pattern.slice(length)) {
					paths[length + towel.stripes.len()] += path_count;
				}
			}
		}
		sum += paths.last().unwrap();
	}
	sum
}

#[derive(Debug, Clone, Copy)]
struct Towel<'l> {
	stripes: &'l [u8],
}

impl<'l> Towel<'l> {
	fn from_str(str: &'l str) -> Self {
		Self {
			stripes: str.as_bytes(),
		}
	}
	fn fits_pattern(self, pattern: Pattern) -> bool {
		self.stripes.len() <= pattern.stripes.len()
			&& pattern
				.stripes
				.iter()
				.zip(self.stripes)
				.all(|(a, b)| a == b)
	}
}

#[derive(Debug, Clone, Copy)]
struct Pattern<'l> {
	stripes: &'l [u8],
}

impl<'l> Pattern<'l> {
	fn from_str(str: &'l str) -> Self {
		Self {
			stripes: str.as_bytes(),
		}
	}
	fn slice(mut self, index: usize) -> Self {
		self.stripes = &self.stripes[index..];
		self
	}
}
