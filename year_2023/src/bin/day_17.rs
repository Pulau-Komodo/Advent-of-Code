use std::collections::{hash_map::Entry, HashMap};

use shared::{Grid, Point};

fn main() {
	shared::print_answers(17, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let grid = Grid::new(
		input
			.lines()
			.map(|line| line.bytes().map(|byte| byte - b'0')),
	);
	let width = grid.width() as u8;
	let height = grid.height() as u8;
	let target = Point {
		x: width - 1,
		y: width - 1,
	};
	let mut visited = HashMap::new();
	let mut frontier = Vec::new();
	frontier.push(RouteState {
		position: Point { x: 0, y: 0 },
		direction: Direction::East,
		steps_in_direction: 0,
		cost: 0,
	});
	let mut cheapest = u32::MAX;
	loop {
		let mut new_frontier = Vec::new();
		for state in frontier.drain(..) {
			if state.position == target {
				cheapest = cheapest.min(state.cost);
			}
			match visited.entry((state.position, state.direction, state.steps_in_direction)) {
				Entry::Occupied(mut entry) => {
					let entry = entry.get_mut();
					if state.cost < *entry {
						*entry = state.cost;
					} else {
						continue;
					}
				}
				Entry::Vacant(entry) => {
					entry.insert(state.cost);
				}
			}
			for (direction, steps_in_direction) in (state.steps_in_direction < 3)
				.then_some((state.direction, state.steps_in_direction + 1))
				.into_iter()
				.chain([
					(state.direction.turn_left(), 1),
					(state.direction.turn_right(), 1),
				]) {
				if let Some(position) = direction.apply(state.position, width, height) {
					let cost = state.cost + grid.get_point(position) as u32;
					new_frontier.push(RouteState {
						position,
						direction,
						steps_in_direction,
						cost,
					})
				}
			}
		}
		if new_frontier.is_empty() {
			break;
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
	cheapest
}

fn get_answer_2(input: &str) -> u32 {
	let grid = Grid::new(
		input
			.lines()
			.map(|line| line.bytes().map(|byte| byte - b'0')),
	);
	let width = grid.width() as u8;
	let height = grid.height() as u8;
	let target = Point {
		x: width - 1,
		y: width - 1,
	};
	let mut visited = HashMap::new();
	let mut frontier = Vec::new();
	frontier.push(RouteState {
		position: Point { x: 0, y: 0 },
		direction: Direction::East,
		steps_in_direction: 0,
		cost: 0,
	});
	let mut cheapest = u32::MAX;
	loop {
		let mut new_frontier = Vec::new();
		for state in frontier.drain(..) {
			if state.position == target && state.steps_in_direction >= 4 {
				cheapest = cheapest.min(state.cost);
			}
			match visited.entry((state.position, state.direction, state.steps_in_direction)) {
				Entry::Occupied(mut entry) => {
					let entry = entry.get_mut();
					if state.cost < *entry {
						*entry = state.cost;
					} else {
						continue;
					}
				}
				Entry::Vacant(entry) => {
					entry.insert(state.cost);
				}
			}
			for (direction, steps_in_direction) in (state.steps_in_direction < 10)
				.then_some((state.direction, state.steps_in_direction + 1))
				.into_iter()
				.chain([()].into_iter().flat_map(|_| {
					(state.steps_in_direction >= 4)
						.then_some([
							(state.direction.turn_left(), 1),
							(state.direction.turn_right(), 1),
						])
						.into_iter()
						.flatten()
				})) {
				if let Some(position) = direction.apply(state.position, width, height) {
					let cost = state.cost + grid.get_point(position) as u32;
					new_frontier.push(RouteState {
						position,
						direction,
						steps_in_direction,
						cost,
					})
				}
			}
		}
		if new_frontier.is_empty() {
			break;
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
	cheapest
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RouteState {
	position: Point<u8>,
	direction: Direction,
	steps_in_direction: u8,
	cost: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
	North,
	East,
	South,
	West,
}

impl Direction {
	fn turn_left(&self) -> Self {
		match self {
			Self::North => Self::West,
			Self::East => Self::North,
			Self::South => Self::East,
			Self::West => Self::South,
		}
	}
	fn turn_right(&self) -> Self {
		match self {
			Self::North => Self::East,
			Self::East => Self::South,
			Self::South => Self::West,
			Self::West => Self::North,
		}
	}
	fn apply(&self, Point { x, y }: Point<u8>, width: u8, height: u8) -> Option<Point<u8>> {
		match self {
			Direction::North => y.checked_sub(1).map(|y| Point { x, y }),
			Direction::East => (x < width - 1).then_some(Point { x: x + 1, y }),
			Direction::South => (y < height - 1).then_some(Point { x, y: y + 1 }),
			Direction::West => x.checked_sub(1).map(|x| Point { x, y }),
		}
	}
}

fn _example_path() -> Vec<Point<u8>> {
	let example = "2>>34^>>>1323
32v>>>35v5623
32552456v>>54
3446585845v52
4546657867v>6
14385987984v4
44578769877v6
36378779796v>
465496798688v
456467998645v
12246868655<v
25465488877v5
43226746555v>";
	example
		.lines()
		.enumerate()
		.flat_map(|(y, line)| {
			line.bytes().enumerate().filter_map(move |(x, byte)| {
				matches!(byte, b'>' | b'<' | b'^' | b'v').then_some(Point {
					x: x as u8,
					y: y as u8,
				})
			})
		})
		.collect()
}
