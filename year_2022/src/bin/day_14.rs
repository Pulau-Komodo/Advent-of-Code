use std::collections::{HashSet, VecDeque};

use shared::{Grid, Point};

fn main() {
	shared::print_answers(14, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let (mut min_x, mut max_x, mut max_y) = (u16::MAX, u16::MIN, u16::MIN);
	let rock_formations: Vec<_> = input
		.lines()
		.map(|line| {
			RockFormation::from_str_tracking(line, |point| {
				min_x = min_x.min(point.x);
				max_x = max_x.max(point.x);
				max_y = max_y.max(point.y);
			})
		})
		.collect();
	// The highest value - the lowest + 1 for fenceposting + 2 for margin
	let grid_size = (max_x - min_x + 3, max_y);
	let mut cave = Cave::new(grid_size.0, grid_size.1, min_x - 1);
	for rock_formation in rock_formations {
		cave.add_rock_formation(rock_formation);
	}
	cave.calculate_sand_amount()
}

fn get_answer_2(input: &str) -> u32 {
	let mut max_y = u16::MIN;
	let mut cave = SparseCave::new();
	for rock_formation in input.lines().map(|line| {
		RockFormation::from_str_tracking(line, |point| {
			max_y = max_y.max(point.y);
		})
	}) {
		cave.add_rock_formation(&rock_formation);
	}
	cave.floor = max_y + 1;
	cave.calculate_sand_amount()
}

const SAND_ORIGIN: Point<u16> = Point { x: 500, y: 0 };

struct SparseCave {
	rocks: HashSet<Point<u16>>,
	floor: u16,
}

impl SparseCave {
	fn new() -> Self {
		let rocks = HashSet::new();
		Self { rocks, floor: 0 }
	}
	fn add_rock_formation(&mut self, rock_formation: &RockFormation) {
		for point in rock_formation {
			self.rocks.insert(point);
		}
	}
	fn calculate_sand_amount(&self) -> u32 {
		let mut frontier = Vec::from([SAND_ORIGIN]);
		let mut new_frontier = Vec::new();
		let mut previous_two = VecDeque::with_capacity(2);
		let mut sand_count = 1;
		for _ in 0..self.floor {
			for point in frontier.drain(..) {
				for x in 0..3 {
					let new_point = Point {
						x: point.x + x - 1,
						y: point.y + 1,
					};
					if (x == 2 || !previous_two.contains(&new_point))
						&& !self.rocks.contains(&new_point)
					{
						new_frontier.push(new_point);
						sand_count += 1;
					}
					if previous_two.len() > 2 {
						previous_two.pop_front();
					}
					previous_two.push_back(new_point)
				}
			}
			if new_frontier.is_empty() {
				break;
			}
			previous_two.clear();
			std::mem::swap(&mut frontier, &mut new_frontier);
		}
		sand_count
	}
}

struct Cave {
	grid: Grid<bool>,
	offset: u16,
	height: u16,
}

impl Cave {
	fn new(width: u16, height: u16, offset: u16) -> Self {
		let grid = Grid::empty(width as usize, height as usize + 1, false);
		Self {
			grid,
			height,
			offset,
		}
	}
	fn add_rock_formation(&mut self, rock_formation: RockFormation) {
		for point in rock_formation.into_iter().map(|point| Point {
			x: point.x - self.offset,
			y: point.y,
		}) {
			self.grid[point] = true;
		}
	}
	fn calculate_sand_amount(&mut self) -> u32 {
		let mut path = Vec::with_capacity(self.height as usize);
		path.push((
			Point {
				x: SAND_ORIGIN.x - self.offset,
				y: SAND_ORIGIN.y,
			},
			0,
		));
		let mut sand_count = 0;
		loop {
			let (last, tries) = path.last_mut().unwrap();
			let offset = match *tries {
				0 => 1,
				1 => 0,
				2 => 2,
				3 => {
					self.grid[*last] = true;
					path.pop();
					sand_count += 1;
					continue;
				}
				_ => unreachable!(),
			};
			*tries += 1;
			let new = Point {
				x: last.x + offset - 1,
				y: last.y + 1,
			};
			if new.y > self.height {
				break;
			}
			if !self.grid[new] {
				path.push((new, 0));
			}
		}
		sand_count
	}
}

#[derive(Clone)]
enum Direction {
	Left,
	Right,
	Up,
	Down,
}

impl Direction {
	fn between(from: Point<u16>, to: Point<u16>) -> Self {
		if to.x > from.x {
			Direction::Right
		} else if to.x < from.x {
			Direction::Left
		} else if to.y > from.y {
			Direction::Down
		} else {
			Direction::Up
		}
	}
}

#[derive(Clone)]
struct RockFormation {
	start: Point<u16>,
	movements: Vec<(Direction, u16)>,
}

impl RockFormation {
	fn from_str_tracking<F>(str: &str, mut tracker: F) -> Self
	where
		F: FnMut(Point<u16>),
	{
		let mut points = str.split(" -> ").map(Point::<u16>::from_comma_separated);
		let start = points.next().unwrap();
		tracker(start);
		let mut previous_point = start;
		let movements = points
			.map(|point| {
				tracker(point);
				let distance = point
					.x
					.abs_diff(previous_point.x)
					.max(point.y.abs_diff(previous_point.y));
				let direction = Direction::between(previous_point, point);
				previous_point = point;
				(direction, distance)
			})
			.collect();
		Self { start, movements }
	}
}

impl<'l> IntoIterator for &'l RockFormation {
	type Item = Point<u16>;
	type IntoIter = RockFormationIter<'l>;
	fn into_iter(self) -> Self::IntoIter {
		RockFormationIter {
			point: self.start,
			rock_formation: self,
			direction: 0,
			progress: 0,
		}
	}
}

struct RockFormationIter<'l> {
	rock_formation: &'l RockFormation,
	point: Point<u16>,
	direction: usize,
	progress: u16,
}

impl<'l> Iterator for RockFormationIter<'l> {
	type Item = Point<u16>;
	fn next(&mut self) -> Option<Self::Item> {
		// Reached the end, return our last point.
		if self.direction == self.rock_formation.movements.len() {
			self.direction += 1;
			return Some(self.point);
		}
		let (direction, distance) = self.rock_formation.movements.get(self.direction)?;
		let point = self.point;
		match direction {
			Direction::Left => self.point.x -= 1,
			Direction::Right => self.point.x += 1,
			Direction::Up => self.point.y -= 1,
			Direction::Down => self.point.y += 1,
		}
		self.progress += 1;
		if self.progress >= *distance {
			self.direction += 1;
			self.progress = 0;
		}
		Some(point)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rock_formation_iter() {
		let rock_formation = RockFormation::from_str_tracking("0,2 -> 2,2 -> 2,0", |_| {});
		let points: Vec<_> = rock_formation.into_iter().collect();
		assert_eq!(
			points,
			[
				Point { x: 0, y: 2 },
				Point { x: 1, y: 2 },
				Point { x: 2, y: 2 },
				Point { x: 2, y: 1 },
				Point { x: 2, y: 0 },
			]
			.to_vec()
		);
	}
}
