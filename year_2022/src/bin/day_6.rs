use std::collections::HashMap;

fn main() {
	shared::print_answers(6, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	find_distinct_string(input, 4)
}

fn get_answer_2(input: &str) -> usize {
	find_distinct_string(input, 14)
}

fn find_distinct_string(str: &str, count: usize) -> usize {
	let mut past_chars: HashMap<char, usize> = HashMap::with_capacity(26);
	let mut unique_streak = 0;
	for (index, char) in str.char_indices() {
		unique_streak += 1;
		if let Some(last_index) = past_chars.get_mut(&char) {
			let ago = index - *last_index;
			unique_streak = unique_streak.min(ago);
			*last_index = index;
		} else {
			past_chars.insert(char, index);
		}
		if unique_streak == count {
			return index + 1;
		}
	}
	panic!("Could not find any string of length {count} with only unique characters.")
}
