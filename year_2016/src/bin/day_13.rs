use std::collections::HashSet;

fn main() {
	shared::print_answers(13, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let number: u32 = input.parse().unwrap();
	let starting_point = Point { x: 1, y: 1 };
	let target_point = Point { x: 31, y: 39 };
	let mut visited = HashSet::from([starting_point]);
	let mut frontier = Vec::from([starting_point]);
	let mut steps = 0;
	'outer: loop {
		steps += 1;
		let mut new_frontier = Vec::new();
		for point in frontier.drain(..) {
			for neighbour in point.passable_neighbours(number) {
				if visited.insert(neighbour) {
					if neighbour == target_point {
						break 'outer;
					}
					new_frontier.push(neighbour);
				}
			}
		}
		if new_frontier.is_empty() {
			panic!("No path found.");
		}
		frontier.append(&mut new_frontier);
	}
	steps
}

fn get_answer_2(input: &str) -> u32 {
	let number: u32 = input.parse().unwrap();
	let starting_point = Point { x: 1, y: 1 };
	let mut visited = HashSet::from([starting_point]);
	let mut frontier = Vec::from([starting_point]);
	for _ in 0..50 {
		let mut new_frontier = Vec::new();
		for point in frontier.drain(..) {
			for neighbour in point.passable_neighbours(number) {
				if visited.insert(neighbour) {
					new_frontier.push(neighbour);
				}
			}
		}
		frontier.append(&mut new_frontier);
	}
	visited.len() as u32
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
	x: u32,
	y: u32,
}

impl Point {
	fn is_passable(&self, number: u32) -> bool {
		count_on_bits(
			self.x * self.x + 3 * self.x + 2 * self.x * self.y + self.y + self.y * self.y + number,
		) % 2 == 0
	}
	fn neighbours(&self) -> impl Iterator<Item = Self> + '_ {
		[(2, 1), (1, 2), (0, 1), (1, 0)]
			.into_iter()
			.map(|(x, y)| Point {
				x: (self.x + x).saturating_sub(1),
				y: (self.y + y).saturating_sub(1),
			})
	}
	fn passable_neighbours(&self, number: u32) -> impl Iterator<Item = Self> + '_ {
		self.neighbours()
			.filter(move |point| point.is_passable(number))
	}
}

fn count_on_bits(mut n: u32) -> u8 {
	let mut count = 0;
	while n > 0 {
		count += (n & 1) as u8;
		n >>= 1;
	}
	count
}
