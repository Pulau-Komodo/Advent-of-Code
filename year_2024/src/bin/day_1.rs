fn main() {
	shared::print_answers(1, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let (mut list_left, list_right) = parse_lists(input);
	list_left.sort_unstable();
	list_left
		.into_iter()
		.zip(list_right)
		.map(|(l, r)| l.abs_diff(r))
		.sum()
}

fn get_answer_2(input: &str) -> u32 {
	let (list_left, list_right) = parse_lists(input);
	list_left
		.into_iter()
		.map(|num_left| {
			let Ok(index) = list_right.binary_search(&num_left) else {
				return 0;
			};
			let count = list_right
				.iter()
				.skip(index + 1)
				.take_while(|num_right| **num_right == num_left)
				.chain(
					list_right
						.iter()
						.rev()
						.skip(list_right.len() - index)
						.take_while(|num_right| **num_right == num_left),
				)
				.count() as u32
				+ 1;
			num_left * count
		})
		.sum()
}

/// Right list is always sorted because it is always useful to sort.
fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
	let (list_left, mut list_right) = input
		.lines()
		.map(|line| {
			let mut nums = line
				.split_ascii_whitespace()
				.map(|num| num.parse::<u32>().unwrap());
			(nums.next().unwrap(), nums.next().unwrap())
		})
		.collect::<(Vec<_>, Vec<_>)>();
	list_right.sort_unstable();
	(list_left, list_right)
}
