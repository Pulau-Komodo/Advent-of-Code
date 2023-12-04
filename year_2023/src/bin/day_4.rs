use std::collections::VecDeque;

fn main() {
	shared::print_answers(4, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	input
		.lines()
		.map(Card::from_line)
		.map(|card| card.score())
		.sum()
}

fn get_answer_2(input: &str) -> u32 {
	let mut copies: VecDeque<u32> = [1; 10].into_iter().collect(); // Technically here I hardcode the maximum number of winning numbers.
	input
		.lines()
		.map(Card::from_line)
		.map(|card| {
			let current_copies = copies.pop_front().unwrap();
			copies.push_back(1);
			for count in copies.iter_mut().take(card.count_matches()) {
				*count += current_copies;
			}
			current_copies
		})
		.sum()
}

#[derive(Debug)]
struct Card<'l> {
	winning_numbers: Vec<u8>,
	drawn_numbers: &'l str,
}

impl<'l> Card<'l> {
	fn from_line(line: &'l str) -> Self {
		let (_, numbers_part) = line.split_once(": ").unwrap();
		let (winning, drawn_numbers) = numbers_part.split_once(" | ").unwrap();
		let mut winning_numbers = Vec::with_capacity(10);
		for number in winning.split_ascii_whitespace() {
			winning_numbers.push(number.parse().unwrap());
		}
		Self {
			winning_numbers,
			drawn_numbers,
		}
	}
	fn drawn_numbers(&self) -> impl Iterator<Item = u8> + '_ {
		self.drawn_numbers
			.split_ascii_whitespace()
			.map(|n| n.parse().unwrap())
	}
	fn count_matches(&self) -> usize {
		self.drawn_numbers()
			.filter(|n| self.winning_numbers.iter().any(|winning| n == winning))
			.count()
	}
	fn score(&self) -> u32 {
		let winners = self.count_matches();
		if winners == 0 {
			0
		} else {
			2_u32.pow(winners as u32 - 1)
		}
	}
}
