fn main() {
	shared::print_answers(9, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> i32 {
	input
		.lines()
		.map(Sequence::from_line)
		.map(Sequence::predict_next)
		.sum()
}

fn get_answer_2(input: &str) -> i32 {
	input
		.lines()
		.map(Sequence::from_line)
		.map(Sequence::extrapolate_prev)
		.sum()
}

struct Sequence {
	sequence: Vec<i32>,
}

impl Sequence {
	fn from_line(line: &str) -> Self {
		let sequence = line.split(' ').map(|str| str.parse().unwrap()).collect();
		Self { sequence }
	}
	fn predict_next(self) -> i32 {
		let mut next_in_sequence = 0;
		let mut generation = self.sequence;
		let mut next_generation = Vec::with_capacity(generation.len() - 1);
		loop {
			let mut generation_iter = generation.drain(..);
			let mut prev_number = generation_iter.next().unwrap();
			for number in generation_iter {
				next_generation.push(number - prev_number);
				prev_number = number;
			}
			next_in_sequence += prev_number;
			if next_generation.iter().all(|n| *n == 0) {
				break;
			}
			std::mem::swap(&mut generation, &mut next_generation);
		}
		next_in_sequence
	}
	fn extrapolate_prev(self) -> i32 {
		let mut first_numbers = Vec::new();
		let mut generation = self.sequence;
		let mut next_generation = Vec::with_capacity(generation.len() - 1);
		first_numbers.push(*generation.first().unwrap());
		loop {
			let mut generation_iter = generation.drain(..);
			let mut prev_number = generation_iter.next().unwrap();
			for number in generation_iter {
				next_generation.push(number - prev_number);
				prev_number = number;
			}
			first_numbers.push(*next_generation.first().unwrap());
			if next_generation.iter().all(|n| *n == 0) {
				break;
			}
			std::mem::swap(&mut generation, &mut next_generation);
		}
		first_numbers.into_iter().rev().fold(0, |acc, n| n - acc)
	}
}
