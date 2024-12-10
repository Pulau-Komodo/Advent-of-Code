use shared::{Grid, Point};

fn main() {
	shared::print_answers(10, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u16 {
	let grid = Grid::with_margin(
		input
			.lines()
			.map(|line| line.bytes().map(|byte| byte - b'0')),
		b'.',
	);
	grid.iter_with_points()
		.filter_map(|(point, height)| (*height == 0).then_some(point))
		.map(|point| count_paths(&grid, point))
		.sum::<usize>() as u16
}

fn get_answer_2(input: &str) -> u16 {
	let grid = Grid::with_margin(
		input
			.lines()
			.map(|line| line.bytes().map(|byte| byte - b'0')),
		b'.',
	);
	let paths = process_paths(&grid);
	grid.iter_with_points::<usize>()
		.filter_map(|(point, height)| (*height == 9).then_some(point))
		.map(|point| paths.get_point(point))
		.sum()
}

fn count_paths(grid: &Grid<u8>, start: Point<usize>) -> usize {
	let mut frontier = Vec::new();
	frontier.push(start);
	for height in 1..=9 {
		let mut new_frontier = Vec::new();
		for position in frontier.drain(..) {
			for neighbour in position.orthogonal_neighbours() {
				if grid.get_point(neighbour) == height
					&& !new_frontier
						.iter()
						.any(|new_position| neighbour == *new_position)
				{
					new_frontier.push(neighbour);
				}
			}
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
	frontier.len()
}

fn process_paths(grid: &Grid<u8>) -> Grid<u16> {
	let mut paths = Grid::new((0..grid.height()).map(|_| (0..grid.width()).map(|_| 0)));
	for peak in grid
		.iter_with_points::<usize>()
		.filter_map(|(point, height)| (*height == 0).then_some(point))
	{
		*paths.get_point_mut(peak) = 1;
	}
	for check_height in 1..=9 {
		for point in grid
			.iter_with_points::<usize>()
			.filter_map(|(point, height)| (*height == check_height).then_some(point))
		{
			let sum: u16 = point
				.orthogonal_neighbours()
				.into_iter()
				.filter(|neighbour| grid.get_point(*neighbour) == check_height - 1)
				.map(|neighbour| paths.get_point(neighbour))
				.sum();
			*paths.get_point_mut(point) = sum;
		}
	}
	paths
}
