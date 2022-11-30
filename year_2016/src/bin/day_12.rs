use std::collections::HashMap;

fn main() {
	shared::print_answers(12, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut state = State::default();
	let instructions: Vec<_> = input.lines().map(Instruction::from_str).collect();
	while state.position < instructions.len() {
		state.apply_instruction(&instructions[state.position]);
	}
	*state.registers.get(&'a').unwrap()
}

fn get_answer_2(input: &str) -> u32 {
	let mut state = State::default();
	state.registers.insert('c', 1);
	let instructions: Vec<_> = input.lines().map(Instruction::from_str).collect();
	while state.position < instructions.len() {
		state.apply_instruction(&instructions[state.position]);
	}
	*state.registers.get(&'a').unwrap()
}

enum Value {
	Literal(u32),
	Register(char),
}

impl Value {
	fn from_str(str: &str) -> Self {
		let literal: Result<u32, _> = str.parse();
		match literal {
			Ok(literal) => Value::Literal(literal),
			Err(_) => Value::Register(str.chars().next().unwrap()),
		}
	}
	fn get(&self, registry: &HashMap<char, u32>) -> u32 {
		match self {
			Self::Literal(n) => *n,
			Self::Register(register) => *registry.get(register).unwrap_or(&0),
		}
	}
}

enum Instruction {
	Copy { value: Value, target: char },
	Increase { target: char },
	Decrease { target: char },
	Jump { condition: Value, offset: i8 },
}

impl Instruction {
	fn from_str(str: &str) -> Self {
		let mut sections = str.split_whitespace();
		match sections.next().unwrap() {
			"cpy" => {
				let value = Value::from_str(sections.next().unwrap());
				let target = sections.next().unwrap().chars().next().unwrap();
				Self::Copy { value, target }
			}
			"inc" => {
				let target = sections.next().unwrap().chars().next().unwrap();
				Self::Increase { target }
			}
			"dec" => {
				let target = sections.next().unwrap().chars().next().unwrap();
				Self::Decrease { target }
			}
			"jnz" => {
				let condition = Value::from_str(sections.next().unwrap());
				let offset = sections.next().unwrap().parse().unwrap();
				Self::Jump { condition, offset }
			}
			_ => panic!("Invalid instruction."),
		}
	}
}

#[derive(Default)]
struct State {
	registers: HashMap<char, u32>,
	position: usize,
}

impl State {
	fn apply_instruction(&mut self, instruction: &Instruction) {
		match instruction {
			Instruction::Copy { value, target } => {
				*self.registers.entry(*target).or_insert(0) = value.get(&self.registers)
			}
			Instruction::Increase { target } => *self.registers.entry(*target).or_insert(0) += 1,
			Instruction::Decrease { target } => *self.registers.entry(*target).or_insert(0) -= 1,
			Instruction::Jump { condition, offset } => {
				if condition.get(&self.registers) != 0 {
					if *offset > 0 {
						self.position += *offset as usize;
					} else {
						self.position -= offset.unsigned_abs() as usize;
					};
					return;
				}
			}
		}
		self.position += 1;
	}
}
