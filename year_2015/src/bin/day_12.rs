fn main() {
	shared::print_answers(12, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> i32 {
	sum_numbers(input)
}

fn get_answer_2(input: &str) -> i32 {
	sum_numbers(&strip_red(input))
}

fn sum_numbers(input: &str) -> i32 {
	let mut total = 0;
	let mut current_value = 0;
	let mut negative = false;
	for char in input.chars() {
		let digit = match char {
			'0' => 0,
			'1' => 1,
			'2' => 2,
			'3' => 3,
			'4' => 4,
			'5' => 5,
			'6' => 6,
			'7' => 7,
			'8' => 8,
			'9' => 9,
			'-' => {
				negative = true;
				continue;
			}
			_ => {
				if negative {
					total -= current_value;
				} else {
					total += current_value;
				}
				current_value = 0;
				negative = false;
				continue;
			}
		};
		current_value *= 10;
		current_value += digit;
	}
	total
}

fn strip_red(input: &str) -> String {
	let mut output = input.to_string();
	while let Some(index) = output.find(":\"red\"") {
		let start_index = find_object_end(&output[0..index], true);
		let end_index = find_object_end(&output[index + 5..], false);
		let mut new_output = output[0..start_index].to_string();
		new_output.push_str(&output[index + end_index + 6..]);
		output = new_output;
	}
	output
}

fn find_object_end(str: &str, backward: bool) -> usize {
	let mut bracket_level = 1;
	let mut iter: Box<dyn DoubleEndedIterator<Item = (usize, char)>> = Box::new(str.char_indices());
	if backward {
		iter = Box::new(iter.rev())
	}
	for (index, char) in iter {
		match (char, backward) {
			('{', true) | ('}', false) => bracket_level -= 1,
			('}', true) | ('{', false) => bracket_level += 1,
			_ => (),
		}
		if bracket_level == 0 {
			return index;
		}
	}
	panic!();
}
