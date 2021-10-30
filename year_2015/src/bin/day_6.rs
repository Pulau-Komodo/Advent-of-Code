fn main() {
	shared::print_answers(6, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let instructions = parse_instructions(input);
	(0..1_000)
		.map(|y| {
			(0..1_000)
				.filter(|x| determine_final_state_1(*x, y, &instructions))
				.count() as u32
		})
		.sum()
}

fn get_answer_2(input: &str) -> u32 {
	let instructions = parse_instructions(input);
	(0..1_000)
		.map(|y| {
			(0..1_000)
				.map(|x| determine_final_state_2(x, y, &instructions))
				.sum::<u16>() as u32
		})
		.sum()
}

enum Action {
	On,
	Off,
	Toggle,
}

struct Instruction {
	action: Action,
	range_x: std::ops::RangeInclusive<u16>,
	range_y: std::ops::RangeInclusive<u16>,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
	input
		.lines()
		.map(|line| {
			let (action, range) = if let Some(range) = line.strip_prefix("turn on ") {
				(Action::On, range)
			} else if let Some(range) = line.strip_prefix("turn off ") {
				(Action::Off, range)
			} else if let Some(range) = line.strip_prefix("toggle ") {
				(Action::Toggle, range)
			} else {
				panic!();
			};
			let (range_x, range_y) = parse_range(range);
			Instruction {
				action,
				range_x,
				range_y,
			}
		})
		.collect()
}

fn parse_range(range: &str) -> (std::ops::RangeInclusive<u16>, std::ops::RangeInclusive<u16>) {
	let (start, end) = range.split_once(" through ").unwrap();
	let (start_x, start_y) = start.split_once(",").unwrap();
	let (end_x, end_y) = end.split_once(",").unwrap();
	let (start_x, start_y, end_x, end_y) = (
		start_x.parse().unwrap(),
		start_y.parse().unwrap(),
		end_x.parse().unwrap(),
		end_y.parse().unwrap(),
	);
	(start_x..=end_x, start_y..=end_y)
}

fn determine_final_state_1(x: u16, y: u16, instructions: &[Instruction]) -> bool {
	let mut toggled = false;
	for instruction in instructions
		.iter()
		.filter(|instruction| instruction.range_x.contains(&x) && instruction.range_y.contains(&y))
		.rev()
	{
		match instruction.action {
			Action::On => return !toggled,
			Action::Off => return toggled,
			Action::Toggle => toggled = !toggled,
		}
	}
	toggled
}

fn determine_final_state_2(x: u16, y: u16, instructions: &[Instruction]) -> u16 {
	let mut brightness: u16 = 0;
	for instruction in instructions
		.iter()
		.filter(|instruction| instruction.range_x.contains(&x) && instruction.range_y.contains(&y))
	{
		match instruction.action {
			Action::On => brightness += 1,
			Action::Off => brightness = brightness.saturating_sub(1),
			Action::Toggle => brightness += 2,
		}
	}
	brightness
}
