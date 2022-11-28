use std::collections::HashMap;

fn main() {
	shared::print_answers(6, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> String {
	let mut frequencies: HashMap<usize, HashMap<char, usize>> = HashMap::new();
	for (pos, char) in input.lines().flat_map(str::char_indices) {
		let position_frequencies = frequencies.entry(pos).or_default();
		*position_frequencies.entry(char).or_insert(0) += 1;
	}
	let max_length = frequencies
		.iter()
		.fold(0, |acc, element| acc.max(*element.0 + 1));
	let mut output = String::with_capacity(max_length);
	for n in 0..max_length {
		let (most_frequent_char, _) = frequencies
			.get(&n)
			.unwrap()
			.iter()
			.max_by_key(|(_char, &frequency)| frequency)
			.unwrap();
		output.push(*most_frequent_char);
	}
	output
}

fn get_answer_2(input: &str) -> String {
	let mut frequencies: HashMap<usize, HashMap<char, usize>> = HashMap::new();
	for (pos, char) in input.lines().flat_map(str::char_indices) {
		let position_frequencies = frequencies.entry(pos).or_default();
		*position_frequencies.entry(char).or_insert(0) += 1;
	}
	let max_length = frequencies
		.iter()
		.fold(0, |acc, element| acc.max(*element.0 + 1));
	let mut output = String::with_capacity(max_length);
	for n in 0..max_length {
		let (most_frequent_char, _) = frequencies
			.get(&n)
			.unwrap()
			.iter()
			.min_by_key(|(_char, &frequency)| frequency)
			.unwrap();
		output.push(*most_frequent_char);
	}
	output
}
