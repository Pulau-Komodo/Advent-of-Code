use std::collections::HashMap;

fn main() {
	shared::print_answers(21, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> Number {
	let data = MonkeyData::from_str(input);
	data.resolve_root()
}

fn get_answer_2(input: &str) -> Number {
	let mut data = MonkeyData::from_str(input);
	data.resolve_bottom_up();
	data.resolve_top_down()
}

type Number = i128;

struct MonkeyData<'l> {
	unresolved: Vec<Monkey<'l>>,
	resolved: HashMap<&'l str, Number>,
}

impl<'l> MonkeyData<'l> {
	fn from_str(str: &'l str) -> Self {
		let mut unresolved = Vec::new();
		let mut resolved = HashMap::new();
		for line in str.lines().map(InputType::from_str) {
			match line {
				InputType::Number { name, number } => {
					resolved.insert(name, number);
				}
				InputType::Monkey(monkey) => unresolved.push(monkey),
			}
		}
		Self {
			unresolved,
			resolved,
		}
	}
	fn resolve_root(mut self) -> i128 {
		loop {
			for monkey in &self.unresolved {
				if let [Some(a), Some(b)] =
					monkey.other_monkeys.map(|monkey| self.resolved.get(monkey))
				{
					let value = monkey.operation.perform(*a, *b);
					if monkey.name == "root" {
						return value;
					}
					self.resolved.insert(monkey.name, value);
				}
			}
			self.unresolved
				.retain_mut(|monkey| !self.resolved.contains_key(monkey.name));
		}
	}
	fn resolve_bottom_up(&mut self) {
		self.resolved.remove("humn");
		loop {
			for monkey in &self.unresolved {
				if let [Some(a), Some(b)] =
					monkey.other_monkeys.map(|monkey| self.resolved.get(monkey))
				{
					let result = monkey.operation.perform(*a, *b);
					self.resolved.insert(monkey.name, result);
				}
			}
			let monkey_count = self.unresolved.len();
			self.unresolved
				.retain_mut(|monkey| !self.resolved.contains_key(monkey.name));
			if self.unresolved.len() == monkey_count {
				break;
			}
		}
	}
	fn remove_root(&mut self) -> Monkey<'l> {
		let root = self
			.unresolved
			.iter()
			.position(|monkey| monkey.name == "root")
			.unwrap();
		self.unresolved.swap_remove(root)
	}
	fn find_starting_point(&mut self, root: Monkey<'l>) -> &'l str {
		if let Some(value) = self.resolved.get(root.other_monkeys[0]) {
			self.resolved.insert(root.other_monkeys[1], *value);
			root.other_monkeys[1]
		} else {
			let value = self.resolved.get(root.other_monkeys[1]).unwrap();
			self.resolved.insert(root.other_monkeys[0], *value);
			root.other_monkeys[0]
		}
	}
	fn resolve_top_down(mut self) -> Number {
		let root = self.remove_root();
		let mut to_resolve = self.find_starting_point(root);
		loop {
			let monkey = self
				.unresolved
				.iter()
				.find(|&monkey| monkey.name == to_resolve)
				.unwrap();
			let value = self.resolved.get(monkey.name).unwrap();
			if let Some(other_value) = self.resolved.get(monkey.other_monkeys[0]) {
				let new_value = monkey.operation.reverse_left_known(*other_value, *value);
				self.resolved.insert(monkey.other_monkeys[1], new_value);
				to_resolve = monkey.other_monkeys[1];
			} else {
				let other_value = self.resolved.get(monkey.other_monkeys[1]).unwrap();
				let new_value = monkey.operation.reverse_right_known(*other_value, *value);
				self.resolved.insert(monkey.other_monkeys[0], new_value);
				to_resolve = monkey.other_monkeys[0];
			}
			if let Some(answer) = self.resolved.get("humn") {
				return *answer;
			}
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Operation {
	Add,
	Subtract,
	Multiply,
	Divide,
}

impl Operation {
	fn perform(self, left: Number, right: Number) -> Number {
		match self {
			Self::Add => left + right,
			Self::Subtract => left - right,
			Self::Multiply => left * right,
			Self::Divide => left / right,
		}
	}
	fn reverse_left_known(self, left: Number, outcome: Number) -> Number {
		match self {
			Operation::Add => outcome - left,
			Operation::Subtract => left - outcome,
			Operation::Multiply => outcome / left,
			Operation::Divide => left / outcome,
		}
	}
	fn reverse_right_known(self, right: Number, outcome: Number) -> Number {
		match self {
			Operation::Add => outcome - right,
			Operation::Subtract => outcome + right,
			Operation::Multiply => outcome / right,
			Operation::Divide => outcome * right,
		}
	}
}

#[derive(Debug, Clone)]
struct Monkey<'l> {
	name: &'l str,
	operation: Operation,
	other_monkeys: [&'l str; 2],
}

enum InputType<'l> {
	Number { name: &'l str, number: Number },
	Monkey(Monkey<'l>),
}

impl<'l> InputType<'l> {
	fn from_str(str: &'l str) -> InputType {
		let name = &str[0..4];
		if str[6..].len() != 11 {
			let number = str[6..].parse().unwrap();
			Self::Number { name, number }
		} else {
			let a = &str[6..10];
			let b = &str[13..17];
			let operation = match str.as_bytes()[11] {
				b'+' => Operation::Add,
				b'-' => Operation::Subtract,
				b'*' => Operation::Multiply,
				b'/' => Operation::Divide,
				_ => panic!(),
			};
			let monkey = Monkey {
				name,
				operation,
				other_monkeys: [a, b],
			};
			Self::Monkey(monkey)
		}
	}
}
