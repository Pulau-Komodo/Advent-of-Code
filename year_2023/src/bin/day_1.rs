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

struct Digit {
	value: u32,
	digit: String,
	word: &'static str,
}

impl Digit {
	fn new(value: u32, word: &'static str) -> Self {
		let digit = format!("{value}");
		Self { value, digit, word }
	}
	fn patterns(&self) -> impl Iterator<Item = &str> {
		[self.digit.as_str(), self.word].into_iter()
	}
	fn find_first(&self, line: &str) -> Option<usize> {
		self.patterns()
			.filter_map(|pattern| line.find(pattern))
			.min()
	}
	fn find_last(&self, line: &str) -> Option<usize> {
		self.patterns()
			.filter_map(|pattern| line.rfind(pattern))
			.max()
	}
}

const DIGITS: [(u32, &str); 9] = [
	(1, "one"),
	(2, "two"),
	(3, "three"),
	(4, "four"),
	(5, "five"),
	(6, "six"),
	(7, "seven"),
	(8, "eight"),
	(9, "nine"),
];

fn get_answer_2(input: &str) -> u32 {
	let digits = DIGITS.map(|(value, word)| Digit::new(value, word));

	input
		.lines()
		.map(|line| {
			let (_, first) = digits
				.iter()
				.flat_map(|digit| digit.find_first(line).map(|pos| (pos, digit.value)))
				.min_by_key(|(pos, _)| *pos)
				.unwrap();

			let (_, last) = digits
				.iter()
				.flat_map(|digit| digit.find_last(line).map(|pos| (pos, digit.value)))
				.max_by_key(|(pos, _)| *pos)
				.unwrap();

			first * 10 + last
		})
		.sum()
}
