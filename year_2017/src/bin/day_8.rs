use std::collections::HashMap;

fn main() {
	shared::print_answers(8, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> i32 {
	let mut registry = HashMap::new();
	for instruction in input.lines().map(Instruction::from_line) {
		instruction.apply(&mut registry);
	}
	registry.values().max().copied().unwrap_or(0).max(0)
}

fn get_answer_2(input: &str) -> i32 {
	let mut registry = HashMap::new();
	let mut highest = i32::MIN;
	for instruction in input.lines().map(Instruction::from_line) {
		if let Some(value) = instruction.apply(&mut registry) {
			highest = highest.max(value);
		}
	}
	highest
}

struct Instruction<'l> {
	register: &'l str,
	amount: i32,
	condition_register: &'l str,
	comparison: Comparison,
	value: i32,
}

impl<'l> Instruction<'l> {
	fn from_line(line: &'l str) -> Self {
		let mut parts = line.split(' ');
		let register = parts.next().unwrap();
		let operation = Operation::from_str(parts.next().unwrap());
		let mut amount = parts.next().unwrap().parse().unwrap();
		if matches!(operation, Operation::Decrease) {
			amount *= -1;
		}
		let condition_register = parts.nth(1).unwrap();
		let comparison = Comparison::from_str(parts.next().unwrap());
		let value = parts.next().unwrap().parse().unwrap();
		Self {
			register,
			amount,
			condition_register,
			comparison,
			value,
		}
	}
	fn apply(&self, registry: &mut HashMap<&'l str, i32>) -> Option<i32> {
		if self.comparison.apply(
			registry.get(&self.condition_register).copied().unwrap_or(0),
			self.value,
		) {
			let entry = registry.entry(self.register).or_insert(0);
			*entry += self.amount;
			Some(*entry)
		} else {
			None
		}
	}
}

enum Operation {
	Increase,
	Decrease,
}

impl Operation {
	fn from_str(str: &str) -> Self {
		match str {
			"inc" => Self::Increase,
			"dec" => Self::Decrease,
			_ => panic!(),
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Comparison {
	Greater,
	Lesser,
	Equal,
	NotEqual,
	GreaterOrEqual,
	LesserOrEqual,
}

impl Comparison {
	fn from_str(str: &str) -> Self {
		match str {
			">" => Self::Greater,
			"<" => Self::Lesser,
			"==" => Self::Equal,
			"!=" => Self::NotEqual,
			">=" => Self::GreaterOrEqual,
			"<=" => Self::LesserOrEqual,
			_ => panic!(),
		}
	}
	fn apply(self, a: i32, b: i32) -> bool {
		match self {
			Comparison::Greater => a > b,
			Comparison::Lesser => a < b,
			Comparison::Equal => a == b,
			Comparison::NotEqual => a != b,
			Comparison::GreaterOrEqual => a >= b,
			Comparison::LesserOrEqual => a <= b,
		}
	}
}
