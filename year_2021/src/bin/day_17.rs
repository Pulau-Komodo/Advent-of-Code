use std::{collections::HashSet, ops::RangeInclusive};

fn main() {
	shared::print_answers(17, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> i32 {
	let (_, y_range) = parse_input(input);
	(y_range.start() + 1) * y_range.start() / 2
}

fn get_answer_2(input: &str) -> i32 {
	let (x_range, y_range) = parse_input(input);
	let max_steps = (-2 * y_range.start()) as u32;
	let horizontals = valid_horizontal_velocities(&x_range, max_steps);
	let valid_velocities = valid_velocities(&y_range, horizontals);
	valid_velocities.len() as i32
}

fn parse_input(input: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
	let input = input.strip_prefix("target area: x=").unwrap();
	let (x_range, y_range) = input.split_once(", y=").unwrap();
	let (x_start, x_end) = x_range.split_once("..").unwrap();
	let (y_start, y_end) = y_range.split_once("..").unwrap();
	let x_start: i32 = x_start.parse().unwrap();
	let x_end: i32 = x_end.parse().unwrap();
	let y_start: i32 = y_start.parse().unwrap();
	let y_end: i32 = y_end.parse().unwrap();
	(x_start..=x_end, y_start..=y_end)
}

fn valid_horizontal_velocities(range: &RangeInclusive<i32>, max_steps: u32) -> Vec<(i32, u32)> {
	(1..=*range.end())
		.filter_map(move |x| {
			let steps = horizontal_steps_inside(x, range, max_steps);
			if steps.is_empty() {
				None
			} else {
				Some(steps.into_iter().map(move |step| (x, step)))
			}
		})
		.flatten()
		.collect()
}

fn horizontal_steps_inside(
	initial_velocity: i32,
	range: &RangeInclusive<i32>,
	max_steps: u32,
) -> Vec<u32> {
	let mut steps = Vec::new();
	let mut step_count = 1;
	let mut distance = initial_velocity;
	let mut velocity = 0.max(initial_velocity - 1);
	loop {
		if distance > *range.end() || step_count > max_steps {
			break steps;
		} else if distance >= *range.start() {
			steps.push(step_count);
		}
		step_count += 1;
		distance += velocity;
		velocity = 0.max(velocity - 1);
	}
}

fn valid_velocities(
	y_range: &RangeInclusive<i32>,
	horizontals: Vec<(i32, u32)>,
) -> HashSet<(i32, i32)> {
	let mut output = HashSet::new();
	for (x, steps) in horizontals {
		for y in vertical_velocities_inside(steps, y_range) {
			output.insert((x, y));
		}
	}
	output
}

fn vertical_velocities_inside(steps: u32, range: &RangeInclusive<i32>) -> Vec<i32> {
	let mut velocities = Vec::new();
	for initial_velocity in *range.start()..=-1 * range.start() - 1 {
		let distance = {
			let mut position = initial_velocity;
			let mut velocity = initial_velocity - 1;
			for _ in 1..steps {
				position += velocity;
				velocity -= 1;
			}
			position
		};
		if range.contains(&distance) {
			velocities.push(initial_velocity);
		}
	}
	velocities
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample_input() {
		let input = "target area: x=20..30, y=-10..-5";
		assert_eq!(45, get_answer_1(input));
		assert_eq!(112, get_answer_2(input));
	}
}
