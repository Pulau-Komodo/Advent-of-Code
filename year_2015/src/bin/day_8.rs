fn main() {
	shared::print_answers(8, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			let mut coding_char_count = 2;
			let mut skip: u8 = 0;
			for (index, char) in line.char_indices() {
				if index == 0 || index == line.len() - 1 {
					continue;
				}
				if skip == 1 && char == 'x' {
					skip = 2;
					coding_char_count += 2;
				} else if skip > 0 {
					skip -= 1;
					continue;
				}
				if char == '\\' {
					skip = 1;
					coding_char_count += 1;
				}
			}
			coding_char_count
		})
		.sum()
}

fn get_answer_2(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			line.chars()
				.filter(|&char| char == '"' || char == '\\')
				.count() as u32
				+ 2
		})
		.sum()
}
