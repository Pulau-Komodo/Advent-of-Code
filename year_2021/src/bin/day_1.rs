fn main() {
	shared::print_answers(1, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut depths = parse_input(input);
	let mut count = 0;
	let mut prev_depth = depths.next().unwrap();
	for depth in depths {
		if depth > prev_depth {
			count += 1;
		}
		prev_depth = depth;
	}
	count
}

fn get_answer_2(input: &str) -> u32 {
	let mut depths = parse_input(input);
	let mut count = 0;
	let mut depth_a = depths.next().unwrap();
	let mut depth_b = depths.next().unwrap();
	let mut depth_c = depths.next().unwrap();
	for depth in depths {
		if depth > depth_a {
			count += 1;
		}
		depth_a = depth_b;
		depth_b = depth_c;
		depth_c = depth;
	}
	count
}

fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
	input.lines().map(str::parse).map(Result::unwrap)
}
