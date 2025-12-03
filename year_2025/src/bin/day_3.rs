fn main() {
	shared::print_answers(3, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	input
		.lines()
		.map(BatteryBank::from_line)
		.map(|bank| bank.highest_joltage(2))
		.sum()
}

fn get_answer_2(input: &str) -> u64 {
	input
		.lines()
		.map(BatteryBank::from_line)
		.map(|bank| bank.highest_joltage(12))
		.sum()
}

struct BatteryBank<'l> {
	batteries: &'l [u8],
}

impl<'l> BatteryBank<'l> {
	fn from_line(line: &'l str) -> Self {
		Self {
			batteries: line.as_bytes(),
		}
	}
	fn highest_joltage(&self, battery_count: usize) -> u64 {
		let mut joltage = 0;
		let mut prev_index = 0;
		for i in (0..battery_count).rev() {
			let (index, battery) = self
				.batteries
				.iter()
				.take(self.batteries.len() - i)
				.enumerate()
				.skip(prev_index)
				.rev()
				.max_by_key(|(_i, b)| *b)
				.unwrap();
			prev_index = index + 1;
			joltage += ((battery - b'0') as u64) * 10_u64.pow(i as u32);
		}
		joltage
	}
}
