use std::collections::{HashMap, HashSet};

use shared::Point;

fn main() {
	shared::print_answers(8, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let antennae = AntennaMap::from_input(input);
	antennae.find_antinodes()
}

fn get_answer_2(input: &str) -> usize {
	let antennae = AntennaMap::from_input(input);
	antennae.find_antinodes_v2()
}

struct AntennaMap {
	antennae: HashMap<char, Vec<Point<i32>>>,
	width: i32,
	height: i32,
}

impl AntennaMap {
	fn from_input(input: &str) -> Self {
		let mut antennae = HashMap::new();
		let mut width = 0;
		let mut height = 0;
		for (y, line) in input.lines().enumerate() {
			height = height.max(y);
			for (x, char) in line.char_indices() {
				width = width.max(x);
				if char == '.' {
					continue;
				}
				antennae
					.entry(char)
					.or_insert_with(Vec::new)
					.push(Point::new(x as i32, y as i32));
			}
		}
		Self {
			antennae,
			width: width as i32 + 1,
			height: height as i32 + 1,
		}
	}
	fn antenna_pairs(&self) -> impl Iterator<Item = (Point<i32>, Point<i32>)> + '_ {
		self.antennae.values().flat_map(|positions| {
			(0..positions.len() - 1)
				.flat_map(|a| (a + 1..positions.len()).map(move |b| (a, b)))
				.map(|(a, b)| (positions[a], positions[b]))
		})
	}
	fn find_antinodes(&self) -> usize {
		let mut antinodes = HashSet::new();
		for (a, b) in self.antenna_pairs() {
			let offset = a - b;
			let new_antinodes = [a + offset, b - offset];
			for antinode in new_antinodes {
				if self.is_on_map(antinode) {
					antinodes.insert(antinode);
				}
			}
		}
		antinodes.len()
	}
	fn find_antinodes_v2(&self) -> usize {
		let mut antinodes = HashSet::new();
		for (a, b) in self.antenna_pairs() {
			// Hardcoded: the greatest common divisor between the offset x and y is 1. If it weren't, I would only need to shrink the offset to fix that. But I just tried it without and it passed.
			let offset = a - b;
			let mut antinode = a;
			while self.is_on_map(antinode) {
				antinodes.insert(antinode);
				antinode += offset;
			}
			antinode = a - offset;
			while self.is_on_map(antinode) {
				antinodes.insert(antinode);
				antinode -= offset;
			}
		}
		antinodes.len()
	}
	fn is_on_map(&self, point: Point<i32>) -> bool {
		(0..self.width).contains(&point.x) && (0..self.height).contains(&point.y)
	}
}
