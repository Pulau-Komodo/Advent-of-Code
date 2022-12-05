fn main() {
	shared::print_answers(5, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> String {
	let (state, operations) = input.split_once("\r\n\r\n").unwrap();
	let mut state = State::from_str(state);
	for operation in operations.lines().map(Operation::from_str) {
		state.apply_operation(&operation);
	}
	state.report_top_items()
}

fn get_answer_2(input: &str) -> String {
	let (state, operations) = input.split_once("\r\n\r\n").unwrap();
	let mut state = State::from_str(state);
	for operation in operations.lines().map(Operation::from_str) {
		state.apply_operation_v2(&operation);
	}
	state.report_top_items()
}

struct State {
	stacks: Vec<Vec<char>>,
}

impl State {
	fn from_str(str: &str) -> Self {
		let mut lines = str.lines().rev();
		let count = lines.next().unwrap().split_ascii_whitespace().count();
		let mut stacks = Vec::with_capacity(count);
		for _ in 0..count {
			stacks.push(Vec::new());
		}
		for line in lines {
			for (i, char) in line.chars().skip(1).step_by(4).enumerate() {
				if char != ' ' {
					stacks[i].push(char);
				}
			}
		}
		Self { stacks }
	}
	fn apply_operation(&mut self, operation: &Operation) {
		for _ in 0..operation.amount {
			let item = self.stacks[operation.from].pop().unwrap();
			self.stacks[operation.to].push(item);
		}
	}
	fn apply_operation_v2(&mut self, operation: &Operation) {
		let new_len = self.stacks[operation.from].len() - operation.amount;
		let mut to = std::mem::take(&mut self.stacks[operation.to]);
		to.extend(self.stacks[operation.from].drain(new_len..));
		std::mem::swap(&mut self.stacks[operation.to], &mut to);
	}
	fn report_top_items(&self) -> String {
		self.stacks
			.iter()
			.map(|stack| stack.last().unwrap_or(&' '))
			.collect()
	}
}

struct Operation {
	amount: usize,
	from: usize,
	to: usize,
}

impl Operation {
	fn from_str(str: &str) -> Self {
		let mut segments = str.split(' ');
		let amount = segments.nth(1).unwrap().parse().unwrap();
		let from = segments.nth(1).unwrap().parse::<usize>().unwrap() - 1;
		let to = segments.nth(1).unwrap().parse::<usize>().unwrap() - 1;
		Self { amount, from, to }
	}
}
