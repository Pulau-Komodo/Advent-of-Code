use std::collections::HashSet;

fn main() {
	shared::print_answers(3, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let directions = parse_input(input);
	let places = log_visited(&directions);
	places.len()
}

fn get_answer_2(input: &str) -> usize {
	let directions = parse_input(input);
	let places = log_visited_robo(&directions);
	places.len()
}

enum Direction {
	North,
	East,
	South,
	West,
}

fn parse_input(input: &str) -> Vec<Direction> {
	use Direction::*;
	input
		.chars()
		.map(|char| match char {
			'^' => North,
			'>' => East,
			'v' => South,
			'<' => West,
			_ => panic!(),
		})
		.collect()
}

fn log_visited(directions: &[Direction]) -> HashSet<(u32, u32)> {
	let mut places = HashSet::new();
	let mut coordinates = (0, 0);
	places.insert(coordinates);
	for direction in directions {
		use Direction::*;
		match direction {
			North => coordinates.1 -= 1,
			East => coordinates.0 += 1,
			South => coordinates.1 += 1,
			West => coordinates.0 -= 1,
		}
		places.insert(coordinates);
	}
	places
}

fn log_visited_robo(directions: &[Direction]) -> HashSet<(u32, u32)> {
	let mut places = HashSet::new();
	let mut coordinates = (0, 0);
	let mut coordinates_robo = (0, 0);
	let mut robo_turn = false;
	places.insert(coordinates);
	for direction in directions {
		use Direction::*;
		let coordinates = if robo_turn {
			&mut coordinates_robo
		} else {
			&mut coordinates
		};
		match direction {
			North => coordinates.1 -= 1,
			East => coordinates.0 += 1,
			South => coordinates.1 += 1,
			West => coordinates.0 -= 1,
		}
		places.insert(*coordinates);
		robo_turn = !robo_turn;
	}
	places
}
