use std::collections::{hash_map::Entry, HashMap, HashSet};

use shared::{Direction, Point};

fn main() {
	shared::print_answers(22, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let mut state: HashSet<_> = input
		.lines()
		.enumerate()
		.flat_map(|(y, line)| {
			line.char_indices().filter_map(move |(x, char)| {
				(char == '#').then_some(Point::new(x as i32, y as i32))
			})
		})
		.collect();
	let mut position = Point::new(
		input.lines().next().unwrap().len() as i32 / 2,
		input.lines().count() as i32 / 2,
	);
	let mut direction = Direction::Up;
	let mut infection_count = 0;
	for _ in 0..10_000 {
		if state.remove(&position) {
			direction.turn_right_mut();
		} else {
			infection_count += 1;
			state.insert(position);
			direction.turn_left_mut();
		}
		position += direction.into_offset();
	}
	infection_count
}

fn get_answer_2(input: &str) -> usize {
	let mut state: HashMap<_, _> = input
		.lines()
		.enumerate()
		.flat_map(|(y, line)| {
			line.char_indices().filter_map(move |(x, char)| {
				(char == '#').then_some((Point::new(x as i32, y as i32), State::Infected))
			})
		})
		.collect();
	let mut position = Point::new(
		input.lines().next().unwrap().len() as i32 / 2,
		input.lines().count() as i32 / 2,
	);
	let mut direction = Direction::Up;
	let mut infection_count = 0;
	for _ in 0..10_000_000 {
		match state.entry(position) {
			Entry::Occupied(mut occupied) => {
				let state = occupied.get_mut();
				match state {
					State::Weakened => {
						infection_count += 1;
						*state = State::Infected;
					}
					State::Infected => {
						direction.turn_right_mut();
						*state = State::Flagged;
					}
					State::Flagged => {
						direction.reverse_mut();
						occupied.remove();
					}
				}
			}
			Entry::Vacant(vacant) => {
				direction.turn_left_mut();
				vacant.insert(State::Weakened);
			}
		}
		position += direction.into_offset();
	}
	infection_count
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
	Weakened,
	Infected,
	Flagged,
}
