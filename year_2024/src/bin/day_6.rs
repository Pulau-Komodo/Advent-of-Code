use std::collections::HashSet;

use shared::{Direction, Offset, Point};

fn main() {
	shared::print_answers(6, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let map = Map::from_input(input);
	let visited = map.find_visited();
	visited.len()
}

fn get_answer_2(input: &str) -> usize {
	let map = Map::from_input(input);
	let visited = map.find_visited();
	visited
		.into_iter()
		.filter(|visited| *visited != map.starting_position)
		.filter(|point| map.test_loop(*point))
		.count()
}

struct Map {
	obstacles: HashSet<Point<usize>>,
	largest_x: usize,
	largest_y: usize,
	starting_position: Point<usize>,
}

impl Map {
	fn from_input(input: &str) -> Self {
		let mut guard_starting_position = Point::default();
		let mut largest_x = 0;
		let mut largest_y = 0;
		let obstacles = input
			.lines()
			.enumerate()
			.flat_map(|(y, line)| {
				line.char_indices()
					.map(move |(x, char)| (Point::new(x + 1, y + 1), char))
			})
			.filter_map(|(point, char)| {
				largest_x = largest_x.max(point.x);
				largest_y = largest_y.max(point.y);
				match char {
					'#' => Some(point),
					'^' => {
						guard_starting_position = point;
						None
					}
					_ => None,
				}
			})
			.collect::<HashSet<_>>();
		Self {
			obstacles,
			largest_x,
			largest_y,
			starting_position: guard_starting_position,
		}
	}
	fn find_visited(&self) -> HashSet<Point<usize>> {
		let mut guard_direction = Direction::Up;
		let mut guard_position = self.starting_position;
		let mut visited = HashSet::new();
		while (1..self.largest_x + 1).contains(&guard_position.x)
			&& (1..self.largest_y + 1).contains(&guard_position.y)
		{
			visited.insert(guard_position);
			loop {
				let in_front = apply_offset(guard_position, guard_direction.into_offset());
				if self.obstacles.contains(&in_front) {
					guard_direction.turn_right_mut();
				} else {
					guard_position = in_front;
					break;
				}
			}
		}
		visited
	}
	fn test_loop(&self, extra_obstacle: Point<usize>) -> bool {
		let mut guard_direction = Direction::Up;
		let mut guard_position = self.starting_position;
		let mut visited = HashSet::new();
		while (1..self.largest_x + 1).contains(&guard_position.x)
			&& (1..self.largest_y + 1).contains(&guard_position.y)
		{
			let mut has_turned = false;
			loop {
				let in_front = apply_offset(guard_position, guard_direction.into_offset());
				if self.obstacles.contains(&in_front) || in_front == extra_obstacle {
					guard_direction.turn_right_mut();
					has_turned = true;
				} else {
					guard_position = in_front;
					break;
				}
			}
			if has_turned && !visited.insert((guard_position, guard_direction)) {
				return true;
			}
		}
		false
	}
}

fn apply_offset(point: Point<usize>, offset: Offset<i32>) -> Point<usize> {
	Point::new(
		(point.x as i32 + offset.x) as usize,
		(point.y as i32 + offset.y) as usize,
	)
}
