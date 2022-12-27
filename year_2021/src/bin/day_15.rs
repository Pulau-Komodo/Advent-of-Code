fn main() {
	shared::print_answers(15, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u16 {
	let mut grid = NavigationGrid::<100>::from_str(input);
	grid.resolve();
	grid.cumulative_risk_grid[100 - 1][100 - 1]
}

fn get_answer_2(input: &str) -> u16 {
	let mut grid = NavigationGrid::<500>::from_str(input);
	grid.expand(5);
	grid.resolve();
	grid.cumulative_risk_grid[500 - 1][500 - 1]
}

#[derive(Clone, Copy)]
struct Point {
	x: usize,
	y: usize,
}

fn adjacent_points<const START: usize, const END: usize>(
	point: Point,
) -> impl Iterator<Item = Point> {
	let offset_range = START + 1..END + 1;
	const OFFSETS: [(usize, usize); 4] = [(1, 0), (2, 1), (1, 2), (0, 1)];
	OFFSETS.iter().filter_map(move |(x_offset, y_offset)| {
		if offset_range.contains(&(point.y + y_offset))
			&& offset_range.contains(&(point.x + x_offset))
		{
			Some(Point {
				x: point.x + x_offset - 1,
				y: point.y + y_offset - 1,
			})
		} else {
			None
		}
	})
}

struct NavigationGrid<const SIZE: usize> {
	risk_grid: [[u8; SIZE]; SIZE],
	cumulative_risk_grid: Box<[[u16; SIZE]; SIZE]>,
}

impl<const SIZE: usize> NavigationGrid<SIZE> {
	fn from_str(str: &str) -> Self {
		let mut risk_grid = [[0; SIZE]; SIZE];
		for (x, (y, char)) in str
			.lines()
			.enumerate()
			.flat_map(|(y, line)| line.chars().map(move |char| (y, char)).enumerate())
		{
			let num = char as u8 - 48; // Parse as digit
			risk_grid[y][x] = num;
		}
		let mut cumulative_risk_grid = Box::new([[u16::MAX; SIZE]; SIZE]);
		cumulative_risk_grid[0][0] = 0;
		Self {
			risk_grid,
			cumulative_risk_grid,
		}
	}
	fn expand(&mut self, multiplier: usize) {
		let factor = SIZE / multiplier;
		for (meta_x, meta_y) in (0..multiplier)
			.flat_map(|y| (0..multiplier).map(move |x| (x, y)))
			.filter(|&coords| coords != (0, 0))
		{
			for point in (0..factor).flat_map(|y| (0..factor).map(move |x| Point { x, y })) {
				let offset_point = Point {
					x: point.x + meta_x * factor,
					y: point.y + meta_y * factor,
				};
				*self.get_risk(offset_point) =
					(*self.get_risk(point) + meta_y as u8 + meta_x as u8) % 9;
				if *self.get_risk(offset_point) == 0 {
					*self.get_risk(offset_point) = 9;
				}
			}
		}
	}
	fn resolve(&mut self) {
		let mut current_points = vec![Point { x: 0, y: 0 }];
		let mut new_current_points = Vec::new();
		loop {
			for point in current_points {
				let value = *self.get_cumulative_risk(point);
				for adjacent in adjacent_points::<0, SIZE>(point) {
					let added_risk = *self.get_risk(adjacent) as u16;
					if value + added_risk < *self.get_cumulative_risk(adjacent) {
						*self.get_cumulative_risk(adjacent) = value + added_risk;
						new_current_points.push(adjacent);
					}
				}
			}
			if new_current_points.is_empty() {
				break;
			} else {
				current_points = new_current_points;
				new_current_points = Vec::new();
			}
		}
	}
	fn get_risk(&mut self, Point { x, y }: Point) -> &mut u8 {
		&mut self.risk_grid[y][x]
	}
	fn get_cumulative_risk(&mut self, Point { x, y }: Point) -> &mut u16 {
		&mut self.cumulative_risk_grid[y][x]
	}
}
