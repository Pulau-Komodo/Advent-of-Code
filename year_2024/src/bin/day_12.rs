use std::array;

use shared::{Direction, Grid, IntoPairIterator, Point};

fn main() {
	shared::print_answers(12, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let grid = Grid::with_margin(input.lines().map(|line| line.bytes()), b' ');
	let mut visited = grid.map_ref(|cell| *cell == b' ');
	let mut price = 0;

	loop {
		let Some(point) = visited
			.iter_with_points::<usize>()
			.find_map(|(point, visited)| (!visited).then_some(point))
		else {
			break;
		};
		*visited.get_point_mut(point) = true;
		let kind = grid.get_point(point);
		let mut region_size = 1;
		let mut perimeter_size = 0;
		let mut frontier = Vec::from([point]);
		loop {
			let mut new_frontier = Vec::new();
			for point in frontier.drain(..) {
				for neighbour in point.orthogonal_neighbours() {
					if grid.get_point(neighbour) == kind {
						let was_visited = visited.get_point_mut(neighbour);
						if !*was_visited {
							*was_visited = true;
							region_size += 1;
							new_frontier.push(neighbour);
						}
					} else {
						perimeter_size += 1;
					}
				}
			}
			if new_frontier.is_empty() {
				break;
			}
			std::mem::swap(&mut frontier, &mut new_frontier);
		}
		price += region_size * perimeter_size;
	}
	price
}

fn get_answer_2(input: &str) -> u32 {
	let grid = Grid::with_margin(input.lines().map(|line| line.bytes()), b' ');
	let mut visited = grid.map_ref(|cell| *cell == b' ');
	let mut price = 0;

	loop {
		let Some(point) = visited
			.iter_with_points::<usize>()
			.find_map(|(point, visited)| (!visited).then_some(point))
		else {
			break;
		};
		*visited.get_point_mut(point) = true;
		let kind = grid.get_point(point);
		let mut region_size = 1;
		let mut perimeter: [Vec<_>; 4] = array::from_fn(|_| Vec::new());
		let mut frontier = Vec::from([point]);
		loop {
			let mut new_frontier = Vec::new();
			for point in frontier.drain(..) {
				for (neighbour, direction) in point.orthogonal_neighbours().into_iter().zip([
					Direction::Up,
					Direction::Left,
					Direction::Right,
					Direction::Down,
				]) {
					if grid.get_point(neighbour) == kind {
						let was_visited = visited.get_point_mut(neighbour);
						if !*was_visited {
							*was_visited = true;
							region_size += 1;
							new_frontier.push(neighbour);
						}
					} else {
						perimeter[direction as usize].push(point);
					}
				}
			}
			if new_frontier.is_empty() {
				break;
			}
			std::mem::swap(&mut frontier, &mut new_frontier);
		}

		price += region_size * count_perimeters(perimeter);
	}
	price
}

fn count_perimeters(mut perimeter_sections: [Vec<Point<usize>>; 4]) -> u32 {
	let mut count = 4;
	for direction in [Direction::Up, Direction::Down] {
		let mut perimeter = std::mem::take(&mut perimeter_sections[direction as usize]);
		perimeter.sort_by_key(|point| (point.y, point.x));
		for (a, b) in perimeter.into_iter().pairs() {
			if a.y != b.y || a.x + 1 != b.x {
				count += 1;
			}
		}
	}
	for direction in [Direction::Left, Direction::Right] {
		let mut perimeter = std::mem::take(&mut perimeter_sections[direction as usize]);
		perimeter.sort_by_key(|point| (point.x, point.y));
		for (a, b) in perimeter.into_iter().pairs() {
			if a.x != b.x || a.y + 1 != b.y {
				count += 1;
			}
		}
	}
	count
}
