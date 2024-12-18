use std::{array, fmt::Write};

fn main() {
	shared::print_answers(17, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> String {
	let (registers, instructions) = input.split_once("\n\n").unwrap();
	let mut registers = registers.lines();
	let mut registry: [u64; 3] = array::from_fn(|_| {
		registers.next().unwrap()["Register A: ".len()..]
			.parse()
			.unwrap()
	});
	let instructions = instructions["Program: ".len()..]
		.trim()
		.split(',')
		.map(|n| n.parse().unwrap())
		.collect::<Vec<_>>();

	let mut output = String::new();
	let mut instruction_pointer = 0;
	while let (Some(&instruction), Some(&operand)) = (
		instructions.get(instruction_pointer),
		instructions.get(instruction_pointer + 1),
	) {
		let instruction = Instruction::from_byte(instruction);
		match instruction.apply(operand, &mut registry) {
			OperationResult::Normal => (),
			OperationResult::JumpTo(i) => {
				instruction_pointer = i as usize;
				continue;
			}
			OperationResult::Output(n) => output.write_fmt(format_args!("{n},")).unwrap(),
		}
		instruction_pointer += 2
	}

	output.pop();
	output
}

fn get_answer_2(input: &str) -> String {
	let (_, instructions) = input.split_once("\n\n").unwrap();
	let instructions = instructions["Program: ".len()..]
		.trim()
		.split(',')
		.rev()
		.map(|n| n.parse().unwrap())
		.collect::<Vec<_>>();

	let result = find_three_bits(&instructions, 0).unwrap();
	format!("{:?}", result)
}

#[derive(Debug)]
enum Instruction {
	Adv,
	Bxl,
	Bst,
	Jnz,
	Bxc,
	Out,
	Bdv,
	Cdv,
}

impl Instruction {
	fn from_byte(byte: u8) -> Self {
		match byte {
			0 => Self::Adv,
			1 => Self::Bxl,
			2 => Self::Bst,
			3 => Self::Jnz,
			4 => Self::Bxc,
			5 => Self::Out,
			6 => Self::Bdv,
			7 => Self::Cdv,
			_ => panic!(),
		}
	}
	fn apply(self, operand: u8, registry: &mut [u64; 3]) -> OperationResult {
		match self {
			Self::Adv => registry[0] /= 2_u64.pow(combo_operand(operand, registry)),
			Self::Bxl => registry[1] ^= operand as u64,
			Self::Bst => registry[1] = (combo_operand(operand, registry) % 8) as u64,
			Self::Jnz => {
				if registry[0] != 0 {
					return OperationResult::JumpTo(operand);
				}
			}
			Self::Bxc => registry[1] ^= registry[2],
			Self::Out => {
				return OperationResult::Output((combo_operand(operand, registry) % 8) as u8)
			}
			Self::Bdv => registry[1] = registry[0] / 2_u64.pow(combo_operand(operand, registry)),
			Self::Cdv => registry[2] = registry[0] / 2_u64.pow(combo_operand(operand, registry)),
		};
		OperationResult::Normal
	}
}

enum OperationResult {
	Normal,
	JumpTo(u8),
	Output(u8),
}

fn combo_operand(operand: u8, registry: &mut [u64; 3]) -> u32 {
	match operand {
		0 => 0,
		1 => 1,
		2 => 2,
		3 => 3,
		4 => registry[0] as u32,
		5 => registry[1] as u32,
		6 => registry[2] as u32,
		_ => panic!(),
	}
}

/// I have completely hardcoded my program. Maybe that wasn't necessary but it seems considerably easier.
fn find_three_bits(instructions: &[u8], value: u64) -> Result<u64, ()> {
	if instructions.is_empty() {
		return Ok(value);
	}
	let target = *instructions.first().unwrap() as u64;
	let value = value << 3;
	for n in 0..8 {
		let value = value + n;
		if validate_n(value, n, target) {
			if let Ok(value) = find_three_bits(&instructions[1..], value) {
				return Ok(value);
			}
		}
	}
	Err(())
}

fn validate_n(value: u64, n: u64, target: u64) -> bool {
	let move_left = n ^ 0b111;
	let shifted_window = (value >> move_left) % 8;
	n ^ shifted_window == target
}
