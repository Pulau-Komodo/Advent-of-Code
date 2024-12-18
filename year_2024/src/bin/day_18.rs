use shared::{Grid, Offset, Point};

fn main() {
	shared::print_answers(18, &[get_answer_1, get_answer_2]);
}

const GRID_SIZE: usize = 71;
const START: Point<usize> = Point::new(0, 0);
const END: Point<usize> = Point::new(GRID_SIZE - 1, GRID_SIZE - 1);
const MARGIN_OFFSET: Offset<usize> = Offset::new(1, 1);

fn get_answer_1(input: &str) -> String {
	let (path_grid, _) = get_grid_and_bytes(input);

	let path = find_path(&path_grid).expect("Found no path.");
	format!("{path}")
}

fn get_answer_2(input: &str) -> String {
	let (mut path_grid, bytes) = get_grid_and_bytes(input);

	for byte in bytes {
		*path_grid.get_point_mut(byte) = false;
		if find_path(&path_grid).is_none() {
			return format!("{},{}", byte.x - 1, byte.y - 1);
		}
	}
	panic!("Found no path blocker.");
}

fn find_path(path_grid: &Grid<bool>) -> Option<u32> {
	let mut prev_frontier = Vec::new();
	let mut frontier = Vec::from([START + MARGIN_OFFSET]);
	let mut new_frontier = Vec::new();
	for step in 0.. {
		for &point in &frontier {
			if point == END + MARGIN_OFFSET {
				return Some(step);
			}
			for neighbour in point.orthogonal_neighbours() {
				if path_grid.get_point(neighbour)
					&& !new_frontier
						.iter()
						.chain(&frontier)
						.chain(&prev_frontier)
						.any(|point| *point == neighbour)
				{
					new_frontier.push(neighbour);
				}
			}
		}
		if new_frontier.is_empty() {
			return None;
		}
		std::mem::swap(&mut prev_frontier, &mut frontier);
		std::mem::swap(&mut frontier, &mut new_frontier);
		new_frontier.clear();
	}
	unreachable!();
}

/// Places first 1024 bytes.
fn get_grid_and_bytes(input: &str) -> (Grid<bool>, impl Iterator<Item = Point<usize>> + '_) {
	let mut path_grid =
		Grid::with_margin((0..GRID_SIZE).map(|_| (0..GRID_SIZE).map(|_| true)), false);
	let mut bytes = input
		.lines()
		.map(Point::from_comma_separated)
		.map(|point| point + MARGIN_OFFSET);
	for point in (&mut bytes).take(1024) {
		*path_grid.get_point_mut::<usize>(point) = false;
	}
	(path_grid, bytes)
}
