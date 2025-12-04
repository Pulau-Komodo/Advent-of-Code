use shared::Grid;

fn main() {
	shared::print_answers(4, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let grid = Grid::with_margin_from_str(input, false, |byte| byte == b'@');
	grid.iter_with_points::<usize>()
		.filter(|(point, roll)| {
			**roll
				&& point
					.neighbours()
					.into_iter()
					.filter(|neighbour| grid.get_point(*neighbour))
					.nth(3)
					.is_none()
		})
		.count() as u32
}

fn get_answer_2(input: &str) -> u32 {
	let mut grid = Grid::with_margin_from_str(input, false, |byte| byte == b'@');
	let mut removed = 0;
	loop {
		let mut new_grid = grid.clone();
		let mut removed_any = false;
		for (point, roll) in grid.iter_with_points::<usize>() {
			if !roll {
				continue;
			}
			if point
				.neighbours()
				.into_iter()
				.filter(|neighbour| grid.get_point(*neighbour))
				.nth(3)
				.is_none()
			{
				*new_grid.get_point_mut(point) = false;
				removed_any = true;
				removed += 1;
			}
		}
		if !removed_any {
			break;
		}
		std::mem::swap(&mut grid, &mut new_grid);
	}
	removed
}
