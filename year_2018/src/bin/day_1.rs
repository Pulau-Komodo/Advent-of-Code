use std::collections::HashSet;

fn main() {
	shared::print_answers(1, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> i32 {
	input.lines().map(|line| line.parse::<i32>().unwrap()).sum()
}

fn get_answer_2(input: &str) -> i32 {
	let nums: Vec<_> = input
		.lines()
		.map(|line| line.parse::<i32>().unwrap())
		.collect();
	let mut visited = HashSet::new();
	let mut frequency = 0;
	for num in nums.iter().cycle() {
		if !visited.insert(frequency) {
			return frequency;
		}
		frequency += num;
	}
	unreachable!();
}
