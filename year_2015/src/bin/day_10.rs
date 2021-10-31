fn main() {
	shared::print_answers(10, &[get_answer]);
}

fn get_answer(input: &str) -> String {
	let now = std::time::Instant::now();
	let mut sequence = input.to_owned();
	for _ in 0..40 {
		sequence = look_and_say(&sequence);
	}
	let answer_1 = sequence.len() as u32;
	let time_1 = now.elapsed().as_micros();
	for _ in 40..50 {
		sequence = look_and_say(&sequence);
	}
	format!("1: {} ({} μs), 2: {}", answer_1, time_1, sequence.len())
}

fn look_and_say(sequence: &str) -> String {
	let mut last_char = ' ';
	let mut last_char_count = 0;
	let mut output = String::new();
	for char in sequence.chars().chain(std::iter::once(' ')) {
		if char != last_char {
			if last_char != ' ' {
				output.push_str(format!("{}{}", last_char_count, last_char).as_str());
			}
			last_char = char;
			last_char_count = 1;
		} else {
			last_char_count += 1;
		}
	}
	output
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample_input() {
		assert_eq!(look_and_say("111221"), "312211".to_string());
	}
}
