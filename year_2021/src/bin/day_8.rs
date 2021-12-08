fn main() {
	shared::print_answers(8, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			let (_signal_patterns, output_patterns) = line.split_once(" | ").unwrap();
			output_patterns
				.split(' ')
				.filter(|pattern| matches!(pattern.len(), 2 | 3 | 4 | 7))
				.count() as u32
		})
		.sum()
}

fn get_answer_2(input: &str) -> u32 {
	let entries = parse_input(input);
	entries
		.map(|(signal_pattern, output_value)| {
			let key = resolve_signal(signal_pattern);
			read_output(key, output_value)
		})
		.sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = ([u8; 10], [u8; 4])> + '_ {
	input.lines().map(|line| {
		let (signal_patterns, output_value) = line.split_once(" | ").unwrap();
		let (mut signal, mut output) = ([0; 10], [0; 4]);
		for (i, element) in signal_patterns.split(' ').enumerate() {
			signal[i] = signal_str_to_number(element);
		}
		for (i, element) in output_value.split(' ').enumerate() {
			output[i] = signal_str_to_number(element);
		}
		(signal, output)
	})
}

fn signal_str_to_number(str: &str) -> u8 {
	let mut output = 0;
	for char in str.chars() {
		match char {
			'a' => output += 1,
			'b' => output += 1 << 1,
			'c' => output += 1 << 2,
			'd' => output += 1 << 3,
			'e' => output += 1 << 4,
			'f' => output += 1 << 5,
			'g' => output += 1 << 6,
			_ => panic!(),
		}
	}
	output
}

fn resolve_signal(patterns: [u8; 10]) -> [u8; 10] {
	let mut key = [0; 10];
	key[1] = patterns
		.iter()
		.position(|&pattern| count_1_bits(pattern) == 2)
		.unwrap();
	key[4] = patterns
		.iter()
		.position(|&pattern| count_1_bits(pattern) == 4)
		.unwrap();
	key[7] = patterns
		.iter()
		.position(|&pattern| count_1_bits(pattern) == 3)
		.unwrap();
	key[8] = patterns
		.iter()
		.position(|&pattern| count_1_bits(pattern) == 7)
		.unwrap();
	key[3] = patterns
		.iter()
		.position(|&pattern| {
			count_1_bits(pattern) == 5 && count_1_bits(pattern & patterns[key[7]]) == 3
		})
		.unwrap();
	key[2] = patterns
		.iter()
		.position(|&pattern| {
			count_1_bits(pattern) == 5 && count_1_bits(pattern & patterns[key[4]]) == 2
		})
		.unwrap();
	key[5] = patterns
		.iter()
		.position(|&pattern| {
			count_1_bits(pattern) == 5 && pattern != patterns[key[2]] && pattern != patterns[key[3]]
		})
		.unwrap();
	key[9] = patterns
		.iter()
		.position(|&pattern| {
			count_1_bits(pattern) == 6 && count_1_bits(pattern & patterns[key[3]]) == 5
		})
		.unwrap();
	key[0] = patterns
		.iter()
		.position(|&pattern| {
			count_1_bits(pattern) == 6
				&& pattern != patterns[key[9]]
				&& count_1_bits(pattern & patterns[key[7]]) == 3
		})
		.unwrap();
	key[6] = patterns
		.iter()
		.position(|&pattern| {
			count_1_bits(pattern) == 6 && count_1_bits(pattern & patterns[key[1]]) == 1
		})
		.unwrap();
	let mut output = [0; 10];
	for (i, &element) in key.iter().enumerate() {
		output[i] = patterns[element];
	}
	output
}

fn count_1_bits(byte: u8) -> u8 {
	let mut output = 0;
	for i in 0..7 {
		if byte & 1 << i != 0 {
			output += 1
		}
	}
	output
}

fn read_output(key: [u8; 10], output: [u8; 4]) -> u32 {
	let mut result = 0;
	for (i, &digit) in output.iter().rev().enumerate() {
		result += key.iter().position(|&k| k == digit).unwrap() as u32 * 10_u32.pow(i as u32);
	}
	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn bit_count() {
		assert_eq!(count_1_bits(0b0), 0);
		assert_eq!(count_1_bits(0b1), 1);
		assert_eq!(count_1_bits(0b101010), 3);
		assert_eq!(count_1_bits(0b001110), 3);
		assert_eq!(count_1_bits(0b1111111), 7);
		assert_eq!(count_1_bits(0b1111000), 4);
	}
	#[test]
	fn answer_2() {
		assert_eq!(get_answer_2("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), 5353);
	}
}
