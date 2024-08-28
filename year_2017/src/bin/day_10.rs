use std::array;

use year_2017::KnotHash;

fn main() {
	shared::print_answers(10, &[get_answer_1, get_answer_2]);
}

const LIST_SIZE: usize = 256;

fn get_answer_1(input: &str) -> String {
	let mut list: [u8; LIST_SIZE] = array::from_fn(|i| i as u8);
	let mut position = 0;
	for (skip_size, length) in input
		.trim()
		.split(',')
		.map(|n| n.parse::<usize>().unwrap())
		.enumerate()
	{
		for offset in 0..length / 2 {
			swap(&mut list, position + offset, position + length - offset - 1);
		}
		position += length + skip_size;
		position %= LIST_SIZE;
	}
	format!("{}", list[0] as u32 * list[1] as u32)
}

fn get_answer_2(input: &str) -> String {
	let input = input.trim();
	KnotHash::new(input).to_string()
}

fn swap(list: &mut [u8; LIST_SIZE], a: usize, b: usize) {
	list.swap(a % LIST_SIZE, b % LIST_SIZE);
}
