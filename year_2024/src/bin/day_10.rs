use shared::{Grid, Point};

fn main() {
	shared::print_answers(10, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let grid = Grid::with_margin(
		input
			.lines()
			.map(|line| line.bytes().map(|byte| byte - b'0')),
		b'.',
	);
	grid.iter_with_points()
		.filter_map(|(point, height)| (*height == 0).then_some(point))
		.map(|point| count_paths(&grid, point, false))
		.sum::<usize>() as u32
}

fn get_answer_2(input: &str) -> u32 {
	let grid = Grid::with_margin(
		input
			.lines()
			.map(|line| line.bytes().map(|byte| byte - b'0')),
		b'.',
	);
	grid.iter_with_points()
		.filter_map(|(point, height)| (*height == 0).then_some(point))
		.map(|point| count_paths(&grid, point, true))
		.sum::<usize>() as u32
}

fn count_paths(grid: &Grid<u8>, start: Point<usize>, allow_repeat_peaks: bool) -> usize {
	let mut frontier = Vec::new();
	frontier.push(start);
	for height in 1..=9 {
		let mut new_frontier = Vec::new();
		for position in frontier.drain(..) {
			for neighbour in position.orthogonal_neighbours() {
				if grid.get_point(neighbour) == height
					&& (allow_repeat_peaks
						|| !new_frontier
							.iter()
							.any(|new_position| neighbour == *new_position))
				{
					new_frontier.push(neighbour);
				}
			}
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
	frontier.len()
}
