fn main() {
	shared::print_answers(7, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let positions = parse_input(input);
	let (min, max) = positions
		.iter()
		.fold((u32::MAX, u32::MIN), |(min, max), &position| {
			(min.min(position), max.max(position))
		});
	(min..=max).fold(u32::MAX, |min, target| {
		min.min(calculate_fuel_costs(&positions, target))
	})
}

fn get_answer_2(input: &str) -> u32 {
	let positions = parse_input(input);
	let (min, max) = positions
		.iter()
		.fold((u32::MAX, u32::MIN), |(min, max), &position| {
			(min.min(position), max.max(position))
		});
	(min..=max).fold(u32::MAX, |min, target| {
		min.min(calculate_fuel_costs_v2(&positions, target))
	})
}

fn parse_input(input: &str) -> Vec<u32> {
	input
		.split(',')
		.map(str::parse)
		.map(Result::unwrap)
		.collect()
}

fn calculate_fuel_costs(positions: &[u32], target: u32) -> u32 {
	positions
		.iter()
		.map(|&position| {
			if position > target {
				position - target
			} else {
				target - position
			}
		})
		.sum()
}

fn calculate_fuel_costs_v2(positions: &[u32], target: u32) -> u32 {
	positions
		.iter()
		.map(|&position| {
			let distance = if position > target {
				position - target
			} else {
				target - position
			};
			distance * (distance + 1) / 2
		})
		.sum()
}
