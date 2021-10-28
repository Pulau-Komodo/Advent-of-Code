fn main() {
	year_2020::print_answers(3, &[part_a, part_b]);
}

fn count_trees(tree_map: &str, step_x: usize, step_y: usize) -> usize {
	let rows = tree_map.lines().skip(step_y).step_by(step_y);
	let mut x: usize = 0;
	let mut tree_count: usize = 0;
	for row in rows {
		x = (x + step_x) % row.len();
		if row.chars().nth(x) == Some('#') {
			tree_count += 1;
		}
	}
	tree_count
}

fn part_a(input: &str) -> String {
	let tree_count = count_trees(input, 3, 1);
	format!("{}", tree_count)
}

fn part_b(input: &str) -> String {
	const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
	let product = SLOPES
		.iter()
		.map(|slope| count_trees(input, slope.0, slope.1))
		.fold(1, |accumulator, count| accumulator * count as u64);
	format!("{}", product)
}
