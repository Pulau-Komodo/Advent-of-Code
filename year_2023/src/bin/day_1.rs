fn main() {
	shared::print_answers(1, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut accumulator = 0;
	let mut prev_digit = None;
	let mut new_line = true;
	for byte in input.bytes() {
		match byte {
			b'\n' => {
				new_line = true;
				accumulator += prev_digit.expect("No second digit on previous line.");
			}
			b'0'..=b'9' => {
				let num = (byte - b'0') as u32;
				if new_line {
					accumulator += num * 10;
					new_line = false;
				}
				prev_digit = Some(num);
			}
			_ => (),
		}
	}
	accumulator
}

const DIGITS: [([&str; 2], u32); 9] = [
	(["1", "one"], 1),
	(["2", "two"], 2),
	(["3", "three"], 3),
	(["4", "four"], 4),
	(["5", "five"], 5),
	(["6", "six"], 6),
	(["7", "seven"], 7),
	(["8", "eight"], 8),
	(["9", "nine"], 9),
];

fn get_answer_2(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			let first = DIGITS
				.iter()
				.flat_map(|(patterns, value)| {
					patterns
						.iter()
						.filter_map(|pattern| line.find(pattern).map(|pos| (pos, *value)))
				})
				.min_by_key(|(pos, _)| *pos)
				.unwrap()
				.1;

			let reversed_line = line.chars().rev().collect::<String>();

			let last = DIGITS
				.iter()
				.flat_map(|(patterns, value)| {
					patterns
						.iter()
						.map(|pattern| pattern.chars().rev().collect::<String>())
						.filter_map(|pattern| reversed_line.find(&pattern).map(|pos| (pos, *value)))
				})
				.min_by_key(|(pos, _)| *pos)
				.unwrap()
				.1;

			first * 10 + last
		})
		.sum()
}
