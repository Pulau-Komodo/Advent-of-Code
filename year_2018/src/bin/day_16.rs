use std::array;

fn main() {
	shared::print_answers(16, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let (samples, _) = input.split_once("\n\n\n").unwrap();
	let ops = Operation::one_of_each();
	samples
		.split("\n\n")
		.map(Sample::from_str)
		.filter(|sample| {
			let mut op_count = 0;
			for op in ops {
				if op.apply(sample.operation, sample.before) == sample.after {
					op_count += 1;
					if op_count == 3 {
						return true;
					}
				}
			}
			false
		})
		.count()
}

fn get_answer_2(input: &str) -> usize {
	let (samples, program) = input.split_once("\n\n\n\n").unwrap();

	let possible_ops = find_possible_ops(samples);
	let ops = identify_ops(possible_ops);

	let registry = run_program(program, ops);

	registry[0] as usize
}

fn find_possible_ops(samples: &str) -> Vec<(u16, Vec<Operation>)> {
	let ops = Operation::one_of_each();
	samples
		.split("\n\n")
		.map(Sample::from_str)
		.map(|sample| {
			(
				sample.operation[0],
				ops.into_iter()
					.filter(move |op| op.apply(sample.operation, sample.before) == sample.after)
					.collect(),
			)
		})
		.collect()
}

fn identify_ops(mut possible_ops: Vec<(u16, Vec<Operation>)>) -> [Operation; 16] {
	// This will not resolve all resolveable scenarios, but it was enough for the puzzle input. You could go full Sudoku to solve more complex cases.
	let mut identified_ops = [None; 16];

	loop {
		let Some((op_code, op_list)) = possible_ops.iter().find(|(_, op_list)| op_list.len() == 1)
		else {
			panic!();
		};
		let op = op_list[0];
		identified_ops[*op_code as usize] = Some(op);
		for (_, list) in &mut possible_ops {
			list.retain(|possible_op| *possible_op != op);
		}
		if identified_ops.iter().all(|op| op.is_some()) {
			break;
		}
	}

	identified_ops.map(|op| op.unwrap())
}

fn run_program(program: &str, ops: [Operation; 16]) -> [u16; 4] {
	let mut registry = [0; 4];

	for line in program.lines().map(|line| -> [u16; 4] {
		let mut nums = line.split(' ').map(|n| n.parse().unwrap());
		array::from_fn(|_| nums.next().unwrap())
	}) {
		let op = ops[line[0] as usize];
		registry = op.apply(line, registry);
	}

	registry
}

struct Sample {
	before: [u16; 4],
	after: [u16; 4],
	operation: [u16; 4],
}

impl Sample {
	fn from_str(str: &str) -> Self {
		let parse = |n: &str| -> u16 { n.parse().unwrap() };
		let mut lines = str.lines();
		let mut before = lines.next().unwrap()[9..19].split(", ").map(parse);
		let before = array::from_fn(|_| before.next().unwrap());
		let mut operation = lines.next().unwrap().split(' ').map(parse);
		let operation = array::from_fn(|_| operation.next().unwrap());
		let mut after = lines.next().unwrap()[9..19].split(", ").map(parse);
		let after = array::from_fn(|_| after.next().unwrap());
		Self {
			before,
			after,
			operation,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
	AddR,
	AddI,
	MulR,
	MulI,
	BanR,
	BanI,
	BorR,
	BorI,
	SetR,
	SetI,
	GtIR,
	GtRI,
	GtRR,
	EqIR,
	EqRI,
	EqRR,
}

impl Operation {
	fn apply(self, operation: [u16; 4], mut data: [u16; 4]) -> [u16; 4] {
		let registers = operation.map(|n| n as usize);
		data[registers[3]] = match self {
			Operation::AddR => data[registers[1]] + data[registers[2]],
			Operation::AddI => data[registers[1]] + operation[2],
			Operation::MulR => data[registers[1]] * data[registers[2]],
			Operation::MulI => data[registers[1]] * operation[2],
			Operation::BanR => data[registers[1]] & data[registers[2]],
			Operation::BanI => data[registers[1]] & operation[2],
			Operation::BorR => data[registers[1]] | data[registers[2]],
			Operation::BorI => data[registers[1]] | operation[2],
			Operation::SetR => data[registers[1]],
			Operation::SetI => operation[1],
			Operation::GtIR => (operation[1] > data[registers[2]]) as u16,
			Operation::GtRI => (data[registers[1]] > operation[2]) as u16,
			Operation::GtRR => (data[registers[1]] > data[registers[2]]) as u16,
			Operation::EqIR => (operation[1] == data[registers[2]]) as u16,
			Operation::EqRI => (data[registers[1]] == operation[2]) as u16,
			Operation::EqRR => (data[registers[1]] == data[registers[2]]) as u16,
		};
		data
	}
	fn one_of_each() -> [Self; 16] {
		[
			Self::AddR,
			Self::AddI,
			Self::MulR,
			Self::MulI,
			Self::BanR,
			Self::BanI,
			Self::BorR,
			Self::BorI,
			Self::SetR,
			Self::SetI,
			Self::GtIR,
			Self::GtRI,
			Self::GtRR,
			Self::EqIR,
			Self::EqRI,
			Self::EqRR,
		]
	}
}
