fn main() {
	shared::print_answers(25, &[get_answer]);
}

fn get_answer(input: &str) -> String {
	number_to_snafu(input.lines().map(snafu_to_number).sum())
}

fn snafu_to_number(input: &str) -> i64 {
	let mut number = 0;
	for char in input.chars() {
		number *= 5;
		number += match char {
			'2' => 2,
			'1' => 1,
			'0' => 0,
			'-' => -1,
			'=' => -2,
			_ => panic!("Invalid input character: {char}"),
		}
	}
	number
}

fn number_to_snafu(mut number: i64) -> String {
	let mut snafu = Vec::new();
	while number > 0 {
		let rem = number % 5;
		snafu.push(rem);
		number /= 5;
	}
	snafu.push(0);
	for i in 0..snafu.len() - 1 {
		if snafu[i] > 2 {
			snafu[i + 1] += 1;
			snafu[i] -= 5;
		}
	}
	if let Some(&0) = snafu.last() {
		snafu.pop();
	}
	snafu
		.into_iter()
		.rev()
		.map(|n| match n {
			-2 => '=',
			-1 => '-',
			0 => '0',
			1 => '1',
			2 => '2',
			_ => panic!("I was hoping it would not come to this."),
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_CASES: [(i64, &str); 16] = [
		(1, "1"),
		(2, "2"),
		(3, "1="),
		(4, "1-"),
		(5, "10"),
		(6, "11"),
		(7, "12"),
		(8, "2="),
		(9, "2-"),
		(10, "20"),
		(15, "1=0"),
		(20, "1-0"),
		(976, "2=-01"),
		(2022, "1=11-2"),
		(12345, "1-0---0"),
		(314159265, "1121-1110-1=0"),
	];
	#[test]
	fn parse() {
		for (number, snafu) in TEST_CASES {
			assert_eq!(snafu_to_number(snafu), number);
		}
	}
	#[test]
	fn snafuify() {
		for (number, snafu) in TEST_CASES {
			assert_eq!(number_to_snafu(number).as_str(), snafu);
		}
	}
}
