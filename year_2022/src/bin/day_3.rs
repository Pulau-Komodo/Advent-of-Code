fn main() {
	shared::print_answers(3, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	input
		.lines()
		.map(Rucksack::from_str)
		.map(|rucksack| score_item(rucksack.find_duplicate()) as u32)
		.sum()
}

fn get_answer_2(input: &str) -> u32 {
	let mut lines = input.lines().map(str::as_bytes);
	let mut sum = 0;
	while let (Some(one), Some(two), Some(three)) = (lines.next(), lines.next(), lines.next()) {
		let common = one
			.iter()
			.find(|byte| {
				[two, three]
					.iter()
					.all(|list| list.iter().any(|other| *byte == other))
			})
			.unwrap();
		sum += score_item(*common) as u32;
	}
	sum
}

struct Rucksack<'l> {
	compartments: [&'l [u8]; 2],
}

impl<'l> Rucksack<'l> {
	fn from_str(str: &'l str) -> Self {
		let bytes = str.as_bytes();
		let compartments = [&bytes[..bytes.len() / 2], &bytes[bytes.len() / 2..]];
		Self { compartments }
	}
	fn find_duplicate(&self) -> u8 {
		*self.compartments[0]
			.iter()
			.find(|byte| self.compartments[1].iter().any(|other| *byte == other))
			.unwrap()
	}
}

fn score_item(item: u8) -> u8 {
	match item {
		b'a'..=b'z' => item - b'a' + 1,
		b'A'..=b'Z' => item - b'A' + 1 + 26,
		_ => panic!("Invalid item."),
	}
}
