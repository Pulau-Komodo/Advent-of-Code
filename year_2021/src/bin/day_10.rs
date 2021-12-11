fn main() {
	shared::print_answers(10, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	input
		.lines()
		.filter_map(|line| parse_line(line).err())
		.map(|char| match char {
			')' => 3,
			']' => 57,
			'}' => 1197,
			'>' => 25137,
			_ => panic!("Unknown char"),
		})
		.sum()
}

fn get_answer_2(input: &str) -> u64 {
	let mut scores: Vec<_> = input
		.lines()
		.filter_map(|line| parse_line(line).ok())
		.map(score_line)
		.collect();
	scores.sort_unstable();
	*scores.get((scores.len() - 1) / 2).unwrap()
}

fn parse_line(line: &str) -> Result<String, char> {
	let mut brackets = String::with_capacity(line.len());
	for char in line.chars() {
		if matches!(char, ')' | ']' | '}' | '>') {
			if !matches!(
				(brackets.pop(), char),
				(Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>')
			) {
				return Err(char);
			}
		} else {
			brackets.push(char);
		}
	}
	Ok(brackets)
}

fn score_line(line: String) -> u64 {
	line.chars().enumerate().map(|(i, char)| match char {
		'(' => 1,
		'[' => 2,
		'{' => 3,
		'<' => 4,
		_ => panic!("Unknown char")
	} * 5_usize.pow(i as u32)).sum::<usize>() as u64
}
