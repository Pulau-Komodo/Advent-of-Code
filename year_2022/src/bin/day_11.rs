use shared::IteratorTop;

fn main() {
	shared::print_answers(11, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let mut monkeys: Vec<_> = input.split("\n\n").map(Monkey::from_str).collect();
	for _ in 0..20 {
		for n in 0..monkeys.len() {
			for (target, mut items) in monkeys[n].inspect::<3>() {
				items.reverse();
				monkeys[target].items.extend(items);
			}
		}
	}
	monkeys
		.into_iter()
		.map(|monkey| monkey.inspections)
		.top::<2>()
		.into_iter()
		.product()
}

fn get_answer_2(input: &str) -> u64 {
	let mut monkeys: Vec<_> = input.split("\n\n").map(Monkey::from_str).collect();
	let modulus = monkeys
		.iter()
		.map(|monkey| monkey.test_divisible_by)
		.product::<u64>();
	for _ in 0..10000 {
		for n in 0..monkeys.len() {
			for (target, mut items) in monkeys[n].inspect::<1>() {
				for item in &mut items {
					*item %= modulus;
				}
				items.reverse();
				monkeys[target].items.extend(items);
			}
		}
	}
	monkeys
		.into_iter()
		.map(|monkey| monkey.inspections)
		.top::<2>()
		.into_iter()
		.product()
}

struct Monkey {
	items: Vec<u64>,
	operation: Operation,
	test_divisible_by: u64,
	true_target: usize,
	false_target: usize,
	inspections: u64,
}

impl Monkey {
	fn from_str(str: &str) -> Self {
		let mut lines = str.lines();
		let items = &lines.nth(1).unwrap()[18..];
		let items = items.split(", ").map(|n| n.parse().unwrap()).collect();
		let operation = Operation::from_str(lines.next().unwrap());
		let test_divisible_by = lines.next().unwrap()[21..].parse().unwrap();
		let true_target = lines.next().unwrap()[29..].parse().unwrap();
		let false_target = lines.next().unwrap()[30..].parse().unwrap();
		Self {
			items,
			operation,
			test_divisible_by,
			true_target,
			false_target,
			inspections: 0,
		}
	}
	fn inspect<const DIVISOR: u64>(&mut self) -> [(usize, Vec<u64>); 2] {
		for item in &mut self.items {
			*item = self.operation.apply(*item);
			*item /= DIVISOR;
			self.inspections += 1;
		}
		let (true_items, false_items) = self
			.items
			.drain(..)
			.partition(|n| n % self.test_divisible_by == 0);
		[
			(self.true_target, true_items),
			(self.false_target, false_items),
		]
	}
}

enum Operation {
	Add(u64),
	Multiply(u64),
	Square,
}

impl Operation {
	fn from_str(str: &str) -> Self {
		let operation = &str[23..];
		let (operator, element) = operation.split_at(1);
		match (operator, &element[1..]) {
			("+", n) => Self::Add(n.parse().unwrap()),
			("*", "old") => Self::Square,
			("*", n) => Self::Multiply(n.parse().unwrap()),
			_ => panic!("Invalid operation."),
		}
	}
	fn apply(&self, item: u64) -> u64 {
		match self {
			Self::Add(n) => item + n,
			Self::Multiply(n) => item * n,
			Self::Square => item * item,
		}
	}
}
