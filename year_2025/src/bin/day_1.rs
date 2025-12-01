fn main() {
	shared::print_answers(1, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut dial = Dial::new();
	let mut count = 0;
	for line in input.lines() {
		dial.turn(line);
		if dial.position == 0 {
			count += 1;
		}
	}
	count
}

fn get_answer_2(input: &str) -> u32 {
	let mut dial = Dial::new();
	let mut count = 0;
	for line in input.lines() {
		count += dial.turn(line);
	}
	count
}

struct Dial {
	position: i32,
}

impl Dial {
	fn new() -> Self {
		Self { position: 50 }
	}
	fn turn(&mut self, instruction: &str) -> u32 {
		let turn = parse_instruction(instruction);
		let started_at_zero = self.position == 0;

		let mut cycles = turn.unsigned_abs() / 100;
		let turn = turn % 100;
		self.position += turn;

		if !started_at_zero && !(1..100).contains(&self.position) {
			cycles += 1;
		}
		self.position = self.position.rem_euclid(100);
		cycles
	}
}

fn parse_instruction(instruction: &str) -> i32 {
	let instruction = instruction.as_bytes();
	let num = shared::bytes_to_integer::<i32>(&instruction[1..]);
	if instruction[0] == b'L' { -num } else { num }
}
