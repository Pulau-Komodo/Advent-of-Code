fn main() {
	shared::print_answers(10, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> Box<dyn std::fmt::Display> {
	let mut state = State::new();
	let mut score = 0;
	let mut instructions = input.lines().map(Instruction::from_str);
	for n in 1..=220 {
		if state.advance_cycle() {
			state.process_instruction(instructions.next().unwrap_or_default());
		}
		if (n - 20) % 40 == 0 {
			score += state.value * n;
		}
	}
	Box::new(score)
}

fn get_answer_2(input: &str) -> Box<dyn std::fmt::Display> {
	let mut state = State::new();
	let mut instructions = input.lines().map(Instruction::from_str);
	let mut pixels = [[' '; 40]; 6];
	for n in 0_usize..240 {
		if state.advance_cycle() {
			state.process_instruction(instructions.next().unwrap_or_default());
		}
		if (state.value - 1..=state.value + 1).contains(&(n as i32 % 40)) {
			pixels[n / 40][n % 40] = '█';
		}
	}
	let output: String = pixels
		.into_iter()
		.flat_map(|line| ['\n'].into_iter().chain(line))
		.collect();
	Box::new(output)
}

#[derive(Default)]
enum Instruction {
	#[default]
	Noop,
	Add(i32),
}

impl Instruction {
	fn from_str(str: &str) -> Self {
		if str == "noop" {
			Self::Noop
		} else {
			let (_, n) = str.split_once(' ').unwrap();
			Self::Add(n.parse().unwrap())
		}
	}
}

struct State {
	value: i32,
	processing_for: u8,
	adding: i32,
}

impl State {
	fn new() -> Self {
		Self {
			value: 1,
			processing_for: 0,
			adding: 0,
		}
	}
	fn advance_cycle(&mut self) -> bool {
		if self.processing_for == 0 {
			self.value += self.adding;
			self.adding = 0;
			true
		} else {
			self.processing_for -= 1;
			false
		}
	}
	fn process_instruction(&mut self, instruction: Instruction) {
		match instruction {
			Instruction::Noop => (),
			Instruction::Add(n) => {
				self.adding = n;
				self.processing_for = 1
			}
		}
	}
}
