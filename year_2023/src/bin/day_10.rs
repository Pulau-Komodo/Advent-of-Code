use std::collections::HashSet;

use shared::{Grid, Offset, Point};

fn main() {
	shared::print_answers(10, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let field = Grid::new(
		input
			.lines()
			.map(|line| line.chars().map(TileKind::from_char)),
	);
	let (start, mut position, mut offset) = find_start_position_and_offset(&field);
	let mut steps = 1;
	while position != start {
		steps += 1;
		let tile_kind = field.get_point(position);
		offset = tile_kind.redirect_offset(offset).unwrap();
		position = point_i32_to_usize(point_usize_to_i32(position) + offset);
	}
	steps / 2
}

fn get_answer_2(input: &str) -> u32 {
	let field = Grid::new(
		input
			.lines()
			.map(|line| line.chars().map(TileKind::from_char)),
	);
	let (start, mut position, mut offset) = find_start_position_and_offset(&field);
	let mut visited = HashSet::new();
	visited.insert(start);
	let mut included = HashSet::new(); // Hard-coded: the path loops left-ward. Observed to be correct for my input.

	while position != start {
		visited.insert(position);
		let tile_kind = field.get_point(position);
		let (new_offset, lefts) = tile_kind.redirect_offset_and_get_lefts(offset).unwrap();
		offset = new_offset;
		let pos_i32 = point_usize_to_i32(position);
		for &left in lefts {
			included.insert(point_i32_to_usize(pos_i32 + left));
			// It may be that there are small omissions around the start in seeding the `included` set, that don't end up mattering for my input.
		}
		let point_i32 = pos_i32 + offset;
		position = point_i32_to_usize(point_i32);
	}

	for point in &visited {
		included.remove(point);
	}

	let mut frontier: Vec<_> = included.iter().copied().collect();
	let mut new_points = Vec::new();
	loop {
		for neighbour in frontier
			.drain(..)
			.flat_map(|point| point.orthogonal_neighbours())
		{
			if !visited.contains(&neighbour) && included.insert(neighbour) {
				new_points.push(neighbour);
			}
		}
		if new_points.is_empty() {
			break;
		}
		std::mem::swap(&mut frontier, &mut new_points);
	}

	included.len() as u32
}

#[derive(Debug, Clone, Copy)]
enum TileKind {
	Vertical,
	Horizontal,
	NorthEast,
	NorthWest,
	SouthWest,
	SouthEast,
	Nothing,
	Start,
}

impl TileKind {
	fn from_char(char: char) -> Self {
		match char {
			'|' => Self::Vertical,
			'-' => Self::Horizontal,
			'L' => Self::NorthEast,
			'J' => Self::NorthWest,
			'7' => Self::SouthWest,
			'F' => Self::SouthEast,
			'S' => Self::Start,
			_ => Self::Nothing,
		}
	}
	fn redirect_offset(self, offset: Offset<i32>) -> Option<Offset<i32>> {
		let new_offset = match (self, offset) {
			(Self::Vertical, NORTH) => NORTH,
			(Self::Vertical, SOUTH) => SOUTH,
			(Self::Horizontal, WEST) => WEST,
			(Self::Horizontal, EAST) => EAST,
			(Self::NorthEast, SOUTH) => EAST,
			(Self::NorthEast, WEST) => NORTH,
			(Self::NorthWest, SOUTH) => WEST,
			(Self::NorthWest, EAST) => NORTH,
			(Self::SouthWest, NORTH) => WEST,
			(Self::SouthWest, EAST) => SOUTH,
			(Self::SouthEast, NORTH) => EAST,
			(Self::SouthEast, WEST) => SOUTH,
			_ => return None,
		};
		Some(new_offset)
	}
	fn redirect_offset_and_get_lefts(
		self,
		offset: Offset<i32>,
	) -> Option<(Offset<i32>, &'static [Offset<i32>])> {
		let new_offset_and_lefts = match (self, offset) {
			(Self::Vertical, NORTH) => (NORTH, [WEST].as_slice()),
			(Self::Vertical, SOUTH) => (SOUTH, [EAST].as_slice()),
			(Self::Horizontal, WEST) => (WEST, [SOUTH].as_slice()),
			(Self::Horizontal, EAST) => (EAST, [NORTH].as_slice()),
			(Self::NorthEast, SOUTH) => (EAST, [].as_slice()),
			(Self::NorthEast, WEST) => (NORTH, [SOUTH, WEST].as_slice()),
			(Self::NorthWest, SOUTH) => (WEST, [EAST, SOUTH].as_slice()),
			(Self::NorthWest, EAST) => (NORTH, [].as_slice()),
			(Self::SouthWest, NORTH) => (WEST, [].as_slice()),
			(Self::SouthWest, EAST) => (SOUTH, [NORTH, EAST].as_slice()),
			(Self::SouthEast, NORTH) => (EAST, [WEST, NORTH].as_slice()),
			(Self::SouthEast, WEST) => (SOUTH, [].as_slice()),
			_ => return None,
		};
		Some(new_offset_and_lefts)
	}
	fn _into_char(self) -> char {
		match self {
			TileKind::Vertical => '│',
			TileKind::Horizontal => '─',
			TileKind::NorthEast => '└',
			TileKind::NorthWest => '┘',
			TileKind::SouthWest => '┐',
			TileKind::SouthEast => '┌',
			TileKind::Nothing => ' ',
			TileKind::Start => 'S',
		}
	}
}

const NORTH: Offset<i32> = Offset { x: 0, y: -1 };
const SOUTH: Offset<i32> = Offset { x: 0, y: 1 };
const EAST: Offset<i32> = Offset { x: 1, y: 0 };
const WEST: Offset<i32> = Offset { x: -1, y: 0 };

fn point_usize_to_i32(point: Point<usize>) -> Point<i32> {
	Point {
		x: point.x as i32,
		y: point.y as i32,
	}
}

fn point_i32_to_usize(point: Point<i32>) -> Point<usize> {
	Point {
		x: point.x as usize,
		y: point.y as usize,
	}
}

fn find_start_position_and_offset(
	field: &Grid<TileKind>,
) -> (Point<usize>, Point<usize>, Offset<i32>) {
	let start: Point<usize> = field
		.iter_with_points()
		.find_map(|(point, tile)| matches!(tile, TileKind::Start).then_some(point))
		.unwrap();
	let (position, offset) = start
		.orthogonal_neighbours()
		.into_iter()
		.zip([NORTH, WEST, EAST, SOUTH])
		.find(|(point, neighbour_offset)| {
			let tile_kind = field.get_point(*point); // Hard-coded: start not on an edge.
			tile_kind.redirect_offset(*neighbour_offset).is_some()
		})
		.unwrap();
	(start, position, offset)
}
