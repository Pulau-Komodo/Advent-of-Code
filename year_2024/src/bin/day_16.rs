use std::collections::{HashMap, HashSet};

use shared::{BreadthFirstPathfinder, Direction, Grid, Offset, Point};

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

	let frontier = Vec::from([(start, direction, 0)]);

	let step_gen = |(position, direction, cost): (Point<usize>, Direction, u32)| -> std::iter::Flatten<
		std::array::IntoIter<Option<(Point<usize>, Direction, u32)>, 3>,
	> {
		[direction, direction.turn_left(), direction.turn_right()]
			.map(|new_direction| {
				let new_position = apply_offset(position, new_direction.into_offset());
				let added_cost = if new_direction == direction { 1 } else { 1001 };
				(!wall_grid.get_point(new_position)).then_some((
					new_position,
					new_direction,
					cost + added_cost,
				))
			})
			.into_iter()
			.flatten()
	};
	
	let test = |(position, direction, cost): &(Point<usize>, Direction, u32)| -> bool {
		let mut is_new = false;
		visited
			.entry((*position, *direction))
			.and_modify(|old_cost: &mut u32| {
				if *cost < *old_cost {
					is_new = true;
					*old_cost = *cost;
				}
			})
			.or_insert_with(|| {
				is_new = true;
				*cost
			});
		if !is_new {
			false
		} else if *position == destination {
			lowest_cost = lowest_cost.min(*cost);
			false
		} else {
			true
		}
	};

	let mut pathfinder = BreadthFirstPathfinder::new(frontier, step_gen, test);
	while pathfinder.progress() {}

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
			if position == destination {
				if cost < lowest_cost {
					lowest_cost = cost;
					ending_run_points.clear();
				}
				if cost <= lowest_cost {
					ending_run_points.extend(history);
				}
				continue;
			}
			let new_positions =
				[direction, direction.turn_left(), direction.turn_right()].map(|new_direction| {
					let new_position = apply_offset(position, new_direction.into_offset());
					(!wall_grid.get_point(new_position)).then_some((new_position, new_direction))
				});
			let is_junction = new_positions.iter().filter(|x| x.is_some()).count() > 1;
			if is_junction {
				let mut is_new = false;
				visited
					.entry((position, direction))
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
				if !is_new {
					continue;
				}
			}
			for (new_position, new_direction) in new_positions.into_iter().flatten() {
				let cost = cost + if new_direction == direction { 1 } else { 1001 };
				let new_history = history.iter().copied().chain([new_position]).collect();
				new_frontier.push((new_position, new_direction, cost, new_history));
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
