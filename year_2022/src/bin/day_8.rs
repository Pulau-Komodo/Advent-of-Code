fn main() {
	shared::print_answers(8, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	TreeGrid::from_str(input).count_visible_from_edge()
}

fn get_answer_2(input: &str) -> usize {
	TreeGrid::from_str(input).highest_scenic_score()
}

struct TreeGrid {
	trees: Vec<Vec<u8>>,
}

impl TreeGrid {
	fn from_str(str: &str) -> Self {
		let trees: Vec<Vec<_>> = str
			.lines()
			.map(|line| line.as_bytes().iter().map(|char| char - b'0').collect())
			.collect();
		Self { trees }
	}
	fn trees(&self) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
		self.trees.iter().enumerate().flat_map(|(y, column)| {
			column
				.iter()
				.enumerate()
				.map(move |(x, &tree)| (x, y, tree))
		})
	}
	fn trees_on_a_straight_line_from(
		&self,
		(x, y): (usize, usize),
	) -> (
		impl Iterator<Item = u8> + '_,
		impl Iterator<Item = u8> + '_,
		impl Iterator<Item = u8> + '_,
		impl Iterator<Item = u8> + '_,
	) {
		let size = self.trees.len();
		(
			(0..x).rev().map(move |x| self.trees[y][x]),
			(x + 1..size).map(move |x| self.trees[y][x]),
			(0..y).rev().map(move |y| self.trees[y][x]),
			(y + 1..size).map(move |y| self.trees[y][x]),
		)
	}
	fn count_visible_from_edge(&self) -> usize {
		self.trees()
			.filter(|&(x, y, tree)| {
				let mut lines = self.trees_on_a_straight_line_from((x, y));
				lines.0.all(|other_tree| other_tree < tree)
					|| lines.1.all(|other_tree| other_tree < tree)
					|| lines.2.all(|other_tree| other_tree < tree)
					|| lines.3.all(|other_tree| other_tree < tree)
			})
			.count()
	}
	fn highest_scenic_score(&self) -> usize {
		let mut max = 0;
		for (x, y, tree) in self.trees() {
			let (mut left, mut right, mut up, mut down) =
				self.trees_on_a_straight_line_from((x, y));
			let mut left_view = 0;
			left.all(|other_tree| {
				left_view += 1;
				other_tree < tree
			});
			let mut right_view = 0;
			right.all(|other_tree| {
				right_view += 1;
				other_tree < tree
			});
			let mut up_view = 0;
			up.all(|other_tree| {
				up_view += 1;
				other_tree < tree
			});
			let mut down_view = 0;
			down.all(|other_tree| {
				down_view += 1;
				other_tree < tree
			});
			let score = left_view * right_view * up_view * down_view;
			max = max.max(score);
		}
		max
	}
}
