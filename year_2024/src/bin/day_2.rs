fn main() {
	shared::print_answers(
		2,
		&[get_answer_1, get_answer_2],
	);
}

fn get_answer_1(input: &str) -> usize {
	input
		.lines()
		.map(Report::from_line)
		.filter(|report| report.is_safe())
		.count()
}

fn get_answer_2(input: &str) -> usize {
	input
		.lines()
		.map(Report::from_line)
		.filter(|report| report.is_safe_v2())
		.count()
}

#[derive(Debug, Clone, Copy)]
struct Report<'l> {
	levels: &'l str,
}

impl<'l> Report<'l> {
	fn from_line(line: &'l str) -> Self {
		Self { levels: line }
	}
	fn is_safe(self) -> bool {
		let mut nums = self.levels.split(' ');
		let mut direction = None;
		let mut prev_num: u8 = nums.next().unwrap().parse().unwrap();
		for num in nums {
			let num = num.parse().unwrap();
			let ordering = prev_num.cmp(&num);
			match (prev_num.abs_diff(num), direction) {
				(1..=3, None) => direction = Some(ordering),
				(1..=3, Some(direction)) if direction == ordering => (),
				_ => return false,
			}
			prev_num = num;
		}
		true
	}
	fn is_safe_v2(self) -> bool {
		'outer: for i in 0..8 {
			let mut nums = self.levels.split(' ');
			if i == 0 {
				nums.next();
			}
			let mut direction = None;
			let mut prev_num: u8 = nums.next().unwrap().parse().unwrap();
			for (index, num) in nums.enumerate() {
				if index + 1 == i {
					continue;
				}
				let num = num.parse().unwrap();
				let ordering = prev_num.cmp(&num);
				match (prev_num.abs_diff(num), direction) {
					(1..=3, None) => direction = Some(ordering),
					(1..=3, Some(direction)) if direction == ordering => (),
					_ => continue 'outer,
				}
				prev_num = num;
			}
			return true;
		}
		false
	}
}
