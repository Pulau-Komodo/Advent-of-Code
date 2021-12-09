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
	fn basin_size(
		&self,
		point: Point,
		outer_edge: &mut Vec<Point>,
		new_outer_edge: &mut Vec<Point>,
		basin: &mut std::collections::HashSet<Point>,
	) -> usize {
		outer_edge.clear();
		outer_edge.push(point);
		basin.clear();
		basin.insert(point);
		loop {
			new_outer_edge.clear();
			for edge_point in outer_edge.iter() {
				for nearby in self.find_nearby_non_9(*edge_point) {
					if basin.insert(nearby) {
						new_outer_edge.push(nearby);
					}
				}
			}
			if new_outer_edge.is_empty() {
				break basin.len();
			}
			*outer_edge = new_outer_edge.clone();
		}
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
		let mut largest = [0; 3];
		let mut outer_edge = Vec::new();
		let mut new_outer_edge = Vec::new();
		let mut basin = std::collections::HashSet::new();
		for minimum in self.local_minima() {
			let size = self.basin_size(minimum, &mut outer_edge, &mut new_outer_edge, &mut basin);
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
