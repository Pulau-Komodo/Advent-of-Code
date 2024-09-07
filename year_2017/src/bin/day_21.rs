use std::array;

use shared::{Grid, Point};

fn main() {
	shared::print_answers(21, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let (outputs_2, outputs_3) = parse_rules(input);
	let mut grid = Grid::new(
		INITIAL_STATE
			.lines()
			.map(|line| line.chars().map(|char| char == '#')),
	);
	for _ in 0..5 {
		if grid.width() % 2 == 0 {
			next_gen::<2, 3>(&mut grid, &outputs_2);
		} else {
			next_gen::<3, 4>(&mut grid, &outputs_3);
		};
	}
	grid.iter().filter(|on| **on).count()
}

fn get_answer_2(input: &str) -> usize {
	let (outputs_2, outputs_3) = parse_rules(input);
	let mut grid = Grid::new(
		INITIAL_STATE
			.lines()
			.map(|line| line.chars().map(|char| char == '#')),
	);
	for _ in 0..18 {
		if grid.width() % 2 == 0 {
			next_gen::<2, 3>(&mut grid, &outputs_2);
		} else {
			next_gen::<3, 4>(&mut grid, &outputs_3);
		};
	}
	grid.iter().filter(|on| **on).count()
}

const INITIAL_STATE: &str = ".#.
..#
###";

fn next_gen<const SQUARE_SIZE: usize, const OUTPUT_SIZE: usize>(
	grid: &mut Grid<bool>,
	outputs: &[[[bool; OUTPUT_SIZE]; OUTPUT_SIZE]],
) {
	let new_size = if SQUARE_SIZE == 2 {
		grid.width() + grid.width() / 2
	} else {
		grid.width() + grid.width() / 3
	};
	let mut new_grid = Grid::empty(new_size, new_size, false);
	for x in 0..grid.width() / SQUARE_SIZE {
		for y in 0..grid.width() / SQUARE_SIZE {
			let square = array::from_fn(|y_o| {
				array::from_fn(|x_o| {
					grid.get_point(Point::new(x * SQUARE_SIZE + x_o, y * SQUARE_SIZE + y_o))
				})
			});
			let output = outputs[flatten::<SQUARE_SIZE>(square) as usize];
			for (y_o, row) in output.into_iter().enumerate() {
				for (x_o, on) in row.into_iter().enumerate() {
					*new_grid
						.get_point_mut(Point::new(x * OUTPUT_SIZE + x_o, y * OUTPUT_SIZE + y_o)) = on;
				}
			}
		}
	}
	std::mem::swap(grid, &mut new_grid);
}

type Outputs2 = Vec<[[bool; 3]; 3]>;
type Outputs3 = Vec<[[bool; 4]; 4]>;

fn parse_rules(input: &str) -> (Outputs2, Outputs3) {
	const RULES_2_COUNT: usize = 6;
	let rules_2: Vec<_> = input
		.lines()
		.take(RULES_2_COUNT)
		.map(EnhancementRule2::from_line)
		.collect();
	let rules_3: Vec<_> = input
		.lines()
		.skip(RULES_2_COUNT)
		.map(EnhancementRule3::from_line)
		.collect();
	let mut outputs_2 = vec![[[false; 3]; 3]; 16];
	let mut outputs_3 = vec![[[false; 4]; 4]; 512];
	for rule in rules_2 {
		for input in rule.inputs {
			outputs_2[input as usize] = rule.output;
		}
	}
	for rule in rules_3 {
		for input in rule.inputs {
			outputs_3[input as usize] = rule.output;
		}
	}
	(outputs_2, outputs_3)
}

#[derive(Debug, Default, Clone)]
struct EnhancementRule2 {
	inputs: [u16; 8],
	output: [[bool; 3]; 3],
}

impl EnhancementRule2 {
	fn from_line(line: &str) -> Self {
		let (input, output) = line.split_once(" => ").unwrap();
		let input = parse::<2>(input);
		let inputs = eight_orientations(input).map(flatten);
		let output = parse(output);
		Self { inputs, output }
	}
}

#[derive(Debug, Default, Clone)]
struct EnhancementRule3 {
	inputs: [u16; 8],
	output: [[bool; 4]; 4],
}

impl EnhancementRule3 {
	fn from_line(line: &str) -> Self {
		let (input, output) = line.split_once(" => ").unwrap();
		let input = parse::<3>(input);
		let inputs = eight_orientations(input).map(flatten);
		let output = parse(output);
		Self { inputs, output }
	}
}

fn flatten<const SIZE: usize>(square: [[bool; SIZE]; SIZE]) -> u16 {
	let mut output = 0;
	for cell in square.iter().flatten() {
		output <<= 1;
		if *cell {
			output += 1;
		}
	}
	output
}

fn eight_orientations<const SIZE: usize>(
	square: [[bool; SIZE]; SIZE],
) -> [[[bool; SIZE]; SIZE]; 8] {
	let mut orientations = [[[false; SIZE]; SIZE]; 8];
	orientations[0] = square; // Original

	for i in 0..SIZE {
		for j in 0..SIZE {
			orientations[1][i][j] = square[SIZE - j - 1][i]; // 90 degrees
			orientations[2][i][j] = square[SIZE - i - 1][SIZE - j - 1]; // 180 degrees
			orientations[3][i][j] = square[j][SIZE - i - 1]; // 270 degrees

			orientations[4][i][j] = square[j][i]; // Flip
			orientations[5][i][j] = square[SIZE - j - 1][SIZE - i - 1]; // Flip + 90
			orientations[6][i][j] = square[SIZE - i - 1][j]; // Flip + 180
			orientations[7][i][j] = square[i][SIZE - j - 1]; // Flip + 270
		}
	}

	orientations
}

fn parse<const SIZE: usize>(str: &str) -> [[bool; SIZE]; SIZE] {
	let mut lines = str.split('/');
	array::from_fn(|_| {
		let mut chars = lines.next().unwrap().chars();
		array::from_fn(|_| match chars.next().unwrap() {
			'#' => true,
			'.' => false,
			_ => panic!(),
		})
	})
}
