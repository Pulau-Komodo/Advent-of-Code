fn main() {
	shared::print_answers(14, &[get_answers]);
}

enum Instruction {
	Mask(Mask),
	Assignment(Assignment),
}

impl Instruction {
	fn from_str(str: &str) -> Self {
		if let Some(mask) = str.strip_prefix("mask = ") {
			Self::Mask(Mask::from_str(mask))
		} else {
			Self::Assignment(Assignment::from_str(str))
		}
	}
}

#[derive(Default)]
struct Mask {
	zeroes: u64,
	ones: u64,
}

impl Mask {
	fn from_str(str: &str) -> Self {
		let mut zeroes = 0;
		let mut ones = 0;
		for char in str.chars() {
			zeroes <<= 1;
			ones <<= 1;
			match char {
				'0' => (),
				'1' => ones += 1,
				'X' => zeroes += 1,
				_ => panic!(),
			}
		}
		Self { zeroes, ones }
	}
	fn apply(&self, value: u64) -> u64 {
		value & self.zeroes | self.ones
	}
}

struct Assignment {
	location: u64,
	value: u64,
}

impl Assignment {
	fn from_str(str: &str) -> Self {
		let stripped = str.strip_prefix("mem[").unwrap();
		let (location, value) = stripped.split_once("] = ").unwrap();
		Self {
			location: location.parse().unwrap(),
			value: value.parse().unwrap(),
		}
	}
}

fn part_1(input: &str) -> u64 {
	let instructions = input.lines().map(Instruction::from_str);
	let mut mask = Mask::default();
	let mut values = std::collections::HashMap::with_capacity(instructions.size_hint().0);
	for instruction in instructions {
		match instruction {
			Instruction::Mask(new_mask) => mask = new_mask,
			Instruction::Assignment(Assignment { location, value }) => {
				values.insert(location, mask.apply(value));
			}
		}
	}
	values.iter().map(|(_location, value)| value).sum::<u64>()
}

enum InstructionV2 {
	Mask(MaskV2),
	Assignment(Assignment),
}

impl InstructionV2 {
	fn from_str(str: &str) -> Self {
		if let Some(mask) = str.strip_prefix("mask = ") {
			Self::Mask(MaskV2::from_str(mask))
		} else {
			Self::Assignment(Assignment::from_str(str))
		}
	}
}

#[derive(Default)]
struct MaskV2 {
	floating: Vec<u64>,
	ones: u64,
}

impl MaskV2 {
	fn from_str(str: &str) -> Self {
		let mut floating = vec![0];
		let mut ones = 0;
		for char in str.chars() {
			for item in floating.iter_mut() {
				*item <<= 1;
			}
			ones <<= 1;
			match char {
				'0' => (),
				'1' => ones += 1,
				'X' => {
					for item in floating.clone() {
						floating.push(item + 1);
					}
				}
				_ => panic!(),
			}
		}
		Self { floating, ones }
	}
	fn apply(&self, key: u64) -> Vec<u64> {
		let key = (key | self.ones) & !self.floating.last().unwrap();
		self.floating.iter().map(|mask| key | mask).collect()
	}
}

fn part_2(input: &str) -> u64 {
	let instructions = input.lines().map(InstructionV2::from_str);
	let mut mask = MaskV2::default();
	let mut values = std::collections::HashMap::with_capacity(instructions.size_hint().0);
	for instruction in instructions {
		match instruction {
			InstructionV2::Mask(new_mask) => mask = new_mask,
			InstructionV2::Assignment(Assignment { location, value }) => {
				for location in mask.apply(location) {
					values.insert(location, value);
				}
			}
		}
	}
	values.iter().map(|(_location, value)| value).sum::<u64>()
}

fn get_answers(input: &str) -> String {
	let sum_1 = part_1(input);
	let sum_2 = part_2(input);
	format!("1: {}, 2: {}", sum_1, sum_2)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn sample_input() {
		let input =
			"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
		assert_eq!(part_1(input), 165);
	}
	#[test]
	fn sample_input_2() {
		let input = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
		assert_eq!(part_2(input), 208);
	}
}

//println!("{:#036b}", mask);
