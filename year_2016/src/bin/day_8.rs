use std::{collections::VecDeque, fmt::Write};

fn main() {
	shared::print_answers(8, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> Box<dyn std::fmt::Display> {
	let mut screen = Screen::default();
	for instruction in input.lines().map(Instruction::from_str).map(Option::unwrap) {
		screen.apply_instruction(instruction)
	}
	Box::new(screen.count_lit_pixels())
}

fn get_answer_2(input: &str) -> Box<dyn std::fmt::Display> {
	let mut screen = Screen::default();
	for instruction in input.lines().map(Instruction::from_str).map(Option::unwrap) {
		screen.apply_instruction(instruction)
	}
	Box::new(screen)
}

enum Instruction {
	Rectangle { width: u8, height: u8 },
	RotateRow { row: u8, amount: u8 },
	RotateColumn { column: u8, amount: u8 },
}

impl Instruction {
	fn from_str(str: &str) -> Option<Self> {
		let (first, rest) = str.split_once(' ')?;
		if first == "rect" {
			let (width, height) = rest.split_once('x')?;
			Some(Self::Rectangle {
				width: width.parse().ok()?,
				height: height.parse().ok()?,
			})
		} else {
			let (direction, rest) = rest.split_once(' ')?;
			let (_, rest) = rest.split_once('=')?;
			let (id, amount) = rest.split_once(" by ")?;
			let id = id.parse().ok()?;
			let amount = amount.parse().ok()?;
			if direction == "row" {
				Some(Self::RotateRow { row: id, amount })
			} else {
				Some(Self::RotateColumn { column: id, amount })
			}
		}
	}
}

const DISPLAY_WIDTH: usize = 50;
const DISPLAY_HEIGHT: usize = 6;

struct Screen {
	pixels: [VecDeque<bool>; DISPLAY_HEIGHT],
}

impl Screen {
	fn apply_instruction(&mut self, instruction: Instruction) {
		use Instruction::*;
		match instruction {
			Rectangle { width, height } => {
				for (x, y) in (0..width).flat_map(|x| (0..height).map(move |y| (x, y))) {
					self.pixels[y as usize][x as usize] = true;
				}
			}
			RotateRow { row, amount } => {
				self.pixels[row as usize].rotate_right(amount as usize % DISPLAY_WIDTH)
			}
			RotateColumn { column, amount } => {
				let column = column as usize;
				let mut column_state: VecDeque<bool> = (0..DISPLAY_HEIGHT)
					.map(|row| self.pixels[row][column])
					.collect();
				column_state.rotate_right(amount as usize % DISPLAY_HEIGHT);
				for (row, is_on) in column_state.into_iter().enumerate() {
					self.pixels[row][column] = is_on;
				}
			}
		}
	}
	fn count_lit_pixels(&self) -> usize {
		self.pixels
			.iter()
			.flat_map(|row| row.iter())
			.filter(|pixel| **pixel)
			.count()
	}
}

impl Default for Screen {
	fn default() -> Self {
		Self {
			pixels: [(); 6].map(|_| VecDeque::from([false; DISPLAY_WIDTH])),
		}
	}
}

impl std::fmt::Display for Screen {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for pixel in self.pixels.iter().flat_map(|line| {
			['\n']
				.into_iter()
				.chain(line.iter().map(|pixel| if *pixel { '█' } else { ' ' }))
		}) {
			f.write_char(pixel)?
		}
		Ok(())
	}
}
