use std::collections::HashMap;

fn main() {
	shared::print_answers(14, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let mut platform = Platform::from_str(input);
	platform.tilt();
	platform.calculate_load()
}

fn get_answer_2(input: &str) -> usize {
	let mut platform = Platform::from_str(input);
	let mut history = HashMap::new();
	let target = 1_000_000_000;
	let mut cycle = 1;
	while cycle < target {
		for _ in 0..4 {
			platform.tilt();
			platform.rotate_cw();
		}
		let old_cycle = *history.entry(platform.clone()).or_insert(cycle);
		let cycle_length = cycle - old_cycle;
		let cycles_left = target - cycle;
		if cycle_length != 0 && cycle_length < cycles_left {
			cycle += cycles_left - cycles_left % cycle_length;
		} else {
			cycle += 1;
		}
	}
	platform.calculate_load()
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Platform {
	cells: Vec<Vec<Cell>>,
}

impl Platform {
	fn from_str(str: &str) -> Self {
		let rows: Vec<Vec<_>> = str
			.lines()
			.map(|line| line.bytes().map(Cell::from_byte).collect())
			.collect();
		let row_length = rows.first().unwrap().len();
		let columns = (0..row_length)
			.map(|column| rows.iter().map(|row| *row.get(column).unwrap()).collect())
			.collect();
		Self { cells: columns }
	}
	fn tilt(&mut self) {
		for column in &mut self.cells {
			let mut empty_spot = 0;
			for y in 0..column.len() {
				match column[y] {
					Cell::CubeRock => empty_spot = y + 1,
					Cell::RoundRock => {
						column[y] = Cell::Empty;
						column[empty_spot] = Cell::RoundRock;
						empty_spot += 1;
					}
					Cell::Empty => (),
				}
			}
		}
	}
	fn rotate_cw(&mut self) {
		let row_length = self.cells.first().unwrap().len();
		let columns = (0..row_length)
			.map(|column| {
				self.cells
					.iter()
					.map(|row| *row.get(column).unwrap())
					.collect()
			})
			.rev()
			.collect();
		self.cells = columns;
	}
	fn calculate_load(&self) -> usize {
		self.cells
			.iter()
			.flat_map(|column| {
				column.iter().rev().enumerate().filter_map(|(index, cell)| {
					matches!(cell, Cell::RoundRock).then_some(index + 1)
				})
			})
			.sum()
	}
	fn _print(&self) {
		let row_length = self.cells.first().unwrap().len();
		let columns: Vec<Vec<_>> = (0..row_length)
			.map(|column| {
				self.cells
					.iter()
					.map(|row| *row.get(column).unwrap())
					.collect()
			})
			.collect();
		for row in &columns {
			for cell in row {
				let char = match cell {
					Cell::RoundRock => 'O',
					Cell::CubeRock => '#',
					Cell::Empty => '.',
				};
				print!("{char}");
			}
			println!();
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
	RoundRock,
	CubeRock,
	Empty,
}

impl Cell {
	fn from_byte(byte: u8) -> Self {
		match byte {
			b'O' => Self::RoundRock,
			b'#' => Self::CubeRock,
			_ => Self::Empty,
		}
	}
}
