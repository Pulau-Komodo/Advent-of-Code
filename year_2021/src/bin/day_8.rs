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

#[derive(Clone, Copy, Default)]
struct SignalPattern {
	bitmap: u8,
	signals: u8,
}

fn parse_input(input: &str) -> impl Iterator<Item = ([SignalPattern; 10], [u8; 4])> + '_ {
	input.lines().map(|line| {
		let (signal_patterns, output_value) = line.split_once(" | ").unwrap();
		let (mut signal, mut output) = ([SignalPattern::default(); 10], [0; 4]);
		for (i, element) in signal_patterns.split(' ').enumerate() {
			signal[i] = SignalPattern {
				bitmap: signal_str_to_number(element),
				signals: element.len() as u8,
			};
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
		output |= 1
			<< match char {
				'a' => 0,
				'b' => 1,
				'c' => 2,
				'd' => 3,
				'e' => 4,
				'f' => 5,
				'g' => 6,
				_ => panic!(),
			}
	}
	output
}

fn resolve_signal(patterns: [SignalPattern; 10]) -> [u8; 10] {
	let mut key = [0; 10];
	fn get_bitmap(patterns: &[SignalPattern], predicate: &dyn Fn(&&SignalPattern) -> bool) -> u8 {
		patterns.iter().find(predicate).unwrap().bitmap
	}
	key[1] = get_bitmap(&patterns, &|&&pattern| pattern.signals == 2);
	key[4] = get_bitmap(&patterns, &|&&pattern| pattern.signals == 4);
	key[7] = get_bitmap(&patterns, &|&&pattern| pattern.signals == 3);
	key[8] = get_bitmap(&patterns, &|&&pattern| pattern.signals == 7);
	key[3] = get_bitmap(&patterns, &|&&pattern| {
		pattern.signals == 5 && pattern.bitmap & key[7] == key[7]
	});
	key[9] = get_bitmap(&patterns, &|&&pattern| {
		pattern.signals == 6 && pattern.bitmap & key[3] == key[3]
	});
	key[0] = get_bitmap(&patterns, &|&&pattern| {
		pattern.signals == 6 && pattern.bitmap != key[9] && pattern.bitmap & key[7] == key[7]
	});
	key[6] = get_bitmap(&patterns, &|&&pattern| {
		pattern.signals == 6 && pattern.bitmap != key[9] && pattern.bitmap != key[0]
	});
	key[5] = get_bitmap(&patterns, &|&&pattern| {
		pattern.signals == 5
			&& pattern.bitmap != key[3]
			&& pattern.bitmap & key[6] == pattern.bitmap
	});
	key[2] = get_bitmap(&patterns, &|&&pattern| {
		pattern.signals == 5 && pattern.bitmap != key[5] && pattern.bitmap != key[3]
	});
	key
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
	fn answer_2() {
		assert_eq!(get_answer_2("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), 5353);
	}
}
