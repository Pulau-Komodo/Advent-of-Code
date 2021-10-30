fn main() {
	shared::print_answers(5, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	input.lines().filter(|line| is_nice_1(line)).count()
}

fn get_answer_2(input: &str) -> usize {
	input.lines().filter(|line| is_nice_2(line)).count()
}

fn is_nice_1(string: &str) -> bool {
	const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
	let mut prev_char = ' ';
	let mut vowel_count = 0;
	let mut consecutive_letters = false;
	for char in string.chars() {
		if VOWELS.contains(&char) {
			vowel_count += 1;
		}
		if char == prev_char {
			consecutive_letters = true;
		}
		if matches!(
			(prev_char, char),
			('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')
		) {
			return false;
		}
		prev_char = char;
	}
	vowel_count >= 3 && consecutive_letters
}

fn is_nice_2(string: &str) -> bool {
	let mut char_pairs = std::collections::HashMap::new();
	let mut prev_prev_char = ' ';
	let mut prev_char = ' ';
	let mut double_pairs = false;
	let mut sandwiched_char = false;
	for (index, char) in string.char_indices() {
		if !double_pairs {
			if let Some(pair_index) = char_pairs.get(&(prev_char, char)) {
				if index - pair_index > 1 {
					double_pairs = true;
				}
			} else {
				char_pairs.insert((prev_char, char), index);
			}
		}
		if prev_prev_char == char {
			sandwiched_char = true;
		}
		if double_pairs && sandwiched_char {
			return true;
		}
		prev_prev_char = prev_char;
		prev_char = char;
	}
	false
}
