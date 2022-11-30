use std::{collections::HashMap, num::NonZeroU8};

fn main() {
	shared::print_answers(10, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let instructions: Vec<_> = input.lines().map(Instruction::from_str).collect();
	let mut bots = HashMap::new();
	let mut outputs: HashMap<u8, Vec<NonZeroU8>> = HashMap::new();
	for instruction in instructions.iter() {
		if let Instruction::Bot(bot_id, bot) = instruction {
			bots.insert(*bot_id, *bot);
		}
	}
	for instruction in instructions.iter() {
		if let Instruction::Value(value, target) = instruction {
			bots.get_mut(target).unwrap().give_chip(*value);
		}
	}
	for instruction in instructions.iter().cycle() {
		if let Instruction::Bot(bot_id, _) = instruction {
			if let Some([low, high]) = bots.get_mut(bot_id).unwrap().pass_on_chips() {
				if low.1 == NonZeroU8::new(17).unwrap() && high.1 == NonZeroU8::new(61).unwrap() {
					return *bot_id as u32;
				}
				match low.0 {
					Target::Bot(bot_id) => bots.get_mut(&bot_id).unwrap().give_chip(low.1),
					Target::Output(output) => outputs.entry(output).or_default().push(low.1),
				}
				match high.0 {
					Target::Bot(bot_id) => bots.get_mut(&bot_id).unwrap().give_chip(high.1),
					Target::Output(output) => outputs.entry(output).or_default().push(high.1),
				}
			}
		}
	}
	unreachable!();
}

fn get_answer_2(input: &str) -> u32 {
	let instructions: Vec<_> = input.lines().map(Instruction::from_str).collect();
	let mut bots = HashMap::new();
	let mut outputs: HashMap<u8, Vec<NonZeroU8>> = HashMap::new();
	for instruction in instructions.iter() {
		if let Instruction::Bot(bot_id, bot) = instruction {
			bots.insert(*bot_id, *bot);
		}
	}
	for instruction in instructions.iter() {
		if let Instruction::Value(value, target) = instruction {
			bots.get_mut(target).unwrap().give_chip(*value);
		}
	}
	let mut product = 1;
	let mut components_found = 0;
	for instruction in instructions.iter().cycle() {
		if let Instruction::Bot(bot_id, _) = instruction {
			if let Some([low, high]) = bots.get_mut(bot_id).unwrap().pass_on_chips() {
				match low.0 {
					Target::Bot(bot_id) => bots.get_mut(&bot_id).unwrap().give_chip(low.1),
					Target::Output(output) => {
						outputs.entry(output).or_default().push(low.1);
						if (0..=2).contains(&output) {
							product *= low.1.get() as u32;
							components_found += 1;
						}
					}
				}
				match high.0 {
					Target::Bot(bot_id) => bots.get_mut(&bot_id).unwrap().give_chip(high.1),
					Target::Output(output) => {
						outputs.entry(output).or_default().push(high.1);
						if (0..=2).contains(&output) {
							product *= high.1.get() as u32;
							components_found += 1;
						}
					}
				}
			}
		}
		if components_found >= 3 {
			break;
		}
	}
	product
}

#[derive(Clone, Copy)]
enum Target {
	Bot(u8),
	Output(u8),
}

impl Target {
	fn from_str(str: &str) -> Self {
		let (which, target) = str.split_once(' ').unwrap_or_else(|| panic!("{str}"));
		let target = target.parse().unwrap();
		match which {
			"bot" => Target::Bot(target),
			"output" => Target::Output(target),
			_ => panic!(),
		}
	}
}

#[derive(Copy, Clone)]
struct Bot {
	chips: [Option<NonZeroU8>; 2],
	low_target: Target,
	high_target: Target,
}

impl Bot {
	fn give_chip(&mut self, chip: NonZeroU8) {
		if self.chips[0].is_none() {
			self.chips[0] = Some(chip);
		} else {
			self.chips[1] = Some(chip);
		}
	}
	fn pass_on_chips(&mut self) -> Option<[(Target, NonZeroU8); 2]> {
		if let [Some(chip_a), Some(chip_b)] = self.chips {
			let (low, high) = if chip_a < chip_b {
				(chip_a, chip_b)
			} else {
				(chip_b, chip_a)
			};
			self.chips[0] = None;
			self.chips[1] = None;
			Some([(self.low_target, low), (self.high_target, high)])
		} else {
			None
		}
	}
}

enum Instruction {
	Bot(u8, Bot),
	Value(NonZeroU8, u8),
}

impl Instruction {
	fn from_str(str: &str) -> Self {
		let (which, rest) = str.split_once(' ').unwrap();
		if which == "value" {
			let (value, rest) = rest.split_once(' ').unwrap();
			let (_, target) = rest.rsplit_once(' ').unwrap();
			let value = value.parse().unwrap();
			let target = target.parse().unwrap();
			Instruction::Value(value, target)
		} else {
			let (bot, rest) = rest.split_once(" gives low to ").unwrap();
			let (low_target, high_target) = rest.split_once(" and high to ").unwrap();
			let bot = bot.parse().unwrap();
			let low_target = Target::from_str(low_target);
			let high_target = Target::from_str(high_target);
			Instruction::Bot(
				bot,
				Bot {
					chips: [None; 2],
					low_target,
					high_target,
				},
			)
		}
	}
}
