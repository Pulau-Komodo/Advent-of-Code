fn main() {
	shared::print_answers(11, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut octopus_grid = OctopusGrid::from_str(input);
	(0..100).map(|_| octopus_grid.step()).sum()
}

fn get_answer_2(input: &str) -> u32 {
	let mut octopus_grid = OctopusGrid::from_str(input);
	(1..).find(|_| octopus_grid.step() == 100).unwrap() as u32
}

#[derive(Clone, Copy)]
struct Point {
	x: usize,
	y: usize,
}

struct NeighbouringPoints {
	neighbours: [Option<Point>; 8],
	next_item: usize,
}

impl NeighbouringPoints {
	fn new(range: std::ops::Range<usize>, point: Point) -> Self {
		let mut nearby = [None; 8];
		let offset_range = range.start + 1..range.end + 1;
		(0..3)
			.flat_map(|y_offset| (0..3).map(move |x_offset| (x_offset, y_offset)))
			.filter_map(|(x_offset, y_offset)| {
				if offset_range.contains(&(point.y + y_offset))
					&& offset_range.contains(&(point.x + x_offset))
					&& (x_offset != 1 || y_offset != 1)
				{
					Some(Point {
						x: point.x + x_offset - 1,
						y: point.y + y_offset - 1,
					})
				} else {
					None
				}
			})
			.enumerate()
			.for_each(|(i, point)| nearby[i] = Some(point));
		Self {
			neighbours: nearby,
			next_item: 0,
		}
	}
}

impl Iterator for NeighbouringPoints {
	type Item = Point;
	fn next(&mut self) -> Option<Self::Item> {
		if self.next_item >= self.neighbours.len() {
			None
		} else {
			let item = self.neighbours[self.next_item];
			self.next_item += 1;
			item
		}
	}
}

struct OctopusGrid {
	grid: [[u8; 10]; 10],
}

impl OctopusGrid {
	fn from_str(str: &str) -> Self {
		let mut grid = [[0; 10]; 10];
		for (x, (y, char)) in str
			.lines()
			.enumerate()
			.flat_map(|(y, line)| line.chars().map(move |char| (y, char)).enumerate())
		{
			let num = char as u8 - 48; // Parse as digit
			grid[y][x] = num;
		}
		OctopusGrid { grid }
	}
	/// Advances the simulation one step and returns how many flashed
	fn step(&mut self) -> u32 {
		self.increment_all();
		self.trigger_flashes()
	}
	fn increment_all(&mut self) {
		(0..10)
			.flat_map(|y| (0..10).map(move |x| Point { x, y }))
			.for_each(|point| self.grid[point.y][point.x] += 1)
	}
	fn trigger_flashes(&mut self) -> u32 {
		let mut flash_count = 0;
		loop {
			let new_flashes: u32 = (0..10)
				.flat_map(|y| (0..10).map(move |x| Point { x, y }))
				.filter(|&point| {
					let value = &mut self.grid[point.y][point.x];
					if *value > 9 {
						*value = 0;
						self.increment_neighbours(point);
						true
					} else {
						false
					}
				})
				.count() as u32;
			flash_count += new_flashes;
			if new_flashes == 0 {
				break flash_count;
			}
		}
	}
	fn increment_neighbours(&mut self, point: Point) {
		NeighbouringPoints::new(0..10, point)
			.for_each(|neighbour| self.increment_non_zero(neighbour));
	}
	fn increment_non_zero(&mut self, Point { x, y }: Point) {
		let value = &mut self.grid[y][x];
		if *value != 0 {
			*value = value.saturating_add(1);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn print_grid(grid: &OctopusGrid) {
		println!("----------");
		for line in grid.grid {
			println!("{:?}", line);
		}
	}

	#[test]
	fn sample_input() {
		let input = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526";
		let mut octopus_grid = OctopusGrid::from_str(input);
		print_grid(&octopus_grid);
		assert_eq!(octopus_grid.step(), 0);
		print_grid(&octopus_grid);
		assert_eq!(octopus_grid.step(), 35);
		print_grid(&octopus_grid);
	}
}
