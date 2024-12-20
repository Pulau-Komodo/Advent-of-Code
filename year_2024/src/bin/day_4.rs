use shared::{Direction8, Grid, Offset, Point};

fn main() {
	shared::print_answers(4, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let grid = Grid::with_margin_from_str(input, b' ', std::convert::identity);
	let mut count = 0;
	for x_point in grid
		.iter_with_points::<usize>()
		.filter_map(|(point, cell)| (*cell == b'X').then_some(point))
	{
		'dir_loop: for direction in Direction8::each() {
			let mut point = x_point;
			for char in [b'M', b'A', b'S'] {
				point = apply_offset(point, direction.into_offset());
				if grid.get_point(point) != char {
					continue 'dir_loop;
				}
			}
			count += 1;
		}
	}
	count
}

fn get_answer_2(input: &str) -> u32 {
	let grid = Grid::with_margin_from_str(input, b' ', std::convert::identity);
	let mut count = 0;
	for x_point in grid
		.iter_with_points::<usize>()
		.filter_map(|(point, cell)| (*cell == b'A').then_some(point))
	{
		if DIAGONALS.into_iter().all(|diagonal| {
			let chars = diagonal.map(|offset| grid.get_point(apply_offset(x_point, offset)));
			matches!(chars, [b'M', b'S'] | [b'S', b'M'])
		}) {
			count += 1;
		}
	}
	count
}

fn apply_offset(point: Point<usize>, offset: Offset<i32>) -> Point<usize> {
	Point::new(
		(point.x as i32 + offset.x) as usize,
		(point.y as i32 + offset.y) as usize,
	)
}

const DIAGONALS: [[Offset<i32>; 2]; 2] = [
	[Offset::new(1, -1), Offset::new(-1, 1)],
	[Offset::new(1, 1), Offset::new(-1, -1)],
];
