use std::collections::VecDeque;

fn main() {
	shared::print_answers(
		19,
		&[
			get_answer_1_good,
			get_answer_1,
			get_answer_2_good,
			get_answer_2,
		],
	);
}

fn get_answer_1(input: &str) -> u32 {
	let mut num: u32 = input.parse().unwrap();
	let mut elf = 0;
	let mut pick_evens = true;
	let mut last_odd = false;
	let mut digit = 0;
	while num > 0 {
		if !pick_evens {
			elf |= 1 << digit;
		}
		digit += 1;
		pick_evens ^= (num & 1 == 1) ^ last_odd;
		last_odd = num & 1 == 1;
		num >>= 1;
	}
	elf + 1
}

// Not mine
fn get_answer_1_good(input: &str) -> u32 {
	let num: u32 = input.parse().unwrap();
	num * 2 - (num + 1).next_power_of_two() + 1
}

fn get_answer_2(input: &str) -> u32 {
	let num: u32 = input.parse().unwrap();
	present_exchange_v2(num)
}

// Not mine
fn get_answer_2_good(input: &str) -> u32 {
	let num: u32 = input.parse().unwrap();
	num - next_power_of_three(num / 3 + 1)
}

fn next_power_of_three(num: u32) -> u32 {
	let mut power = 1;
	while power < num {
		power *= 3;
	}
	power
}

fn present_exchange_v2(elf_count: u32) -> u32 {
	let mut elves: VecDeque<_> = (0..elf_count).collect();
	elves.rotate_left(elf_count as usize / 2 + elf_count as usize % 2 - 1);
	while elves.len() > 1 {
		if elves.len() % 2 == 0 {
			elves.rotate_left(1);
		}
		elves.pop_front();
	}
	elves[0] + 1
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn v2() {
		for n in 1..50 {
			let result = present_exchange_v2(n);
			println!("{n:b}: {result:b}");
		}
	}
}
