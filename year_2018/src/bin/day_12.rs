use std::{array, collections::VecDeque};

use shared::IntoPairIterator;

fn main() {
	shared::print_answers(12, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> i64 {
	let (mut state, rules) = parse_input(input);

	for _ in 0..20 {
		let mut next_state = Vec::new();
		for pot in state.first().unwrap() - 2..state.last().unwrap() + 2 {
			let substate = array::from_fn(|i| state.contains(&(pot - 2 + i as i32)));
			if rules[substate_to_number(substate)] {
				next_state.push(pot);
			}
		}
		std::mem::swap(&mut state, &mut next_state);
	}

	state.iter().map(|n| *n as i64).sum()
}

fn get_answer_2(input: &str) -> i64 {
	const TARGET: i64 = 50_000_000_000;
	let (mut state, rules) = parse_input(input);

	let mut sums: VecDeque<i32> = [0; 9].into_iter().collect();
	sums.push_back(state.iter().sum());

	for steps in 1..=TARGET {
		let mut next_state = Vec::new();
		for pot in state.first().unwrap() - 2..state.last().unwrap() + 2 {
			let substate = array::from_fn(|i| state.contains(&(pot - 2 + i as i32)));
			if rules[substate_to_number(substate)] {
				next_state.push(pot);
			}
		}
		std::mem::swap(&mut state, &mut next_state);

		sums.pop_front();
		sums.push_back(state.iter().sum());
		let difference = sums[1] - sums[0];
		if sums.iter().pairs().all(|(a, b)| b - a == difference) {
			// Stable growth found (could not tell you why this happens)
			let steps_left = TARGET - steps;
			return sums[9] as i64 + steps_left * difference as i64;
		}
	}
	panic!();
}

fn parse_input(input: &str) -> (Vec<i32>, [bool; 32]) {
	let (initial_state, rule_text) = input.split_once("\n\n").unwrap();
	let initial_state = initial_state["initial state: ".len()..]
		.bytes()
		.enumerate()
		.filter_map(|(index, plant)| (plant == b'#').then_some(index as i32))
		.collect();
	let mut rules = [false; 32];
	for line in rule_text.lines() {
		let (input, output) = line.split_once(" => ").unwrap();
		if output == "#" {
			rules[input_to_number(input)] = true;
		}
	}
	(initial_state, rules)
}

fn input_to_number(input: &str) -> usize {
	let mut number = 0;
	for byte in input.bytes() {
		number <<= 1;
		if byte == b'#' {
			number += 1;
		}
	}
	number
}

fn substate_to_number(substate: [bool; 5]) -> usize {
	let mut number = 0;
	for plant in substate {
		number <<= 1;
		if plant {
			number += 1;
		}
	}
	number
}
