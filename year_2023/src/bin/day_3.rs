use shared::{Grid, Offset, Point};

fn main() {
	shared::print_answers(3, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let grid = Grid::with_margin(
		input.lines().map(|line| line.bytes().map(Cell::from_byte)),
		Cell::Empty,
	);
	let mut sum = 0;
	for y in 0..grid.height() {
		let mut number = None::<u32>;
		let mut is_part_number = false;
		for x in 0..grid.width() {
			let point = Point::new(x, y);
			let cell = grid.get_point(point);
			match cell {
				Cell::Digit(digit) => {
					if is_any_symbol(
						&grid,
						[point - Offset::new(0, 1), point + Offset::new(0, 1)],
					) {
						is_part_number = true;
					}
					if let Some(ref mut number) = number {
						*number *= 10;
						*number += digit as u32;
					} else {
						let previous = point - Offset::new(1, 0);
						let neighbours = [
							previous - Offset::new(0, 1),
							previous,
							previous + Offset::new(0, 1),
						];
						if is_any_symbol(&grid, neighbours) {
							is_part_number = true;
						}
						number = Some(digit as u32);
					}
				}
				_ => {
					if let Some(number) = number.take() {
						let neighbours =
							[point - Offset::new(0, 1), point, point + Offset::new(0, 1)];
						if is_any_symbol(&grid, neighbours) {
							is_part_number = true;
						}
						if is_part_number {
							sum += number;
						}
						is_part_number = false;
					}
				}
			}
		}
	}
	sum
}

fn get_answer_2(input: &str) -> u32 {
	let grid = Grid::with_margin(
		input.lines().map(|line| line.bytes().map(Cell::from_byte)),
		Cell::Empty,
	);
	let mut sum = 0;
	let grid_height = grid.size() / grid.width();

	for y in 0..grid_height {
		'cell: for x in 0..grid.width() {
			let point = Point::new(x, y);
			let cell = grid.get_point(point);
			if !matches!(cell, Cell::Symbol(b'*')) {
				continue;
			}
			let mut neighbouring_numbers = Vec::with_capacity(2);
			let up = point - Offset::new(0, 1);
			match get_independent_digits(&grid, up) {
				DigitSearchResult::Two(left, right) => neighbouring_numbers.extend([left, right]),
				DigitSearchResult::One(digit_pos) => neighbouring_numbers.push(digit_pos),
				DigitSearchResult::None => (),
			}
			let left = point - Offset::new(1, 0);
			let right = point + Offset::new(1, 0);
			for neighbour in [left, right] {
				if matches!(grid.get_point(neighbour), Cell::Digit(_)) {
					if neighbouring_numbers.len() == 2 {
						continue 'cell; // Not a gear.
					}
					neighbouring_numbers.push(neighbour);
				}
			}
			let down = point + Offset::new(0, 1);
			match get_independent_digits(&grid, down) {
				DigitSearchResult::Two(left, right) => {
					if neighbouring_numbers.is_empty() {
						neighbouring_numbers.extend([left, right]);
					}
				}
				DigitSearchResult::One(digit_pos) => {
					if neighbouring_numbers.len() == 1 {
						neighbouring_numbers.push(digit_pos)
					}
				}
				DigitSearchResult::None => (),
			}
			if neighbouring_numbers.len() != 2 {
				continue 'cell; // Not a gear.
			}
			sum += neighbouring_numbers
				.into_iter()
				.map(|pos| complete_number(&grid, pos))
				.product::<u32>();
		}
	}
	sum
}

#[derive(Clone, Copy)]
enum Cell {
	Digit(u8),
	Symbol(u8),
	Empty,
}

impl Cell {
	fn from_byte(byte: u8) -> Self {
		match byte {
			b'0'..=b'9' => Cell::Digit(byte - b'0'),
			b'.' => Cell::Empty,
			_ => Cell::Symbol(byte),
		}
	}
}

fn is_any_symbol(grid: &Grid<Cell>, points: impl IntoIterator<Item = Point<usize>>) -> bool {
	points
		.into_iter()
		.any(|point| matches!(grid.get_point(point), Cell::Symbol(_)))
}

enum DigitSearchResult {
	Two(Point<usize>, Point<usize>),
	One(Point<usize>),
	None,
}

fn get_independent_digits(grid: &Grid<Cell>, mid_point: Point<usize>) -> DigitSearchResult {
	if matches!(grid.get_point(mid_point), Cell::Digit(_)) {
		return DigitSearchResult::One(mid_point);
	}
	let left = mid_point - Offset::new(1, 0);
	let right = mid_point + Offset::new(1, 0);
	match (grid.get_point(left), grid.get_point(right)) {
		(Cell::Digit(_), Cell::Digit(_)) => DigitSearchResult::Two(left, right),
		(Cell::Digit(_), _) => DigitSearchResult::One(left),
		(_, Cell::Digit(_)) => DigitSearchResult::One(right),
		_ => DigitSearchResult::None,
	}
}

fn complete_number(grid: &Grid<Cell>, starting_point: Point<usize>) -> u32 {
	let Cell::Digit(digit) = grid.get_point(starting_point) else {
		panic!();
	};
	let mut number = digit as u32;
	let mut x_offset = 1;
	while let Cell::Digit(digit) = grid.get_point(starting_point - Offset::new(x_offset, 0)) {
		number += digit as u32 * 10_u32.pow(x_offset as u32);
		x_offset += 1;
	}
	x_offset = 1;
	while let Cell::Digit(digit) = grid.get_point(starting_point + Offset::new(x_offset, 0)) {
		number *= 10;
		number += digit as u32;
		x_offset += 1;
	}
	number
}
