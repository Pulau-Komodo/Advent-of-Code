use shared::count_digits;

fn main() {
	shared::print_answers(2, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let mut sum = 0;
	for range in input.trim().split(',') {
		let (start, end) = range.split_once('-').unwrap();
		let start: u64 = start.parse().unwrap();
		let end: u64 = end.parse().unwrap();
		for value in start..=end {
			if is_invalid_1(value) {
				sum += value;
			}
		}
	}
	sum
}

fn get_answer_2(input: &str) -> u64 {
	let mut sum = 0;
	for range in input.trim().split(',') {
		let (start, end) = range.split_once('-').unwrap();
		let start: u64 = start.parse().unwrap();
		let end: u64 = end.parse().unwrap();
		for value in start..=end {
			if is_invalid_2(value) {
				sum += value;
			}
		}
	}
	sum
}

fn is_invalid_1(value: u64) -> bool {
	let digits = count_digits(value, 10);
	if digits % 2 == 1 {
		return false;
	}
	let divisor = 10_u32.pow(digits as u32 / 2) as u64;
	let left_part = value / divisor;
	let right_part = value - left_part * divisor;

	right_part == left_part
}

fn is_invalid_2(value: u64) -> bool {
	let digits = count_digits(value, 10) as u32;
	'group_size: for group_size in 1..=digits / 2 {
		if !digits.is_multiple_of(group_size) {
			continue;
		}
		let divisor = 10_u32.pow(digits - group_size) as u64;
		let segment = value / divisor;
		let mut remainder = value - segment * divisor;
		for group in 1..digits / group_size {
			let divisor = 10_u32.pow(digits - group_size * (group + 1)) as u64;
			let new_segment = remainder / divisor;
			if segment != new_segment {
				continue 'group_size;
			}
			remainder -= new_segment * divisor;
		}
		return true;
	}
	false
}
