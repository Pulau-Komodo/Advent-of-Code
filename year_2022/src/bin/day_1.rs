use shared::IteratorTop;

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
		.top::<3>()
		.iter()
		.sum()
}
