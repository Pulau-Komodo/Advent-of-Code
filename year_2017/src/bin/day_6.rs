use std::{
	collections::{HashMap, HashSet},
	hash::Hash,
};

fn main() {
	shared::print_answers(6, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut state_history = HashSet::new();
	let mut state = State::<16>::from_str(input);
	let mut count = 0;
	loop {
		count += 1;
		state.cycle();
		if !state_history.insert(state) {
			break count;
		}
	}
}

fn get_answer_2(input: &str) -> u32 {
	let mut state_history = HashMap::new();
	let mut state = State::<16>::from_str(input);
	let mut count: u32 = 0;
	loop {
		count += 1;
		state.cycle();
		if let Some(prev_count) = state_history.get(&state) {
			break count - *prev_count;
		} else {
			state_history.insert(state, count);
		}
	}
}

#[derive(Clone, Copy, Eq)]
struct State<const N: usize> {
	stacks: [u8; N],
}

impl<const N: usize> State<N> {
	fn from_str(str: &str) -> Self {
		let mut stacks = [0; N];
		for (i, num) in str.split_ascii_whitespace().enumerate() {
			stacks[i] = num.parse().unwrap();
		}
		Self { stacks }
	}
	fn cycle(&mut self) {
		let (index, tallest_stack) = self
			.stacks
			.iter_mut()
			.enumerate()
			.rev()
			.max_by_key(|(_, stack)| **stack)
			.unwrap();
		let to_distribute = std::mem::take(tallest_stack);
		for i in index + 1..to_distribute as usize + index + 1 {
			self.stacks[i % self.stacks.len()] += 1;
		}
	}
}

impl<const N: usize> Hash for State<N> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		let mut packed_stacks: u128 = 0;
		for stack in self.stacks {
			packed_stacks <<= 8;
			packed_stacks += stack as u128;
		}
		packed_stacks.hash(state);
	}
}

impl<const N: usize> PartialEq for State<N> {
	fn eq(&self, other: &Self) -> bool {
		self.stacks == other.stacks
	}
}
