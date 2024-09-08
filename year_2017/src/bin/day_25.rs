use std::{collections::VecDeque, ops::Sub};

fn main() {
	shared::print_answers(25, &[get_answer]);
}

fn get_answer(input: &str) -> usize {
	let (start, states) = input.split_once("\n\n").unwrap();
	let mut lines = start.lines();
	let mut state = lines.next().unwrap().bytes().nth_back(1).unwrap().sub(b'A') as usize;
	let step_count: u32 = lines
		.next()
		.unwrap()
		.split(' ')
		.nth_back(1)
		.unwrap()
		.parse()
		.unwrap();
	let states: Vec<_> = states.split("\n\n").map(State::from_str).collect();
	let mut cursor = 0;
	let mut tape = VecDeque::new();
	tape.push_back(false);
	for _ in 0..step_count {
		let value = &mut tape[cursor];
		let actions = states[state].actions[*value as usize];
		*value = actions.write;
		state = actions.next_state;
		match actions.direction {
			Direction::Right => {
				cursor += 1;
				if cursor > tape.len() - 1 {
					tape.push_back(false);
				}
			}
			Direction::Left => {
				if let Some(new_cursor) = cursor.checked_sub(1) {
					cursor = new_cursor;
				} else {
					tape.push_back(false);
					tape.rotate_right(1);
				}
			}
		}
	}
	tape.into_iter().filter(|on| *on).count()
}

struct State {
	actions: [Actions; 2],
}

impl State {
	fn from_str(str: &str) -> Self {
		let (_, outcomes) = str.split_once("If the current value is 0:\n").unwrap();
		let (outcome_true, outcome_false) =
			outcomes.split_once("If the current value is 1:\n").unwrap();
		Self {
			actions: [outcome_true, outcome_false].map(Actions::from_str),
		}
	}
}

#[derive(Debug, Clone, Copy)]
struct Actions {
	write: bool,
	direction: Direction,
	next_state: usize,
}

impl Actions {
	fn from_str(str: &str) -> Self {
		let mut lines = str.lines();
		let write = lines.next().unwrap() == "    - Write the value 1.";
		let direction = if lines.next().unwrap() == "    - Move one slot to the right." {
			Direction::Right
		} else {
			Direction::Left
		};
		let next_state = lines.next().unwrap().bytes().nth_back(1).unwrap().sub(b'A') as usize;
		Self {
			write,
			direction,
			next_state,
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Direction {
	Left,
	Right,
}
