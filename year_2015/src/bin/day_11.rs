use std::ops::Div;

fn main() {
	shared::print_answers(11, &[get_answers]);
}

fn get_answers(input: &str) -> String {
	let now = std::time::Instant::now();
	let mut pw = Password::from_str(input);
	pw.increment();
	while !pw.is_valid() {
		pw.increment();
	}
	let answer_1 = pw.as_string();
	let time_1 = now.elapsed().as_micros();
	pw.increment();
	while !pw.is_valid() {
		pw.increment();
	}
	format!("1: {} ({} μs), 2: {}", answer_1, time_1, pw.as_string())
}

const CHARS: [char; 23] = [
	'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v',
	'w', 'x', 'y', 'z',
];

struct Password {
	value: u64,
}

impl Password {
	fn from_str(str: &str) -> Self {
		let char_values: std::collections::HashMap<char, u8> = CHARS
			.iter()
			.enumerate()
			.map(|(i, &char)| (char, i as u8))
			.collect();
		let value = str
			.chars()
			.rev()
			.enumerate()
			.map(|(place, char)| {
				(*char_values.get(&char).unwrap() as u64) * 23u64.pow(place as u32)
			})
			.sum();
		Self { value }
	}
	fn as_string(&self) -> String {
		let mut value = self.value;
		(0..8)
			.rev()
			.map(|n| {
				let pos_value = 23u64.pow(n);
				let char_value = value.div(pos_value) as usize;
				value %= pos_value;
				CHARS[char_value]
			})
			.collect()
	}
	fn is_valid(&self) -> bool {
		let string = self.as_string();
		let mut prev_byte = 0;
		let mut overlap_count: u8 = 0;
		let mut chain_length: u8 = 0;
		let mut prev_overlapped = false;
		for byte in string.into_bytes() {
			if byte == prev_byte && !prev_overlapped && overlap_count < 2 {
				overlap_count += 1;
				prev_overlapped = true;
			} else if prev_overlapped {
				prev_overlapped = false;
			}
			if chain_length < 2 {
				if byte == prev_byte + 1 {
					chain_length += 1;
				} else {
					chain_length = 0;
				}
			}
			if overlap_count == 2 && chain_length == 2 {
				return true;
			}
			prev_byte = byte;
		}
		false
	}
	fn increment(&mut self) {
		self.value += 1;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn test_conversion(str: &str, value: u64) {
		let password = Password::from_str(str);
		assert_eq!(value, password.value);
		assert_eq!(str, password.as_string().as_str());
	}

	#[test]
	fn convert_values() {
		test_conversion("aaaaaaaa", 0);
		test_conversion("aaaaaaab", 1);
		test_conversion("aaaaaaay", 21);
		test_conversion("aaaaaaaz", 22);
		test_conversion("aaaaaaba", 23);
		test_conversion("aaaaaabz", 45);
	}

	#[test]
	fn test_valid() {
		let password = Password::from_str("abccddef");
		assert!(password.is_valid());
		let password = Password::from_str("abcccefg");
		assert!(!password.is_valid());
		let password = Password::from_str("abacddeg");
		assert!(!password.is_valid());
	}
}
