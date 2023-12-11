use std::ops::Range;

use shared::Point;

fn main() {
	shared::print_answers(11, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let galaxies = parse_galaxies(input);
	calculate_distances(&galaxies, 2)
}

fn get_answer_2(input: &str) -> usize {
	let galaxies = parse_galaxies(input);
	calculate_distances(&galaxies, 1_000_000)
}

fn calculate_distances(galaxies: &[Point<usize>], expansion_factor: usize) -> usize {
	let (empty_columns, empty_rows) = find_empty_columns_and_rows(galaxies);
	galaxies
		.iter()
		.enumerate()
		.flat_map(|(index, a)| {
			galaxies[index + 1..]
				.iter()
				.map(|b| calculate_distance(*a, *b, &empty_columns, &empty_rows, expansion_factor))
		})
		.sum()
}

fn parse_galaxies(input: &str) -> Vec<Point<usize>> {
	input
		.lines()
		.enumerate()
		.flat_map(|(y, line)| {
			line.bytes()
				.enumerate()
				.filter_map(move |(x, byte)| (byte == b'#').then_some(Point { x, y }))
		})
		.collect()
}

fn find_empty_columns_and_rows(galaxies: &[Point<usize>]) -> (Vec<usize>, Vec<usize>) {
	let mut non_empty_columns = Vec::new();
	let mut non_empty_rows = Vec::new();
	let mut largest_x = 0;
	let mut largest_y = 0;
	for galaxy in galaxies {
		if non_empty_columns.iter().all(|&column| column != galaxy.x) {
			non_empty_columns.push(galaxy.x);
		}
		if non_empty_rows.iter().all(|&row| row != galaxy.y) {
			non_empty_rows.push(galaxy.y);
		}
		largest_x = largest_x.max(galaxy.x);
		largest_y = largest_y.max(galaxy.y);
	}
	let empty_columns: Vec<_> = (0..largest_x)
		.filter(|&x| non_empty_columns.iter().all(|&column| column != x))
		.collect();
	let empty_rows: Vec<_> = (0..largest_y)
		.filter(|&y| non_empty_rows.iter().all(|&row| row != y))
		.collect();
	(empty_columns, empty_rows)
}

fn count_list_in_range(list: &[usize], range: Range<usize>) -> usize {
	list.iter().filter(|item| range.contains(item)).count()
}

fn taxicab_distance(a: Point<usize>, b: Point<usize>) -> usize {
	a.abs_diff(b).component_sum()
}

fn calculate_distance(
	a: Point<usize>,
	b: Point<usize>,
	empty_columns: &[usize],
	empty_rows: &[usize],
	expansion_factor: usize,
) -> usize {
	let empty_columns = count_list_in_range(empty_columns, a.x.min(b.x)..a.x.max(b.x));
	let empty_rows = count_list_in_range(empty_rows, a.y.min(b.y)..a.y.max(b.y));
	taxicab_distance(a, b) + (expansion_factor - 1) * (empty_rows + empty_columns)
}
