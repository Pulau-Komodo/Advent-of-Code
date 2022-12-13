fn main() {
	shared::print_answers(5, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	JumpInstructions::from_str(input).follow_until_done(|_| 1)
}

fn get_answer_2(input: &str) -> u32 {
	JumpInstructions::from_str(input).follow_until_done(|jump| if jump >= 3 { -1 } else { 1 })
}

struct JumpInstructions {
	instructions: Vec<i16>,
}

impl JumpInstructions {
	fn from_str(str: &str) -> Self {
		let instructions = str.lines().map(|line| line.parse().unwrap()).collect();
		Self { instructions }
	}
	fn follow_until_done<F>(mut self, add: F) -> u32
	where
		F: Fn(i16) -> i16,
	{
		let mut position = 0;
		let mut steps = 0;
		while position < self.instructions.len() {
			steps += 1;
			let jump = self.instructions.get_mut(position).unwrap();
			position += *jump as usize;
			*jump += add(*jump);
		}
		steps
	}
}
