fn main() {
	shared::print_answers(13, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	input
		.lines()
		.map(Scanner::from_line)
		.filter(|scanner| scanner.will_hit(0))
		.map(|scanner| scanner.score())
		.sum()
}

fn get_answer_2(input: &str) -> u32 {
	let scanners: Vec<_> = input.lines().map(Scanner::from_line).collect();
	for offset in 0.. {
		if scanners.iter().all(|scanner| !scanner.will_hit(offset)) {
			return offset;
		}
	}
	unreachable!();
}

struct Scanner {
	depth: u32,
	range: u32,
}

impl Scanner {
	fn from_line(line: &str) -> Self {
		let (depth, range) = line.split_once(": ").unwrap();
		let depth = depth.parse().unwrap();
		let range = range.parse().unwrap();
		Self { depth, range }
	}
	fn score(&self) -> u32 {
		self.depth * self.range
	}
	fn will_hit(&self, offset: u32) -> bool {
		(self.depth + offset) % (self.range * 2 - 2) == 0
	}
}
