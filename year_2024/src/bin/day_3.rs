use shared::try_split_number;

fn main() {
	shared::print_answers(3, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut segments = input.split("mul(");
	if !input.starts_with("mul(") {
		segments.next();
	}
	segments
		.filter_map(|segment| try_finish_mul(segment).map(|(num, _)| num))
		.sum()
}

fn get_answer_2(input: &str) -> u32 {
	let mut sum = 0;
	let mut enabled = true;
	let mut cursor_start = 0;
	let mut cursor_end = 1;
	while cursor_end < input.len() {
		let substring = &input[cursor_start..cursor_end];
		if !enabled {
			if substring == "do()" {
				enabled = true;
			} else if "do()".starts_with(substring) {
				cursor_end += 1;
				continue;
			}
		} else if substring == "don't()" {
			enabled = false;
		} else if substring == "mul(" {
			if let Some((num, cursor_offset)) = try_finish_mul(&input[cursor_end..]) {
				sum += num;
				cursor_end += cursor_offset;
			}
		} else if ["don't()", "mul("]
			.into_iter()
			.any(|string| string.starts_with(substring))
		{
			cursor_end += 1;
			continue;
		}
		cursor_start = cursor_end;
		cursor_end = cursor_start + 1;
	}
	sum
}

fn try_finish_mul(substring: &str) -> Option<(u32, usize)> {
	let (a, rest) = try_split_number::<u32>(substring)?;
	let rest = rest.strip_prefix(',')?;
	let (b, rest) = try_split_number::<u32>(rest)?;
	if rest.starts_with(')') {
		let cursor_offset = substring.len() - rest.len() + 1;
		Some((a * b, cursor_offset))
	} else {
		None
	}
}
