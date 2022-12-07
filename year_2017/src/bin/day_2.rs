fn main() {
	shared::print_answers(2, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			let (min, max) =
				line.split_ascii_whitespace()
					.fold((u32::MAX, u32::MIN), |(min, max), n| {
						let n = n.parse().unwrap();
						(min.min(n), max.max(n))
					});
			max - min
		})
		.sum()
}

fn get_answer_2(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			let numbers: Vec<_> = line
				.split_ascii_whitespace()
				.map(|text| text.parse::<u32>().unwrap())
				.collect();
			for (i, a) in numbers.iter().enumerate() {
				for b in numbers.iter().skip(i + 1) {
					if let Some(n) = try_divide(*a, *b) {
						return n;
					}
				}
			}
			panic!("Could not find dividing pair.")
		})
		.sum()
}

fn try_divide(a: u32, b: u32) -> Option<u32> {
	if a % b == 0 {
		Some(a / b)
	} else if b % a == 0 {
		Some(b / a)
	} else {
		None
	}
}
