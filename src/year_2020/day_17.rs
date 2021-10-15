type Grid = Vec<Vec<Vec<Vec<bool>>>>;
type GridRef<'l> = &'l [Vec<Vec<Vec<bool>>>];

#[derive(PartialEq, Clone, Copy)]
struct Position {
	x: usize,
	y: usize,
	z: usize,
	w: usize,
}

fn read_input_and_initialize(input: &str, cycles: usize, four_dimensional: bool) -> Grid {
	let slice = input
		.lines()
		.map(|line| line.chars().map(|char| char == '#').collect::<Vec<_>>())
		.collect::<Vec<_>>();
	let w_length = 1;
	let z_length = 1;
	let y_length = slice.len();
	let x_length = slice.get(0).unwrap().len();
	let w_target_length = if four_dimensional {
		w_length + 2 * cycles
	} else {
		w_length
	};
	let z_target_length = z_length + 2 * cycles;
	let y_target_length = y_length + 2 * cycles;
	let x_target_length = x_length + 2 * cycles;
	(0..w_target_length)
		.map(|w| {
			(0..z_target_length)
				.map(|z| {
					(0..y_target_length)
						.map(|y| {
							(0..x_target_length)
								.map(|x| {
									if w == cycles
										&& z == cycles && (cycles..cycles + y_length).contains(&y)
										&& (cycles..cycles + x_length).contains(&x)
									{
										*slice
											.get(y - cycles)
											.and_then(|row| row.get(x - cycles))
											.unwrap()
									} else {
										false
									}
								})
								.collect::<Vec<_>>()
						})
						.collect::<Vec<_>>()
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

fn cycle(state: Grid) -> Grid {
	state
		.iter()
		.enumerate()
		.map(|(w, grid)| {
			grid.iter()
				.enumerate()
				.map(|(z, slice)| {
					slice
						.iter()
						.enumerate()
						.map(|(y, row)| {
							row.iter()
								.enumerate()
								.map(|(x, _cell)| {
									let position = Position { x, y, z, w };
									get_new_position_state(&state, position)
								})
								.collect::<Vec<_>>()
						})
						.collect::<Vec<_>>()
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

fn get_new_position_state(state: GridRef, position: Position) -> bool {
	let w_range = position.w.saturating_sub(1)..=position.w + 1;
	let z_range = position.z.saturating_sub(1)..=position.z + 1;
	let y_range = position.y.saturating_sub(1)..=position.y + 1;
	let x_range = position.x.saturating_sub(1)..=position.x + 1;
	let position_active = get_position_state(state, position);
	let mut active_count = 0;
	for w in w_range {
		for z in z_range.clone() {
			for y in y_range.clone() {
				for x in x_range.clone() {
					let near_position = Position { x, y, z, w };
					if near_position == position {
						continue;
					} else if get_position_state(state, near_position) {
						active_count += 1;
					}
					if active_count > 3 {
						return false;
					}
				}
			}
		}
	}
	position_active && (2..=3).contains(&active_count) || !position_active && active_count == 3
}

fn get_position_state(state: GridRef, Position { x, y, z, w }: Position) -> bool {
	*state
		.get(w)
		.and_then(|grid| grid.get(z))
		.and_then(|slice| slice.get(y))
		.and_then(|row| row.get(x))
		.unwrap_or(&false)
}

fn count_active(state: GridRef) -> usize {
	state
		.iter()
		.map(|grid| {
			grid.iter()
				.map(|slice| {
					slice
						.iter()
						.map(|row| row.iter().filter(|cell| **cell).count())
						.sum::<usize>()
				})
				.sum::<usize>()
		})
		.sum()
}

pub fn get_answer_1(input: String) -> String {
	let mut state = read_input_and_initialize(&input, 6, false);
	for _ in 0..6 {
		state = cycle(state);
	}
	let count = count_active(&state);
	format!("{}", count)
}

pub fn get_answer_2(input: String) -> String {
	let mut state = read_input_and_initialize(&input, 6, true);
	for _ in 0..6 {
		state = cycle(state);
	}
	let count = count_active(&state);
	format!("{}", count)
}

#[cfg(test)]
mod tests {
	use super::*;
	fn print_grid(state: GridRef) {
		let output: String = state
			.iter()
			.map(|grid| {
				let mut output = grid
					.iter()
					.map(|slice| {
						let mut output = slice
							.iter()
							.map(|row| {
								let mut output = row
									.iter()
									.map(|cell| if *cell { '#' } else { '.' })
									.collect::<String>();
								output.push('\n');
								output
							})
							.collect::<String>();
						output.push('\n');
						output
					})
					.collect::<String>();
				output.push('\n');
				output
			})
			.collect();
		println!("{}", output);
		println!("---");
	}
	#[test]
	fn initialize() {
		let input = std::fs::read_to_string("./input/2020/17.txt").expect("Could not read file");
		println!("{:?}", read_input_and_initialize(&input, 6, false));
	}
	#[test]
	fn sample_input_0() {
		let input = ".#.\n..#\n###";
		let state = read_input_and_initialize(input, 0, false);
		print_grid(&state);
		assert_eq!(count_active(&state), 5);
	}
	#[test]
	fn sample_input_1() {
		let input = ".#.\n..#\n###";
		let mut state = read_input_and_initialize(input, 1, false);
		print_grid(&state);
		state = cycle(state);
		print_grid(&state);
		assert_eq!(count_active(&state), 11);
	}
	#[test]
	fn sample_input_6() {
		let input = ".#.\n..#\n###";
		let mut state = read_input_and_initialize(input, 6, false);
		for _ in 0..6 {
			state = cycle(state);
		}
		//print_grid(&state);
		assert_eq!(count_active(&state), 112);
	}
	#[test]
	fn sample_input_3d() {
		let input = ".#.\n..#\n###";
		let mut state = read_input_and_initialize(input, 6, true);
		for _ in 0..6 {
			state = cycle(state);
		}
		//print_grid(&state);
		assert_eq!(count_active(&state), 848);
	}
}
