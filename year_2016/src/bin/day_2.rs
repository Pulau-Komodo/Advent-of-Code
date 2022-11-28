use std::num::NonZeroU8;

fn main() {
	shared::print_answers(2, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> String {
	let mut state = State::default();
	input
		.lines()
		.map(|line| {
			for instruction in line.chars().map(Instruction::from_char) {
				state.apply_instruction(instruction);
			}
			state.get_button().to_string()
		})
		.collect()
}

fn get_answer_2(input: &str) -> String {
	let mut state = StateV2::default();
	input
		.lines()
		.map(|line| {
			for instruction in line.chars().map(Instruction::from_char) {
				state.apply_instruction(instruction);
			}
			state.get_button().unwrap()
		})
		.collect()
}

enum Instruction {
	Up,
	Right,
	Down,
	Left,
}

impl Instruction {
	fn from_char(char: char) -> Self {
		match char {
			'U' => Self::Up,
			'R' => Self::Right,
			'D' => Self::Down,
			'L' => Self::Left,
			_ => panic!("Invalid instruction."),
		}
	}
}

struct State {
	x: u8,
	y: u8,
}

impl State {
	fn apply_instruction(&mut self, instruction: Instruction) {
		match instruction {
			Instruction::Up => self.y = self.y.saturating_sub(1),
			Instruction::Right => self.x = (self.x + 1).min(2),
			Instruction::Down => self.y = (self.y + 1).min(2),
			Instruction::Left => self.x = self.x.saturating_sub(1),
		}
	}
	fn get_button(&self) -> u8 {
		self.x + self.y * 3 + 1
	}
}

impl Default for State {
	fn default() -> Self {
		Self { x: 1, y: 1 }
	}
}

#[rustfmt::skip]
const KEYPAD_V2: [[Option<NonZeroU8>; 7]; 7] = [
	[None, None, None, None, None, None, None],
	[None, None, None, NonZeroU8::new(1), None, None, None],
	[None, None, NonZeroU8::new(2), NonZeroU8::new(3), NonZeroU8::new(4), None, None],
	[None, NonZeroU8::new(5), NonZeroU8::new(6), NonZeroU8::new(7), NonZeroU8::new(8), NonZeroU8::new(9), None],
	[None, None, NonZeroU8::new(10), NonZeroU8::new(11), NonZeroU8::new(12), None, None],
	[None, None, None, NonZeroU8::new(13), None, None, None],
	[None, None, None, None, None, None, None],
];

#[derive(Clone, Copy)]
struct StateV2 {
	x: u8,
	y: u8,
}

impl StateV2 {
	fn apply_instruction(&mut self, instruction: Instruction) {
		let mut new_state = *self;
		match instruction {
			Instruction::Up => new_state.y -= 1,
			Instruction::Right => new_state.x += 1,
			Instruction::Down => new_state.y += 1,
			Instruction::Left => new_state.x -= 1,
		}
		if new_state.get_button().is_some() {
			*self = new_state;
		}
	}
	fn get_button(&self) -> Option<char> {
		Some(
			char::from_digit(
				KEYPAD_V2[self.y as usize][self.x as usize]?.get() as u32,
				16,
			)
			.unwrap()
			.to_ascii_uppercase(),
		)
	}
}

impl Default for StateV2 {
	fn default() -> Self {
		Self { x: 1, y: 3 }
	}
}
