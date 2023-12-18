use std::collections::{HashMap, HashSet};

use shared::Point;

fn main() {
	shared::print_answers(16, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let grid = Grid::from_input(input);
	grid.count_energized_by_entry(Light {
		position: Point { x: 0, y: 0 },
		direction: Direction::Right,
	})
}

fn get_answer_2(input: &str) -> usize {
	let grid = Grid::from_input(input);
	(0..=grid.max_x)
		.flat_map(|x| {
			[
				Light {
					position: Point { x, y: 0 },
					direction: Direction::Down,
				},
				Light {
					position: Point { x, y: grid.max_y },
					direction: Direction::Up,
				},
			]
		})
		.chain((0..=grid.max_y).flat_map(|y| {
			[
				Light {
					position: Point { x: 0, y },
					direction: Direction::Right,
				},
				Light {
					position: Point { x: grid.max_x, y },
					direction: Direction::Left,
				},
			]
		}))
		.map(|entry| grid.count_energized_by_entry(entry))
		.max()
		.unwrap()
}

#[derive(Clone, Copy)]
enum Cell {
	/** `.` */
	Empty,
	/** `\` */
	MirrorNwSe,
	/** `/` */
	MirrorSwNe,
	/** `-` */
	SplitterHorizontal,
	/** `|` */
	SplitterVertical,
}

impl Cell {
	fn from_byte(byte: u8) -> Self {
		match byte {
			b'\\' => Self::MirrorNwSe,
			b'/' => Self::MirrorSwNe,
			b'-' => Self::SplitterHorizontal,
			b'|' => Self::SplitterVertical,
			_ => Self::Empty,
		}
	}
}

struct Grid {
	cells: HashMap<Point<u8>, Cell>,
	max_x: u8,
	max_y: u8,
}

impl Grid {
	fn from_input(input: &str) -> Self {
		let mut cells = HashMap::new();
		let mut max_x = 0;
		let mut max_y = 0;
		for (x, y, byte) in input.lines().enumerate().flat_map(|(y, line)| {
			line.bytes()
				.enumerate()
				.map(move |(x, byte)| (x as u8, y as u8, byte))
		}) {
			let cell = Cell::from_byte(byte);
			max_x = u8::max(max_x, x);
			max_y = u8::max(max_y, y);
			if !matches!(cell, Cell::Empty) {
				cells.insert(Point { x, y }, cell);
			}
		}
		Self {
			cells,
			max_x,
			max_y,
		}
	}
	fn get(&self, point: Point<u8>) -> Cell {
		*self.cells.get(&point).unwrap_or(&Cell::Empty)
	}
	fn redirect_light(&self, light: Light) -> &'static [Direction] {
		match (self.get(light.position), light.direction) {
			(Cell::Empty, Direction::Right) => [Direction::Right].as_slice(),
			(Cell::Empty, Direction::Left) => [Direction::Left].as_slice(),
			(Cell::Empty, Direction::Up) => [Direction::Up].as_slice(),
			(Cell::Empty, Direction::Down) => [Direction::Down].as_slice(),
			(Cell::MirrorNwSe, Direction::Right) => [Direction::Down].as_slice(),
			(Cell::MirrorNwSe, Direction::Down) => [Direction::Right].as_slice(),
			(Cell::MirrorNwSe, Direction::Left) => [Direction::Up].as_slice(),
			(Cell::MirrorNwSe, Direction::Up) => [Direction::Left].as_slice(),
			(Cell::MirrorSwNe, Direction::Right) => [Direction::Up].as_slice(),
			(Cell::MirrorSwNe, Direction::Up) => [Direction::Right].as_slice(),
			(Cell::MirrorSwNe, Direction::Left) => [Direction::Down].as_slice(),
			(Cell::MirrorSwNe, Direction::Down) => [Direction::Left].as_slice(),
			(Cell::SplitterHorizontal, Direction::Right) => [Direction::Right].as_slice(),
			(Cell::SplitterHorizontal, Direction::Left) => [Direction::Left].as_slice(),
			(Cell::SplitterHorizontal, Direction::Up | Direction::Down) => {
				[Direction::Left, Direction::Right].as_slice()
			}
			(Cell::SplitterVertical, Direction::Up) => [Direction::Up].as_slice(),
			(Cell::SplitterVertical, Direction::Down) => [Direction::Down].as_slice(),
			(Cell::SplitterVertical, Direction::Right | Direction::Left) => {
				[Direction::Up, Direction::Down].as_slice()
			}
		}
	}
	fn count_energized_by_entry(&self, entry: Light) -> usize {
		let mut lights = HashSet::new();
		lights.insert(entry);
		let mut visited = HashSet::new();
		loop {
			let mut new_lights = HashSet::new();
			let mut any_new = false;
			for light in lights.drain() {
				if !visited.insert(light) {
					continue;
				}
				any_new = true;
				for &new_direction in self.redirect_light(light) {
					if let Some(new_point) =
						new_direction.apply(light.position, self.max_x, self.max_y)
					{
						new_lights.insert(Light {
							position: new_point,
							direction: new_direction,
						});
					}
				}
			}
			if !any_new {
				break;
			}
			std::mem::swap(&mut lights, &mut new_lights);
		}
		let energized: HashSet<_> = visited.into_iter().map(|light| light.position).collect();
		energized.len()
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
	Right,
	Down,
	Left,
	Up,
}

impl Direction {
	fn apply(&self, Point { x, y }: Point<u8>, max_x: u8, max_y: u8) -> Option<Point<u8>> {
		match self {
			Direction::Right => (x < max_x).then_some(Point { x: x + 1, y }),
			Direction::Down => (y < max_y).then_some(Point { x, y: y + 1 }),
			Direction::Left => Some(Point {
				x: x.checked_sub(1)?,
				y,
			}),
			Direction::Up => Some(Point {
				x,
				y: y.checked_sub(1)?,
			}),
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Light {
	position: Point<u8>,
	direction: Direction,
}
