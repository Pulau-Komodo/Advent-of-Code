use shared::IteratorTop;

fn main() {
	shared::print_answers(11, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let mut monkeys: Vec<_> = input.split("\n\n").map(Monkey::from_str).collect();
	let mut monkey = Monkey::default();
	for _ in 0..20 {
		for n in 0..monkeys.len() {
			std::mem::swap(&mut monkeys[n], &mut monkey);
			for (target, item) in monkey.inspect::<3>() {
				monkeys[target].items.push(item);
			}
			std::mem::swap(&mut monkeys[n], &mut monkey);
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
	let mut monkey = Monkey::default();
	let modulus = monkeys
		.iter()
		.map(|monkey| monkey.test_divisible_by)
		.product::<u64>();
	for _ in 0..10000 {
		for n in 0..monkeys.len() {
			std::mem::swap(&mut monkeys[n], &mut monkey);
			for (target, item) in monkey.inspect::<1>() {
				monkeys[target].items.push(item % modulus);
			}
			std::mem::swap(&mut monkeys[n], &mut monkey);
		}
	}
	monkeys
		.into_iter()
		.map(|monkey| monkey.inspections)
		.top::<2>()
		.into_iter()
		.product()
}

#[derive(Default)]
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
	fn inspect<const DIVISOR: u64>(&mut self) -> impl Iterator<Item = (usize, u64)> + '_ {
		for item in &mut self.items {
			*item = self.operation.apply(*item);
			*item /= DIVISOR;
			self.inspections += 1;
		}
		self.items.drain(..).map(|n| {
			if n % self.test_divisible_by == 0 {
				(self.true_target, n)
			} else {
				(self.false_target, n)
			}
		})
	}
}

#[derive(Default)]
enum Operation {
	Add(u64),
	Multiply(u64),
	#[default]
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
