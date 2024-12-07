use std::collections::HashSet;

use shared::{Direction, Offset, Point, SmallMap};

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
	obstructions: Obstructions,
	largest_x: usize,
	largest_y: usize,
	starting_position: Point<usize>,
}

impl Map {
	fn from_input(input: &str) -> Self {
		let mut starting_position = Point::default();
		let mut largest_x = 0;
		let mut largest_y = 1;
		let mut highest_obstruction_x = 0;
		let mut y_obstructions = Vec::new();
		for (y, line) in input.lines().enumerate() {
			largest_y += 1;
			y_obstructions.push(Vec::new());
			for (x, char) in line.char_indices() {
				if char == '#' {
					y_obstructions.last_mut().unwrap().push(x);
					highest_obstruction_x = highest_obstruction_x.max(x);
				} else if char == '^' {
					starting_position = Point::new(x + 1, y + 1);
				}
				largest_x = largest_x.max(x + 1);
			}
		}
		let mut x_obstructions = (0..=highest_obstruction_x)
			.map(|_| Vec::new())
			.collect::<Vec<_>>();
		for (y, row) in y_obstructions.iter().enumerate() {
			for x in row {
				x_obstructions[*x].push(y);
			}
		}
		for column in &mut x_obstructions {
			column.sort();
		}
		let obstructions = Obstructions {
			x: x_obstructions,
			y: y_obstructions,
		};
		Self {
			obstructions,
			largest_x,
			largest_y,
			starting_position,
		}
	}
	fn find_visited(&self) -> HashSet<Point<usize>> {
		let mut guard_direction = Direction::Up;
		let mut guard_position = self.starting_position;
		let mut visited = HashSet::new();
		'outer: loop {
			let Some(distance) = self
				.obstructions
				.walkable(guard_position, guard_direction, None)
			else {
				while (1..self.largest_x + 1).contains(&guard_position.x)
					&& (1..self.largest_y + 1).contains(&guard_position.y)
				{
					visited.insert(guard_position);
					guard_position = apply_offset(guard_position, guard_direction.into_offset());
				}
				break 'outer;
			};
			for _ in 0..distance {
				visited.insert(guard_position);
				guard_position = apply_offset(guard_position, guard_direction.into_offset());
			}
			guard_direction.turn_right_mut();
		}
		visited
	}
	fn test_loop(&self, extra_obstacle: Point<usize>) -> bool {
		let mut guard_direction = Direction::Up;
		let mut guard_position = self.starting_position;
		let mut visited = SmallMap::new();
		while let Some(distance) =
			self.obstructions
				.walkable(guard_position, guard_direction, Some(extra_obstacle))
		{
			guard_position = apply_offset(
				guard_position,
				guard_direction.into_offset() * distance as i32,
			);
			guard_direction.turn_right_mut();
			if distance > 0 && visited.insert(guard_position, ()).is_some() {
				return true;
			}
		}
		false
	}
}

struct Obstructions {
	x: Vec<Vec<usize>>,
	y: Vec<Vec<usize>>,
}

impl Obstructions {
	fn walkable(
		&self,
		position: Point<usize>,
		direction: Direction,
		bonus_obstruction: Option<Point<usize>>,
	) -> Option<usize> {
		let position = apply_offset(position, Offset::new(-1, -1));
		let (line, start, reverse) = match direction {
			Direction::Up => (&self.x[position.x], position.y, true),
			Direction::Right => (&self.y[position.y], position.x, false),
			Direction::Down => (&self.x[position.x], position.y, false),
			Direction::Left => (&self.y[position.y], position.x, true),
		};
		let distance = if reverse {
			line.iter()
				.rev()
				.find(|obstruction| **obstruction < start)
				.map(|obstruction| start - obstruction - 1)
		} else {
			line.iter()
				.find(|obstruction| **obstruction > start)
				.map(|obstruction| obstruction - start - 1)
		};
		let Some(obstruction) = bonus_obstruction else {
			return distance;
		};
		let obstruction = apply_offset(obstruction, Offset::new(-1, -1));
		let bonus_distance = match direction {
			Direction::Up if position.x == obstruction.x => position.y.checked_sub(obstruction.y),
			Direction::Right if position.y == obstruction.y => {
				obstruction.x.checked_sub(position.x)
			}
			Direction::Down if position.x == obstruction.x => obstruction.y.checked_sub(position.y),
			Direction::Left if position.y == obstruction.y => position.x.checked_sub(obstruction.x),
			_ => None,
		}
		.map(|n| n - 1);
		match (distance, bonus_distance) {
			(Some(distance), Some(bonus)) => Some(distance.min(bonus)),
			_ => distance.or(bonus_distance),
		}
	}
}

fn apply_offset(point: Point<usize>, offset: Offset<i32>) -> Point<usize> {
	Point::new(
		(point.x as i32 + offset.x) as usize,
		(point.y as i32 + offset.y) as usize,
	)
}
