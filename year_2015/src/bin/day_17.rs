fn main() {
	shared::print_answers(17, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let containers: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();
	(0..2usize.pow(containers.len() as u32))
		.filter(|i| {
			let capacity: u32 = containers
				.iter()
				.enumerate()
				.filter(|(index, _)| 1 << index & i != 0)
				.map(|(_, capacity)| capacity)
				.sum();
			capacity == 150
		})
		.count()
}

fn get_answer_2(input: &str) -> usize {
	let containers: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();
	let mut smallest_number = u32::MAX;
	let mut smallest_count = 0;
	for i in 0..2usize.pow(containers.len() as u32) {
		let capacity: u32 = containers
			.iter()
			.enumerate()
			.filter(|(index, _)| 1 << index & i != 0)
			.map(|(_, capacity)| capacity)
			.sum();
		if capacity == 150 {
			let ones = count_ones(i);
			use std::cmp::Ordering::*;
			match ones.cmp(&smallest_number) {
				Less => {
					smallest_number = ones;
					smallest_count = 1;
				}
				Equal => smallest_count += 1,
				Greater => (),
			}
		}
	}
	smallest_count
}

fn count_ones(mut num: usize) -> u32 {
	let mut count = 0;
	while num > 0 {
		if num & 1 == 1 {
			count += 1;
		}
		num >>= 1;
	}
	count
}
