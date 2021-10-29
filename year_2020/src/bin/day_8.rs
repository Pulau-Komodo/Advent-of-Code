fn main() {
	shared::print_answers(8, &[part_a, part_b]);
}

enum Instruction {
	Accumulate(i32),
	Jump(i32),
	NoOp(i32),
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
	input
		.lines()
		.map(|line| {
			let (operation, argument) = line.split_once(' ').unwrap();
			let number = argument.parse().unwrap();
			match operation {
				"acc" => Instruction::Accumulate(number),
				"jmp" => Instruction::Jump(number),
				"nop" => Instruction::NoOp(number),
				_ => panic!(),
			}
		})
		.collect()
}

fn execute_instructions(instructions: &[Instruction], change: Option<i32>) -> (i32, bool) {
	let mut instructions = instructions
		.iter()
		.map(|instruction| (instruction, false))
		.collect::<Vec<(&Instruction, bool)>>();
	let mut accumulator = 0;
	let mut position: i32 = 0;
	loop {
		let (instruction, visited) = instructions.get_mut(position as usize).unwrap();
		if *visited {
			return (accumulator, false);
		}
		*visited = true;
		match instruction {
			Instruction::Accumulate(amount) => {
				accumulator += *amount;
				position += 1
			}
			Instruction::Jump(offset) => {
				if change != Some(position) {
					position += *offset
				} else {
					position += 1
				}
			}
			Instruction::NoOp(offset) => {
				if change != Some(position) {
					position += 1
				} else {
					position += *offset
				}
			}
		}
		if position >= instructions.len() as i32 {
			return (accumulator, true);
		}
	}
}

fn part_a(input: &str) -> String {
	let instructions = parse_instructions(input);
	let (result, _) = execute_instructions(&instructions, None);
	format!("{}", result)
}

fn part_b(input: &str) -> String {
	let instructions = parse_instructions(input);
	let result = (0..)
		.find_map(|n| match execute_instructions(&instructions, Some(n)) {
			(_, false) => None,
			(result, true) => Some(result),
		})
		.unwrap();
	format!("{}", result)
}
