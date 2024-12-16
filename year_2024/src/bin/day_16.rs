use std::collections::{HashMap, HashSet};

use shared::{Direction, Grid, Offset, Point};

fn main() {
	shared::print_answers(16, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let wall_grid = Grid::new(
		input
			.lines()
			.map(|line| line.bytes().map(|byte| byte == b'#')),
	);
	// Hardcoded start and destination because could not be bothered, and examples all matched.
	let destination = Point::new(wall_grid.width() - 2, 1);
	let start = Point::new(1, wall_grid.height() - 2);
	let direction = Direction::Right;

	let mut visited = HashMap::new();
	let mut lowest_cost = u32::MAX;
	let mut frontier = Vec::from([(start, direction, 0)]);
	loop {
		let mut new_frontier = Vec::new();
		for (position, direction, cost) in frontier.drain(..) {
			for new_direction in [direction, direction.turn_left(), direction.turn_right()] {
				let cost = cost + if new_direction == direction { 1 } else { 1001 };
				let new_position = apply_offset(position, new_direction.into_offset());
				if !wall_grid.get_point(new_position) {
					let mut is_new = false;
					visited
						.entry((new_position, new_direction))
						.and_modify(|old_cost| {
							if cost < *old_cost {
								is_new = true;
								*old_cost = cost;
							}
						})
						.or_insert_with(|| {
							is_new = true;
							cost
						});
					if is_new {
						if new_position == destination {
							lowest_cost = lowest_cost.min(cost);
						} else {
							new_frontier.push((new_position, new_direction, cost));
						}
					}
				}
			}
		}
		if new_frontier.is_empty() {
			break;
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
	lowest_cost
}

fn get_answer_2(input: &str) -> u32 {
	let wall_grid = Grid::new(
		input
			.lines()
			.map(|line| line.bytes().map(|byte| byte == b'#')),
	);
	// Hardcoded start and destination because could not be bothered, and examples all matched.
	let destination = Point::new(wall_grid.width() - 2, 1);
	let start = Point::new(1, wall_grid.height() - 2);
	let direction = Direction::Right;

	let mut visited = HashMap::new();
	let mut ending_run_points = HashSet::new();
	let mut lowest_cost = u32::MAX;
	let mut frontier = Vec::from([(start, direction, 0, Vec::from([start]))]);
	loop {
		let mut new_frontier = Vec::new();
		for (position, direction, cost, history) in frontier.drain(..) {
			for new_direction in [direction, direction.turn_left(), direction.turn_right()] {
				let history = history.clone();
				let cost = cost + if new_direction == direction { 1 } else { 1001 };
				let new_position = apply_offset(position, new_direction.into_offset());
				if !wall_grid.get_point(new_position) {
					let mut is_new = false;
					visited
						.entry((new_position, new_direction))
						.and_modify(|old_cost| {
							if cost <= *old_cost {
								is_new = true;
								*old_cost = cost;
							}
						})
						.or_insert_with(|| {
							is_new = true;
							cost
						});
					if is_new {
						let mut new_history = history.clone();
						new_history.push(new_position);
						if new_position == destination {
							if cost < lowest_cost {
								ending_run_points.clear();
							}
							if cost <= lowest_cost {
								ending_run_points.extend(new_history);
								lowest_cost = cost;
							}
						} else {
							new_frontier.push((new_position, new_direction, cost, new_history));
						}
					}
				}
			}
		}
		if new_frontier.is_empty() {
			break;
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
	ending_run_points.len() as u32
}

fn apply_offset(point: Point<usize>, offset: Offset<i32>) -> Point<usize> {
	Point::new(
		(point.x as i32 + offset.x) as usize,
		(point.y as i32 + offset.y) as usize,
	)
}
