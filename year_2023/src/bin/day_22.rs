use std::collections::HashMap;

use shared::{Vec2, Vec3};

fn main() {
	shared::print_answers(22, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let mut bricks = input.lines().map(Brick::from_line).collect::<Vec<_>>();
	bricks.sort_by_key(|brick| brick.start.z);

	let resting_on = find_supporting_bricks(&bricks);
	let undisintegrateable = find_sole_supporters(&resting_on);

	bricks.len() - undisintegrateable.len()
}

// This has some obvious potential for optimisation in memoisation. Some allocations could also be avoided.
fn get_answer_2(input: &str) -> usize {
	let mut bricks = input.lines().map(Brick::from_line).collect::<Vec<_>>();
	bricks.sort_by_key(|brick| brick.start.z);

	let resting_on = find_supporting_bricks(&bricks);
	let sole_supporters = find_sole_supporters(&resting_on);

	let mut disintegrated_count = 0;

	for id in sole_supporters {
		let mut resting_on = resting_on.clone();
		let mut disintegrated = vec![id];
		loop {
			let mut newly_disintegrated = Vec::new();
			for brick in disintegrated.drain(..) {
				for (supported, supporting) in &mut resting_on {
					if supporting.is_empty() {
						continue;
					}
					*supporting = supporting
						.iter_mut()
						.filter_map(|item| (*item != Cell::Brick(brick)).then_some(*item))
						.collect();
					if supporting.is_empty() {
						newly_disintegrated.push(*supported);
						disintegrated_count += 1;
					}
				}
			}
			if newly_disintegrated.is_empty() {
				break;
			}
			std::mem::swap(&mut disintegrated, &mut newly_disintegrated);
		}
	}
	disintegrated_count
}

#[derive(Debug)]
struct Brick {
	start: Vec3<usize>,
	end: Vec3<usize>,
}

impl Brick {
	fn from_line(line: &str) -> Self {
		let (start, end) = line.split_once('~').unwrap();
		let mut start = start.split(',').map(|num| num.parse().unwrap());
		let start = Vec3::<usize>::new(
			start.next().unwrap(),
			start.next().unwrap(),
			start.next().unwrap(),
		);
		let mut end = end.split(',').map(|num| num.parse().unwrap());
		let end = Vec3::<usize>::new(
			end.next().unwrap(),
			end.next().unwrap(),
			end.next().unwrap(),
		);
		assert!(start.x <= end.x); // Hardcoded assumption about the input
		assert!(start.y <= end.y);
		assert!(start.z <= end.z);
		Self { start, end }
	}
	fn footprint(&self) -> impl Iterator<Item = Vec2<usize>> + '_ {
		(self.start.x..=self.end.x)
			.flat_map(|x| (self.start.y..=self.end.y).map(move |y| Vec2::new(x, y)))
	}
	fn iter(&self) -> impl Iterator<Item = Vec3<usize>> + '_ {
		(self.start.x..=self.end.x).flat_map(move |x| {
			(self.start.y..=self.end.y)
				.flat_map(move |y| (self.start.z..=self.end.z).map(move |z| Vec3::new(x, y, z)))
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
	Floor,
	Brick(usize),
	Empty,
}

impl Cell {
	fn _to_char(&self) -> char {
		match self {
			Cell::Floor => '=',
			Cell::Brick(id) => {
				let byte = match (id % 62) as u8 {
					id @ 0..=9 => id + b'0',
					id @ 10..=35 => id - 10 + b'a',
					id @ 36..=61 => id - 36 + b'A',
					_ => unreachable!(),
				};
				char::from(byte)
			}
			Cell::Empty => ' ',
		}
	}
}

#[derive(Debug)]
struct Grid {
	cells: Vec<Vec<Vec<Cell>>>,
}

impl Grid {
	fn new(x: usize, y: usize, z: usize) -> Self {
		let row = Vec::from_iter([Cell::Empty].into_iter().cycle().take(x));
		let slice = Vec::from_iter([row].into_iter().cycle().take(y));
		let mut cells = Vec::from_iter([slice].into_iter().cycle().take(z));
		for x in 0..x {
			for y in 0..y {
				cells[0][y][x] = Cell::Floor;
			}
		}
		Self { cells }
	}
	fn get(&self, x: usize, y: usize, z: usize) -> Cell {
		self.cells[z][y][x]
	}
	fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut Cell {
		&mut self.cells[z][y][x]
	}
	fn _print(&self) {
		for slice in self.cells.iter().rev() {
			let mut display = vec![' '; slice.len()];
			for y in 0..slice.len() {
				for x in (0..slice[0].len()).rev() {
					if display[y] == ' ' {
						display[y] = slice[y][x]._to_char();
					}
				}
			}
			let display: String = display.into_iter().collect();
			println!("{}", display)
		}
	}
	fn _print_slices(&self) {
		for slice in self.cells.iter().rev() {
			for row in slice {
				let row: String = row.iter().map(Cell::_to_char).collect();
				println!("{}", row);
			}
			println!("---");
		}
	}
}

fn find_supporting_bricks(bricks: &[Brick]) -> HashMap<usize, Vec<Cell>> {
	let (max_x, max_y, max_z) = bricks
		.iter()
		.fold((0, 0, 0), |(max_x, max_y, max_z), brick| {
			(
				max_x.max(brick.end.x),
				max_y.max(brick.end.y),
				max_z.max(brick.end.z),
			)
		});
	let mut grid = Grid::new(max_x + 1, max_y + 1, max_z + 1);
	// Brick IDs + the bricks they're resting on
	let mut resting_on = HashMap::<usize, Vec<Cell>>::new();

	for (index, brick) in bricks.iter().enumerate() {
		for z in (0..=brick.start.z).rev() {
			let mut placed = false;
			for cell in brick.footprint() {
				let cell = grid.get(cell.x, cell.y, z);
				if !matches!(cell, Cell::Empty) {
					resting_on.entry(index).or_default().push(cell);
					placed = true;
				}
			}
			if placed {
				let dropped_distance = brick.start.z - z - 1;
				for cell in brick.iter() {
					*grid.get_mut(cell.x, cell.y, cell.z - dropped_distance) = Cell::Brick(index);
				}
				break;
			}
		}
	}

	for resting in resting_on.values_mut() {
		resting.sort_unstable();
		resting.dedup();
	}
	resting_on
}

fn find_sole_supporters(resting_on: &HashMap<usize, Vec<Cell>>) -> Vec<usize> {
	let mut undisintegrateable = Vec::new();

	for supporting in resting_on.values() {
		if supporting.len() == 1 && supporting[0] != Cell::Floor {
			let Cell::Brick(id) = supporting[0] else {
				panic!()
			};
			undisintegrateable.push(id);
		}
	}

	undisintegrateable.sort_unstable();
	undisintegrateable.dedup();
	undisintegrateable
}

#[cfg(test)]
mod tests {
	use super::*;

	const BRICKS: &str = "7,12,15~7,12,15
5,8,257~5,8,290
10,13,100~10,23,100
30,20,150~50,20,150";

	#[test]
	fn test_footprint() {
		let bricks = BRICKS.lines().map(Brick::from_line).collect::<Vec<_>>();
		for brick in bricks {
			println!("{:?}", brick);
			for block in brick.footprint() {
				println!("{:?}", block);
			}
		}
	}
	#[test]
	fn test_brick_blocks() {
		let bricks = BRICKS.lines().map(Brick::from_line).collect::<Vec<_>>();
		for brick in bricks {
			println!("{:?}", brick);
			for block in brick.iter() {
				println!("{:?}", block);
			}
		}
	}
	#[test]
	fn test_vertical_blocks() {
		let brick = Brick::from_line("3,3,3~3,3,10");
		assert_eq!(brick.iter().count(), 8);
	}
	#[test]
	fn test_complete() {
		assert_eq!(0, get_answer_1(""));
		assert_eq!(1, get_answer_1("1,1,4~1,4,4"));
		assert_eq!(
			4,
			get_answer_1(
				"1,1,4~1,4,4
2,1,4~2,4,4
3,1,4~3,4,4
4,1,4~4,4,4"
			)
		);
		assert_eq!(
			5,
			get_answer_1(
				"1,3,7~4,3,7
1,1,4~1,4,4
2,1,4~2,4,4
3,1,4~3,4,4
4,1,4~4,4,4"
			)
		);
		assert_eq!(
			1,
			get_answer_1(
				"3,3,3~3,3,10
0,3,12~3,3,12"
			)
		)
	}
}
