fn main() {
	shared::print_answers(9, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let height_map = HeightMap::<100>::from_str(input);
	height_map.sum_risk_levels()
}

fn get_answer_2(input: &str) -> u32 {
	let height_map = HeightMap::<100>::from_str(input);
	let largest_basins = height_map.three_largest_basins();
	largest_basins.iter().map(|&size| size as u32).product()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
	x: usize,
	y: usize,
}

struct HeightMap<const SIZE: usize> {
	map: [[u8; SIZE]; SIZE],
}

impl<const SIZE: usize> HeightMap<SIZE> {
	fn from_str(str: &str) -> Self {
		let mut map = [[0; SIZE]; SIZE];
		for (x, (y, char)) in str
			.lines()
			.enumerate()
			.map(|(y, line)| line.chars().map(move |char| (y, char)).enumerate())
			.flatten()
		{
			let num = char as u8 - 48; // Parse as digit
			map[y][x] = num;
		}
		HeightMap { map }
	}
	fn is_local_minimum(&self, Point { x, y }: Point) -> bool {
		let value = self.map[y][x];
		(y == 0 || self.map[y - 1][x] > value)
			&& (x == SIZE - 1 || self.map[y][x + 1] > value)
			&& (y == SIZE - 1 || self.map[y + 1][x] > value)
			&& (x == 0 || self.map[y][x - 1] > value)
	}
	fn local_minima(&self) -> impl Iterator<Item = Point> + '_ {
		(0..SIZE)
			.map(|y| (0..SIZE).map(move |x| Point { x, y }))
			.flatten()
			.filter(move |&point| self.is_local_minimum(point))
	}
	fn sum_risk_levels(&self) -> u32 {
		self.local_minima()
			.map(|Point { x, y }: Point| self.map[y][x] as u32 + 1)
			.sum()
	}
	fn find_nearby_non_9(&self, Point { x, y }: Point) -> Vec<Point> {
		let value = self.map[y][x];
		if value == 8 {
			return Vec::with_capacity(0);
		}
		let mut output = Vec::with_capacity(4);
		if y > 0 && self.map[y - 1][x] != 9 {
			output.push(Point { y: y - 1, x });
		}
		if x < SIZE - 1 && self.map[y][x + 1] != 9 {
			output.push(Point { y, x: x + 1 });
		}
		if y < SIZE - 1 && self.map[y + 1][x] != 9 {
			output.push(Point { y: y + 1, x });
		}
		if x > 0 && self.map[y][x - 1] != 9 {
			output.push(Point { y, x: x - 1 });
		}
		output
	}
	fn three_largest_basins(&self) -> [usize; 3] {
		let mut basin_finder = BasinFinder::with_height_map(self);
		let mut largest = [0; 3];
		for minimum in self.local_minima() {
			let size = basin_finder.basin_size(minimum);
			if size >= largest[0] {
				largest[2] = largest[1];
				largest[1] = largest[0];
				largest[0] = size;
			} else if size >= largest[1] {
				largest[2] = largest[1];
				largest[1] = size;
			} else if size > largest[2] {
				largest[2] = size;
			}
		}
		largest
	}
}

struct BasinFinder<'l, const SIZE: usize> {
	height_map: &'l HeightMap<SIZE>,
	outer_edge: Vec<Point>,
	new_outer_edge: Vec<Point>,
	basin: std::collections::HashSet<Point>,
}

impl<'l, const SIZE: usize> BasinFinder<'l, SIZE> {
	fn with_height_map(height_map: &'l HeightMap<SIZE>) -> Self {
		Self {
			height_map,
			outer_edge: Vec::new(),
			new_outer_edge: Vec::new(),
			basin: std::collections::HashSet::new(),
		}
	}
	fn basin_size(&mut self, point: Point) -> usize {
		self.outer_edge.clear();
		self.outer_edge.push(point);
		self.basin.clear();
		self.basin.insert(point);
		loop {
			self.new_outer_edge.clear();
			for edge_point in self.outer_edge.iter() {
				for nearby in self.height_map.find_nearby_non_9(*edge_point) {
					if self.basin.insert(nearby) {
						self.new_outer_edge.push(nearby);
					}
				}
			}
			if self.new_outer_edge.is_empty() {
				break self.basin.len();
			}
			self.outer_edge = self.new_outer_edge.clone();
		}
	}
}
