use std::{fmt::Display, ops::RangeInclusive};

use shared::Vec2;

fn main() {
	shared::print_answers(17, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> Box<dyn Display> {
	let mut frontier: Vec<Position> = Vec::from([Position::new()]);
	let mut new_frontier = Vec::new();
	let solution = 'outer: loop {
		for position in frontier.drain(..) {
			for new_position in position.possible_moves(input) {
				if new_position.is_solution() {
					break 'outer new_position;
				} else {
					new_frontier.push(new_position)
				}
			}
		}
		if new_frontier.is_empty() {
			panic!("Could not find a path.")
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	};
	Box::new(solution.history)
}

fn get_answer_2(input: &str) -> Box<dyn Display> {
	let mut frontier: Vec<Position> = Vec::from([Position::new()]);
	let mut new_frontier = Vec::new();
	let mut latest_find = None;
	loop {
		for position in frontier.drain(..) {
			for new_position in position.possible_moves(input) {
				if new_position.is_solution() {
					latest_find = Some(new_position);
				} else {
					new_frontier.push(new_position);
				}
			}
		}
		if new_frontier.is_empty() {
			break;
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
	Box::new(latest_find.expect("No path found.").history.len())
}

const OPEN_VALUES: RangeInclusive<u8> = 0xb..=0xf;
const DIRECTION_CHARS: [char; 4] = ['U', 'D', 'L', 'R'];
// up, down, left, right
const OFFSETS: [Vec2<i8>; 4] = [
	Vec2::new(0, -1),
	Vec2::new(0, 1),
	Vec2::new(-1, 0),
	Vec2::new(1, 0),
];

struct Position {
	pos: Vec2<i8>,
	history: String,
}

impl Position {
	fn new() -> Self {
		Self {
			pos: Vec2::new(0, 0),
			history: String::new(),
		}
	}
	fn possible_moves(&self, passcode: &str) -> impl Iterator<Item = Self> + '_ {
		let hash = shared::md5(format!("{passcode}{}", self.history).as_bytes());
		let hash = hash.to_le_bytes();
		let relevant: [_; 4] = std::array::from_fn(|i| {
			let byte_index = 15 - i / 2;
			if i % 2 == 0 {
				hash[byte_index] >> 4
			} else {
				hash[byte_index] & 0b1111
			}
		});
		OFFSETS
			.into_iter()
			.zip(relevant)
			.zip(DIRECTION_CHARS)
			.filter_map(move |((offset, decider), char)| {
				let pos = self.pos + offset;
				(OPEN_VALUES.contains(&decider)
					&& (0..4).contains(&pos.x)
					&& (0..4).contains(&pos.y))
				.then(|| {
					let mut history = String::with_capacity(self.history.len() + 1);
					history.push_str(&self.history);
					history.push(char);
					Position { pos, history }
				})
			})
	}
	fn is_solution(&self) -> bool {
		self.pos == Vec2 { x: 3, y: 3 }
	}
}
