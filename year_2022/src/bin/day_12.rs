use std::collections::HashSet;

use shared::{FlatPoint, Grid};

fn main() {
	shared::print_answers(12, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	Heightmap::from_str(input).find_path(|start, position, _| position == start)
}

fn get_answer_2(input: &str) -> u32 {
	Heightmap::from_str(input).find_path(|_, _, height| height == b'a')
}

struct Heightmap {
	grid: Grid<u8>,
	start: FlatPoint,
	destination: FlatPoint,
}

impl Heightmap {
	fn from_str(str: &str) -> Self {
		let mut grid =
			Grid::with_margin(str.lines().map(|line| line.as_bytes().iter().copied()), 0);
		let mut start = FlatPoint::default();
		let mut destination = FlatPoint::default();
		for (index, height) in grid.iter_mut().enumerate() {
			if *height == b'S' {
				start.index = index;
				*height = b'a';
			} else if *height == b'E' {
				destination.index = index;
				*height = b'z';
			}
		}
		Self {
			grid,
			start,
			destination,
		}
	}
	fn find_path<F>(&self, test: F) -> u32
	where
		F: Fn(FlatPoint, FlatPoint, u8) -> bool,
	{
		let mut frontier = Vec::from([(self.destination, b'z')]);
		let mut new_frontier = Vec::new();
		let mut visited = HashSet::with_capacity(self.grid.size());
		visited.insert(self.destination);
		let mut steps = 0;
		'outer: loop {
			steps += 1;
			for (position, height) in frontier.drain(..) {
				for new_position in position.orthogonal_neighbours(self.grid.width()) {
					let new_height = self.grid[new_position];
					if new_height + 1 >= height && !visited.contains(&new_position) {
						if test(self.start, new_position, new_height) {
							dbg!(visited.len());
							break 'outer;
						}
						new_frontier.push((new_position, new_height));
						visited.insert(new_position);
					}
				}
			}
			if new_frontier.is_empty() {
				panic!("Could not find a path after steps: {steps}.");
			}
			std::mem::swap(&mut frontier, &mut new_frontier);
		}
		steps
	}
}
