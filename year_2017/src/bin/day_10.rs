use std::{array, fmt::Write};

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
	let suffix = [17, 31, 73, 47, 23];

	let mut list: [u8; LIST_SIZE] = array::from_fn(|i| i as u8);
	let mut position = 0;
	let mut skip_size = 0;
	for _ in 0..64 {
		for length in input.bytes().chain(suffix).map(|n| n as usize) {
			for offset in 0..length / 2 {
				swap(&mut list, position + offset, position + length - offset - 1);
			}
			position += length + skip_size;
			position %= LIST_SIZE;
			skip_size += 1;
		}
	}
	let mut output = String::with_capacity(32);
	for chunk in 0..16 {
		let offset = chunk * 16;
		let xored = xor_slice(&list[offset..offset + 16]);
		output.write_fmt(format_args!("{:02x}", xored)).unwrap();
	}
	output
}

fn swap(list: &mut [u8; LIST_SIZE], a: usize, b: usize) {
	list.swap(a % LIST_SIZE, b % LIST_SIZE);
}

fn xor_slice(slice: &[u8]) -> u8 {
	slice.iter().copied().reduce(|acc, n| acc ^ n).unwrap()
}
