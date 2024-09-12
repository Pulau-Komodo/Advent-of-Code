use std::array;

fn main() {
	shared::print_answers(14, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let offset = process_input(input);
	let target = offset + 10;

	let recipes = develop_recipes(|recipes| recipes.len() == target);

	recipes
		.into_iter()
		.skip(offset)
		.fold(0, |acc, n| acc * 10 + n as u64)
}

fn get_answer_2(input: &str) -> u64 {
	let target: Vec<_> = input.trim().bytes().map(|byte| byte - b'0').collect();

	let recipes = develop_recipes(|recipes| matches_last_10(recipes, &target));

	recipes.len() as u64 - 10
}

fn process_input(input: &str) -> usize {
	input.trim().parse().unwrap()
}

fn develop_recipes(test: impl Fn(&[u8]) -> bool) -> Vec<u8> {
	let mut recipes = vec![3, 7];
	let mut positions: [usize; 2] = array::from_fn(std::convert::identity);
	loop {
		let sum: u8 = positions.iter().map(|pos| recipes[*pos]).sum();
		let digit_one = sum / 10;
		let digit_two = sum % 10;
		if digit_one > 0 {
			recipes.push(digit_one);
			if test(&recipes) {
				break;
			}
		}
		recipes.push(digit_two);
		if test(&recipes) {
			break;
		}
		for pos in &mut positions {
			*pos += 1 + recipes[*pos] as usize;
			*pos %= recipes.len();
		}
	}
	recipes
}

fn matches_last_10(recipes: &[u8], target: &[u8]) -> bool {
	recipes.len() >= 10
		&& recipes
			.iter()
			.skip(recipes.len() - 10)
			.zip(target)
			.all(|(a, b)| a == b)
}
