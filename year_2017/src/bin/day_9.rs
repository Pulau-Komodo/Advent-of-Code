fn main() {
	shared::print_answers(9, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut score = 0;
	let mut open_groups = 0;
	let mut in_garbage = false;
	let mut canceling = false;
	for byte in input.bytes() {
		match byte {
			b'{' if !in_garbage => {
				open_groups += 1;
				score += open_groups;
			}
			b'}' if !in_garbage => open_groups -= 1,
			b'<' => {
				in_garbage = true;
				canceling = false
			}
			b'>' if !canceling => in_garbage = false,
			b'!' if in_garbage => canceling = !canceling,
			_ => canceling = false,
		}
	}
	score
}

fn get_answer_2(input: &str) -> u32 {
	let mut count = 0;
	let mut in_garbage = false;
	let mut canceling = false;
	for byte in input.bytes() {
		match byte {
			b'{' | b'}' if !in_garbage => {}
			b'<' if !in_garbage => {
				in_garbage = true;
			}
			b'>' if !canceling => in_garbage = false,
			b'!' if in_garbage => canceling = !canceling,
			_ if canceling => canceling = false,
			_ if in_garbage => count += 1,
			_ => (),
		}
	}
	count
}
