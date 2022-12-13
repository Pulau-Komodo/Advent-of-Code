use std::collections::HashSet;

fn main() {
	shared::print_answers(4, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	input
		.lines()
		.filter(|line| has_no_duplicate_words(line))
		.count()
}

fn get_answer_2(input: &str) -> usize {
	input
		.lines()
		.filter(|line| has_no_anagram_words(line))
		.count()
}

fn has_no_duplicate_words(input: &str) -> bool {
	let mut words = HashSet::new();
	input.split(' ').all(|word| words.insert(word))
}

fn has_no_anagram_words(input: &str) -> bool {
	let mut words = HashSet::new();
	input.split(' ').all(|word| {
		let mut word = word.as_bytes().to_vec();
		word.sort();
		words.insert(word)
	})
}
