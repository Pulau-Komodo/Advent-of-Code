fn main() {
	shared::print_answers(1, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	input
		.split("\r\n\r\n")
		.map(|group| {
			group
				.lines()
				.map(|calories| calories.parse::<u32>().unwrap())
				.sum::<u32>()
		})
		.max()
		.unwrap()
}

fn get_answer_2(input: &str) -> u32 {
	input
		.split("\r\n\r\n")
		.map(|group| {
			group
				.lines()
				.map(|calories| calories.parse::<u32>().unwrap())
				.sum::<u32>()
		})
		.fold([0, 0, 0], |mut acc, new_value| {
			if new_value > acc[2] {
				acc[2] = new_value;
				acc.sort_unstable_by(|a, b| b.cmp(a));
			}
			acc
		})
		.iter()
		.sum()
}
