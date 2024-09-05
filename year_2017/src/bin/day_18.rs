use std::collections::VecDeque;

use shared::SmallMap;

fn main() {
	shared::print_answers(18, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> i64 {
	let instructions: Vec<_> = input.lines().map(Instruction::from_str).collect();

	let mut frequency = None;
	let mut program = Program::new(&instructions, 0);
	loop {
		match program.progress() {
			ProgressOutcome::Send(value) => frequency = Some(value),
			ProgressOutcome::Stop => return frequency.unwrap(),
			ProgressOutcome::Nothing => (),
		}
	}
}

fn get_answer_2(input: &str) -> i64 {
	let instructions: Vec<_> = input.lines().map(Instruction::from_str).collect();

	let mut programs = [
		Program::new(&instructions, 0),
		Program::new(&instructions, 1),
	];
	let mut current_program = 0;
	let mut sent_count = 0;
	loop {
		match programs[current_program].progress() {
			ProgressOutcome::Send(value) => {
				programs[(current_program + 1) % 2].add_to_queue(value);
				if current_program == 1 {
					sent_count += 1;
				}
			}
			ProgressOutcome::Stop => {
				current_program = (current_program + 1) % 2;
				if !programs[current_program].can_go() {
					return sent_count;
				}
			}
			ProgressOutcome::Nothing => {}
		}
	}
}

#[derive(Clone, Copy)]
enum Instruction {
	Send(Value),
	Receive(u8),
	Set(u8, Value),
	Add(u8, Value),
	Mul(u8, Value),
	Mod(u8, Value),
	Jump(Value, Value),
}

impl Instruction {
	fn from_str(str: &str) -> Self {
		let (instruction, rest) = str.split_once(' ').unwrap();
		match instruction {
			"snd" => Self::Send(Value::from_str(rest)),
			"rcv" => Self::Receive(get_register(rest)),
			"set" => {
				let (register, value) = split_into_register_and_value(rest);
				Self::Set(register, value)
			}
			"add" => {
				let (register, value) = split_into_register_and_value(rest);
				Self::Add(register, value)
			}
			"mul" => {
				let (register, value) = split_into_register_and_value(rest);
				Self::Mul(register, value)
			}
			"mod" => {
				let (register, value) = split_into_register_and_value(rest);
				Self::Mod(register, value)
			}
			"jgz" => {
				let (condition, jump_size) = rest.split_once(' ').unwrap();
				Self::Jump(Value::from_str(condition), Value::from_str(jump_size))
			}
			_ => panic!(),
		}
	}
	fn execute(self, registry: &mut SmallMap<u8, i64>) -> Outcome {
		match self {
			Self::Send(value) => {
				let value = value.get(registry);
				return Outcome::Send(value);
			}
			Self::Receive(register) => {
				return Outcome::Receive(register);
			}
			Self::Set(register, value) => {
				let value = value.get(registry);
				registry.insert(register, value);
			}
			Self::Add(register, value) => {
				let value = value.get(registry);
				*registry.get_mut_or_insert(register, 0) += value;
			}
			Self::Mul(register, value) => {
				let value = value.get(registry);
				*registry.get_mut_or_insert(register, 0) *= value;
			}
			Self::Mod(register, value) => {
				let value = value.get(registry);
				let value_x = registry.get_mut_or_insert(register, 0);
				*value_x = value_x.rem_euclid(value);
			}
			Self::Jump(condition, jump_size) => {
				if condition.get(registry) > 0 {
					let jump_size = jump_size.get(registry);
					return Outcome::Jump(jump_size);
				}
			}
		}
		Outcome::Nothing
	}
}

impl std::fmt::Debug for Instruction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Send(value) => f.debug_tuple("Send").field(value).finish(),
			Self::Receive(register) => f
				.debug_tuple("Receive")
				.field(&char::from(*register))
				.finish(),
			Self::Set(register, value) => f
				.debug_tuple("Set")
				.field(&char::from(*register))
				.field(value)
				.finish(),
			Self::Add(register, value) => f
				.debug_tuple("Add")
				.field(&char::from(*register))
				.field(value)
				.finish(),
			Self::Mul(register, value) => f
				.debug_tuple("Mul")
				.field(&char::from(*register))
				.field(value)
				.finish(),
			Self::Mod(register, value) => f
				.debug_tuple("Mod")
				.field(&char::from(*register))
				.field(value)
				.finish(),
			Self::Jump(condition, jump_size) => f
				.debug_tuple("Jump")
				.field(condition)
				.field(jump_size)
				.finish(),
		}
	}
}

#[derive(Debug)]
enum Outcome {
	Send(i64),
	Receive(u8),
	Jump(i64),
	Nothing,
}

fn get_register(str: &str) -> u8 {
	let register = str.as_bytes()[0];
	assert!(register.is_ascii_lowercase());
	register
}

fn split_into_register_and_value(str: &str) -> (u8, Value) {
	let (register, value) = str.split_once(' ').unwrap();
	(get_register(register), Value::from_str(value))
}

#[derive(Clone, Copy)]
enum Value {
	Constant(i64),
	Register(u8),
}

impl Value {
	fn from_str(str: &str) -> Self {
		if let Ok(num) = str.parse() {
			Self::Constant(num)
		} else {
			Self::Register(get_register(str))
		}
	}
	fn get(&self, registry: &SmallMap<u8, i64>) -> i64 {
		match self {
			Self::Constant(num) => *num,
			Self::Register(register) => registry.get(register).copied().unwrap_or(0),
		}
	}
}

impl std::fmt::Debug for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Constant(value) => f.debug_tuple("Constant").field(value).finish(),
			Self::Register(register) => f
				.debug_tuple("Register")
				.field(&char::from(*register))
				.finish(),
		}
	}
}

struct Program<'l> {
	instructions: &'l [Instruction],
	position: i64,
	queue: VecDeque<i64>,
	registry: SmallMap<u8, i64>,
	is_terminated: bool,
	is_waiting: bool,
}

impl<'l> Program<'l> {
	fn new(instructions: &'l [Instruction], p: i64) -> Self {
		let mut registry = SmallMap::new();
		registry.insert(b'p', p);
		Self {
			instructions,
			position: 0,
			queue: VecDeque::new(),
			registry,
			is_terminated: false,
			is_waiting: false,
		}
	}
	fn add_to_queue(&mut self, value: i64) {
		self.queue.push_back(value);
		self.is_waiting = false;
	}
	fn can_go(&self) -> bool {
		!self.is_terminated && !self.is_waiting
	}
	fn progress(&mut self) -> ProgressOutcome {
		let Some(instruction) = self
			.position
			.try_into()
			.ok()
			.and_then(|index: usize| self.instructions.get(index))
		else {
			self.is_terminated = true;
			return ProgressOutcome::Stop;
		};
		let outcome = instruction.execute(&mut self.registry);
		match outcome {
			Outcome::Receive(register) => {
				if let Some(value) = self.queue.pop_front() {
					self.registry.insert(register, value);
					self.position += 1;
				} else {
					self.is_waiting = true;
					return ProgressOutcome::Stop;
				}
			}
			Outcome::Send(value) => {
				self.position += 1;
				return ProgressOutcome::Send(value);
			}
			Outcome::Jump(jump) => self.position += jump,
			Outcome::Nothing => self.position += 1,
		}
		ProgressOutcome::Nothing
	}
}

#[derive(Debug)]
enum ProgressOutcome {
	Send(i64),
	Stop,
	Nothing,
}
