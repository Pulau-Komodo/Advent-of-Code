struct GameState {
	deck_one: Vec<u8>,
	deck_two: Vec<u8>,
	history: Vec<(Vec<u8>, Vec<u8>)>,
}

impl GameState {
	fn from_str(str: &str) -> Self {
		let (one, two) = str.split_once("\r\n\r\n").unwrap();
		let deck_one = one
			.lines()
			.skip(1)
			.map(str::parse)
			.collect::<Result<_, _>>()
			.unwrap();
		let deck_two = two
			.lines()
			.skip(1)
			.map(str::parse)
			.collect::<Result<_, _>>()
			.unwrap();
		GameState {
			deck_one,
			deck_two,
			history: Vec::new(),
		}
	}
	fn advance(&mut self) {
		let one = self.deck_one.remove(0);
		let two = self.deck_two.remove(0);
		if one > two {
			self.deck_one.push(one);
			self.deck_one.push(two);
		} else {
			self.deck_two.push(two);
			self.deck_two.push(one);
		}
	}
	fn advance_recursive(&mut self) {
		if self
			.history
			.iter()
			.any(|state| state.0 == self.deck_one && state.1 == self.deck_two)
		{
			self.deck_two.clear();
			return;
		}
		self.history
			.push((self.deck_one.clone(), self.deck_two.clone()));
		let one = self.deck_one.remove(0);
		let two = self.deck_two.remove(0);
		let player_one_won =
			if one as usize <= self.deck_one.len() && two as usize <= self.deck_two.len() {
				let deck_one = &self.deck_one[0..one as usize];
				let deck_two = &self.deck_two[0..two as usize];
				if deck_one.iter().max().unwrap() > deck_two.iter().max().unwrap() {
					// Shortcut from Reddit thread to cut computation time down to a tenth
					true
				} else {
					let deck_one = deck_one.to_owned();
					let deck_two = deck_two.to_owned();
					let mut inner_game = GameState {
						deck_one,
						deck_two,
						history: Vec::new(),
					};
					while !inner_game.is_settled() {
						inner_game.advance_recursive();
					}
					inner_game.deck_two.is_empty()
				}
			} else {
				one > two
			};
		if player_one_won {
			self.deck_one.push(one);
			self.deck_one.push(two);
		} else {
			self.deck_two.push(two);
			self.deck_two.push(one);
		}
	}
	fn is_settled(&self) -> bool {
		self.deck_one.is_empty() || self.deck_two.is_empty()
	}
	fn score(&self) -> u32 {
		let winning_deck = if self.deck_one.is_empty() {
			&self.deck_two
		} else {
			&self.deck_one
		};
		winning_deck
			.iter()
			.rev()
			.enumerate()
			.map(|(index, card)| (index as u32 + 1) * *card as u32)
			.sum()
	}
}

pub fn get_answer_1(input: String) -> String {
	let mut game_state = GameState::from_str(&input);
	while !game_state.is_settled() {
		game_state.advance()
	}
	format!("{}", game_state.score())
}

pub fn get_answer_2(input: String) -> String {
	let mut game_state = GameState::from_str(&input);
	while !game_state.is_settled() {
		game_state.advance_recursive()
	}
	format!("{}", game_state.score())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample_input() {
		let input = "Player 1:\n9\n2\n6\n3\n1\r\n\r\nPlayer 2:\n5\n8\n4\n7\n10";
		let mut game_state = GameState::from_str(input);
		while !game_state.is_settled() {
			game_state.advance()
		}
		assert_eq!(game_state.score(), 306);
	}
	#[test]
	fn sample_input_2() {
		let input = "Player 1:\n9\n2\n6\n3\n1\r\n\r\nPlayer 2:\n5\n8\n4\n7\n10";
		let mut game_state = GameState::from_str(input);
		while !game_state.is_settled() {
			game_state.advance_recursive()
		}
		assert_eq!(game_state.score(), 291);
	}
}
