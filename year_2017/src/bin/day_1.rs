fn main() {
	shared::print_answers(1, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let bytes = input.as_bytes();
	let wrap = if bytes[bytes.len() - 1] == bytes[0] {
		(bytes[0] - b'0') as u32
	} else {
		0
	};
	bytes
		.windows(2)
		.filter(|pair| pair[0] == pair[1])
		.map(|pair| (pair[0] - b'0') as u32)
		.sum::<u32>()
		+ wrap
}

fn get_answer_2(input: &str) -> u32 {
	input
		.as_bytes()
		.windows(input.as_bytes().len() / 2 + 1)
		.filter_map(|window| {
			let [a, .., b] = window else { panic!() };
			(a == b).then(|| (a - b'0') as u32)
		})
		.sum::<u32>()
		* 2
}
