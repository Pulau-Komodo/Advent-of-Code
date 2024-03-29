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
	let mut number = None::<u32>;
	let mut is_part_number = false;
	for (point, &cell) in grid.iter_with_points() {
		match cell {
			Cell::Digit(digit) => {
				if is_symbol(&grid, [point - Offset::Y, point + Offset::Y]) {
					is_part_number = true;
				}
				if let Some(ref mut number) = number {
					*number *= 10;
					*number += digit as u32;
				} else {
					let previous = point - Offset::new(1, 0);
					let neighbours = [previous - Offset::Y, previous, previous + Offset::Y];
					if is_symbol(&grid, neighbours) {
						is_part_number = true;
					}
					number = Some(digit as u32);
				}
			}
			_ => {
				if let Some(number) = number.take() {
					let neighbours = [point - Offset::Y, point, point + Offset::Y];
					if is_symbol(&grid, neighbours) {
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
			let up = point - Offset::Y;
			let left = point - Offset::X;
			let right = point + Offset::X;
			let down = point + Offset::Y;
			for digit_pos in get_independent_digits(&grid, up)
				.into_iter()
				.chain(
					[left, right]
						.into_iter()
						.filter(|point| matches!(grid.get_point(*point), Cell::Digit(_))),
				)
				.chain(
					std::iter::once(())
						.flat_map(|_| get_independent_digits(&grid, down).into_iter()),
				) {
				if neighbouring_numbers.len() >= 2 {
					continue 'cell; // Not a gear.
				}
				neighbouring_numbers.push(digit_pos);
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

fn is_symbol(grid: &Grid<Cell>, points: impl IntoIterator<Item = Point<usize>>) -> bool {
	points
		.into_iter()
		.any(|point| matches!(grid.get_point(point), Cell::Symbol(_)))
}

enum DigitSearchResult {
	Two(Point<usize>, Point<usize>),
	One(Point<usize>),
	None,
}

impl DigitSearchResult {
	fn into_iter(self) -> impl Iterator<Item = Point<usize>> {
		let list = match self {
			Self::Two(left, right) => [Some(left), Some(right)],
			Self::One(point) => [Some(point), None],
			Self::None => [None, None],
		};
		list.into_iter().flatten()
	}
}

fn get_independent_digits(grid: &Grid<Cell>, mid_point: Point<usize>) -> DigitSearchResult {
	if matches!(grid.get_point(mid_point), Cell::Digit(_)) {
		return DigitSearchResult::One(mid_point);
	}
	let left = mid_point - Offset::X;
	let right = mid_point + Offset::X;
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
