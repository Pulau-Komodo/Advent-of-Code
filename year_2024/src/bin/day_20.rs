use shared::{Grid, Point};

fn main() {
	shared::print_answers(20, &[get_answer_1, get_answer_2]);
}

const SKIP_SIZE: usize = 100;

fn get_answer_1(input: &str) -> u32 {
	let path = get_path(input);

	let mut count = 0;
	for a in 0..path.len().saturating_sub(SKIP_SIZE) {
		for b in a + SKIP_SIZE + 2..path.len() {
			let point_a = path[a];
			let point_b = path[b];
			if point_a.abs_diff(point_b).component_sum() == 2 {
				count += 1;
			}
		}
	}
	count
}

fn get_answer_2(input: &str) -> u32 {
	let path = get_path(input);

	let mut count = 0;
	for a in 0..path.len().saturating_sub(SKIP_SIZE) {
		for b in a + SKIP_SIZE + 2..path.len() {
			let point_a = path[a];
			let point_b = path[b];
			let distance = point_a.abs_diff(point_b).component_sum() as usize;
			if distance <= 20 && b - a >= SKIP_SIZE + distance {
				count += 1;
			}
		}
	}
	count
}

fn get_path(input: &str) -> Vec<Point<i32>> {
	let mut position = Point::zero();
	let mut end = Point::zero();
	for (point, byte) in input.lines().enumerate().flat_map(|(y, line)| {
		line.bytes()
			.enumerate()
			.map(move |(x, byte)| (Point::new(x as i32, y as i32), byte))
	}) {
		if byte == b'S' {
			position = point;
		} else if byte == b'E' {
			end = point;
		}
	}
	let path_grid = Grid::new(
		input
			.lines()
			.map(|line| line.bytes().map(|byte| byte != b'#')),
	);

	let mut path = Vec::from([position]);
	while position != end {
		position = position
			.orthogonal_neighbours()
			.into_iter()
			.find(|neighbour| {
				path.len()
					.checked_sub(2)
					.map(|i| path[i] != *neighbour)
					.unwrap_or(true)
					&& path_grid.get_point(Point::new(neighbour.x as usize, neighbour.y as usize))
			})
			.unwrap();
		path.push(position);
	}
	path
}
