fn main() {
	shared::print_answers(17, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let step_count: usize = input.trim().parse().unwrap();

	let mut buffer = Vec::with_capacity(2018);
	buffer.push(0);
	let mut pos = 0;
	for num in 1..=2017 {
		pos += step_count;
		pos %= buffer.len();
		pos += 1;
		buffer.insert(pos, num);
	}

	buffer[pos + 1]
}

fn get_answer_2(input: &str) -> usize {
	let step_count: usize = input.trim().parse().unwrap();

	let mut buffer_length = 1;
	let mut second = None;
	let mut pos = 0;
	for num in 1..=50_000_000 {
		pos += step_count;
		pos %= buffer_length;
		pos += 1;
		buffer_length += 1;
		if pos == 1 {
			second = Some(num);
		}
	}

	second.unwrap()
}
