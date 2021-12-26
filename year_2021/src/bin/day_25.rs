fn main() {
	shared::print_answers(25, &[get_answer]);
}

fn get_answer(input: &str) -> u32 {
	let mut grid = CucumberGrid::from_str(input);
	let mut count = 1;
	while grid.advance() {
		count += 1;
	}
	count
}

#[derive(Clone, Copy)]
struct Point {
	x: usize,
	y: usize,
}

#[derive(Debug, Clone, Copy)]
enum Space {
	Empty,
	EastFacingCucumber,
	SouthFacingCucumber,
}

impl Space {
	fn from_byte(byte: u8) -> Self {
		match byte {
			b'.' => Self::Empty,
			b'>' => Self::EastFacingCucumber,
			b'v' => Self::SouthFacingCucumber,
			_ => panic!("Invalid input"),
		}
	}
}

struct CucumberGrid {
	grid: [[Space; 139]; 137],
}

impl CucumberGrid {
	fn new() -> Self {
		let grid = [[Space::Empty; 139]; 137];
		Self { grid }
	}
	fn from_str(str: &str) -> Self {
		let mut grid = [[Space::Empty; 139]; 137];
		str.lines().zip(&mut grid).for_each(|(line, row)| {
			line.as_bytes()
				.iter()
				.zip(row)
				.for_each(|(byte, cell)| *cell = Space::from_byte(*byte))
		});
		Self { grid }
	}
	fn advance(&mut self) -> bool {
		let mut moved = false;
		let mut new_grid = Self::new();
		for point in (0..137).flat_map(|y| (0..139).map(move |x| Point { x, y })) {
			let item = self.grid[point.y][point.x];
			match item {
				Space::EastFacingCucumber => {
					if matches!(self.get_point_east_of(point), &mut Space::Empty) {
						*new_grid.get_point_east_of(point) = item;
						moved = true;
					} else {
						*new_grid.get_point(point) = item;
					}
				}
				Space::SouthFacingCucumber => *new_grid.get_point(point) = item,
				Space::Empty => (),
			}
		}
		self.grid = new_grid.grid;
		let mut new_grid = Self::new();
		for point in (0..137).flat_map(|y| (0..139).map(move |x| Point { x, y })) {
			let item = self.grid[point.y][point.x];
			match item {
				Space::SouthFacingCucumber => {
					if matches!(self.get_point_south_of(point), &mut Space::Empty) {
						*new_grid.get_point_south_of(point) = item;
						moved = true;
					} else {
						*new_grid.get_point(point) = item;
					}
				}
				Space::EastFacingCucumber => *new_grid.get_point(point) = item,
				Space::Empty => (),
			}
		}
		self.grid = new_grid.grid;
		moved
	}
	fn get_point_east_of(&mut self, Point { x, y }: Point) -> &mut Space {
		self.get_point(Point {
			x: (x + 1) % 139,
			y,
		})
	}
	fn get_point_south_of(&mut self, Point { x, y }: Point) -> &mut Space {
		self.get_point(Point {
			x,
			y: (y + 1) % 137,
		})
	}
	fn get_point(&mut self, Point { x, y }: Point) -> &mut Space {
		&mut self.grid[y][x]
	}
}
