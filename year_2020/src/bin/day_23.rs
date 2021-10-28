fn main() {
	year_2020::print_answers(23, &[get_answer_1, get_answer_2]);
}

struct GameState {
	cups: std::collections::HashMap<u32, u32>,
	current_cup: u32,
}

impl GameState {
	fn from_str_with_count(str: &str, count: u32) -> Self {
		let starting_cups: Vec<u32> = str
			.chars()
			.map(|char| char.to_string().parse().unwrap())
			.collect();
		let mut cups = std::collections::HashMap::with_capacity(count as usize);
		for (index, &value) in starting_cups.iter().enumerate() {
			let next_cup = starting_cups
				.get(index + 1)
				.map(u32::clone)
				.unwrap_or_else(|| {
					if count > starting_cups.len() as u32 {
						index as u32 + 2
					} else {
						*starting_cups.get(0).unwrap()
					}
				});
			cups.insert(value, next_cup);
		}
		if count > starting_cups.len() as u32 {
			for index in 10..count + 1 {
				let next_cup = if index + 1 > count {
					*starting_cups.get(0).unwrap()
				} else {
					index + 1
				};
				cups.insert(index, next_cup);
			}
		}
		GameState {
			cups,
			current_cup: *starting_cups.get(0).unwrap(),
		}
	}
	fn advance(&mut self) {
		let one = *self.cups.get(&self.current_cup).unwrap();
		let two = *self.cups.get(&one).unwrap();
		let three = *self.cups.get(&two).unwrap();
		let next = *self.cups.get(&three).unwrap();
		let mut selection = self.current_cup;
		let target = loop {
			if selection == 1 {
				selection = self.cups.len() as u32;
			} else {
				selection -= 1;
			}
			if ![one, two, three].contains(&selection) {
				break selection;
			}
		};
		let new_next = *self.cups.get(&target).unwrap();
		self.cups.insert(self.current_cup, next);
		self.cups.insert(target, one);
		self.cups.insert(three, new_next);
		self.current_cup = next;
	}
	fn output_order(&self) -> String {
		let mut output = String::with_capacity(self.cups.len());
		let mut cup = *self.cups.get(&1).unwrap();
		while output.len() < self.cups.len() - 1 {
			output.push_str(format!("{}", cup).as_str());
			cup = *self.cups.get(&cup).unwrap();
		}
		output
	}
	fn output_two_cups(&self) -> (u32, u32) {
		let first_cup = *self.cups.get(&1).unwrap();
		let second_cup = *self.cups.get(&first_cup).unwrap();
		(first_cup, second_cup)
	}
}

fn get_answer_1(input: &str) -> String {
	let mut game = GameState::from_str_with_count(input, input.len() as u32);
	for _ in 0..100 {
		game.advance();
	}
	game.output_order()
}

fn get_answer_2(input: &str) -> String {
	let mut game = GameState::from_str_with_count(input, 1_000_000);
	for _ in 0..10_000_000 {
		game.advance();
	}
	let cups = game.output_two_cups();
	format!("{}", cups.0 as u64 * cups.1 as u64)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_input() {
		let mut game = GameState::from_str_with_count("389125467", 9);
		for _ in 0..10 {
			game.advance();
		}
		assert_eq!(game.output_order(), "92658374");
		let mut game = GameState::from_str_with_count("389125467", 9);
		for _ in 0..100 {
			game.advance();
		}
		assert_eq!(game.output_order(), "67384529");
	}
	#[test]
	fn test_input_2() {
		let mut game = GameState::from_str_with_count("389125467", 1_000_000);
		for _ in 0..10_000_000 {
			game.advance();
		}
		let cups = game.output_two_cups();
		assert_eq!((cups.0, cups.1), (934001, 159792));
	}
}
