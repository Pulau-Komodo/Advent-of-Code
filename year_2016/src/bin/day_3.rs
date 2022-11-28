fn main() {
	shared::print_answers(3, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	input
		.lines()
		.map(NumberSet::from_str)
		.filter(NumberSet::could_be_triangle)
		.count()
}

fn get_answer_2(input: &str) -> usize {
	let mut lines = input.lines();
	let mut count = 0;
	while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
		count += NumberSet::from_str_vertical([a, b, c])
			.into_iter()
			.filter(NumberSet::could_be_triangle)
			.count();
	}
	count
}

struct NumberSet {
	numbers: [u16; 3],
}

impl NumberSet {
	fn from_str(str: &str) -> Self {
		let mut split = str.split_whitespace();
		let numbers = [
			split.next().unwrap(),
			split.next().unwrap(),
			split.next().unwrap(),
		]
		.map(|n| n.parse().unwrap());
		Self { numbers }
	}
	fn from_str_vertical(str: [&str; 3]) -> [Self; 3] {
		let mut numbers = [[0; 3]; 3];
		for (line_n, line) in str.into_iter().enumerate() {
			for (column_n, n) in line.split_whitespace().enumerate() {
				numbers[column_n][line_n] = n.parse().unwrap();
			}
		}
		numbers.map(|numbers| Self { numbers })
	}
	fn could_be_triangle(&self) -> bool {
		self.numbers[0] < self.numbers[1] + self.numbers[2]
			&& self.numbers[1] < self.numbers[0] + self.numbers[2]
			&& self.numbers[2] < self.numbers[0] + self.numbers[1]
	}
}
