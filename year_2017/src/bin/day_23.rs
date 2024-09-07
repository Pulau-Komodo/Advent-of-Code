fn main() {
	shared::print_answers(23, &[get_answer_1, get_answer_2, get_answer_1b]);
}

fn get_answer_1(input: &str) -> u32 {
	let instructions: Vec<_> = input.lines().map(Instruction::from_line).collect();

	let mut program = Program::new(instructions, false);
	let mut mul_count = 0;
	loop {
		match program.progress() {
			Some(true) => mul_count += 1,
			Some(false) => (),
			None => break,
		}
	}
	mul_count
}

fn get_answer_1b(_input: &str) -> u32 {
	(79_u32 - 2).pow(2)
}

fn get_answer_2(_input: &str) -> u32 {
	let start = 79 * 100 + 100000;
	let end = start + 17000;
	(start..end)
		.step_by(17)
		.filter(|num| !is_prime(*num))
		.count() as u32
}

fn _get_answer_2b(input: &str) -> u32 {
	// Tried a small optimisation in the assembly but it was not enough to finish before I ran out of patience.
	let mut instructions: Vec<_> = input.lines().map(Instruction::from_line).collect();
	instructions[15] = Instruction::Jump(Value::Constant(1), Value::Constant(10));
	println!("{:?}", instructions);

	let mut program = Program::new(instructions, true);
	while program.progress().is_some() {}

	program.registry[0] as u32
}

struct Program {
	registry: [i32; 8],
	position: i32,
	instructions: Vec<Instruction>,
}

impl Program {
	fn new(instructions: Vec<Instruction>, is_part_2: bool) -> Self {
		let mut registry = [0; 8];
		if is_part_2 {
			registry[0] = 1;
		}
		Self {
			registry,
			position: 0,
			instructions,
		}
	}
	fn progress(&mut self) -> Option<bool> {
		let instruction = self
			.position
			.try_into()
			.ok()
			.and_then(|i: usize| self.instructions.get(i))?;

		let mut multiplied = false;
		match instruction {
			Instruction::Set(register, value) => {
				let value = value.get(&self.registry);
				self.registry[*register] = value;
			}
			Instruction::Sub(register, value) => {
				let value = value.get(&self.registry);
				self.registry[*register] -= value;
			}
			Instruction::Mul(register, value) => {
				let value = value.get(&self.registry);
				self.registry[*register] *= value;
				multiplied = true;
			}
			Instruction::Jump(condition, jump_size) => {
				let condition = condition.get(&self.registry);
				if condition != 0 {
					let jump_size = jump_size.get(&self.registry);
					self.position += jump_size;
					return Some(false);
				}
			}
		}
		self.position += 1;
		Some(multiplied)
	}
}

fn get_register(str: &str) -> usize {
	let register = str.as_bytes()[0];
	assert!(register.is_ascii_lowercase());
	(register - b'a') as usize
}

fn split_into_register_and_value(str: &str) -> (usize, Value) {
	let (register, value) = str.split_once(' ').unwrap();
	(get_register(register), Value::from_str(value))
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
	Set(usize, Value),
	Sub(usize, Value),
	Mul(usize, Value),
	Jump(Value, Value),
}

impl Instruction {
	fn from_line(line: &str) -> Self {
		let (instruction, rest) = line.split_once(' ').unwrap();
		match instruction {
			"set" => {
				let (register, value) = split_into_register_and_value(rest);
				Self::Set(register, value)
			}
			"sub" => {
				let (register, value) = split_into_register_and_value(rest);
				Self::Sub(register, value)
			}
			"mul" => {
				let (register, value) = split_into_register_and_value(rest);
				Self::Mul(register, value)
			}
			"jnz" => {
				let (condition, jump_size) = rest.split_once(' ').unwrap();
				Self::Jump(Value::from_str(condition), Value::from_str(jump_size))
			}
			_ => panic!(),
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Value {
	Constant(i32),
	Register(usize),
}

impl Value {
	fn from_str(str: &str) -> Self {
		if let Ok(num) = str.parse() {
			Self::Constant(num)
		} else {
			Self::Register(get_register(str))
		}
	}
	fn get(&self, registry: &[i32]) -> i32 {
		match self {
			Self::Constant(num) => *num,
			Self::Register(register) => registry[*register],
		}
	}
}

fn is_prime(num: u32) -> bool {
	for divisor in 2..(num as f32).sqrt() as u32 {
		if num % divisor == 0 {
			return false;
		}
	}
	true
}
