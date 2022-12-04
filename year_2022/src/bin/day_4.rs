use std::ops::RangeInclusive;

fn main() {
	shared::print_answers(4, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	input
		.lines()
		.map(AssignmentPair::from_str)
		.filter(AssignmentPair::one_contains_other)
		.count() as u32
}

fn get_answer_2(input: &str) -> u32 {
	input
		.lines()
		.map(AssignmentPair::from_str)
		.filter(AssignmentPair::overlaps_at_all)
		.count() as u32
}

struct AssignmentPair {
	sections: [RangeInclusive<u32>; 2],
}

impl AssignmentPair {
	fn from_str(str: &str) -> Self {
		let (first, second) = str.split_once(',').unwrap();
		let sections = [first, second].map(|text| {
			let (start, end) = text.split_once('-').unwrap();
			let start = start.parse().unwrap();
			let end = end.parse().unwrap();
			start..=end
		});
		Self { sections }
	}
	fn one_contains_other(&self) -> bool {
		self.sections[0].start() <= self.sections[1].start()
			&& self.sections[0].end() >= self.sections[1].end()
			|| self.sections[1].start() <= self.sections[0].start()
				&& self.sections[1].end() >= self.sections[0].end()
	}
	fn overlaps_at_all(&self) -> bool {
		self.sections[0].contains(self.sections[1].start())
			|| self.sections[1].contains(self.sections[0].start())
	}
}
