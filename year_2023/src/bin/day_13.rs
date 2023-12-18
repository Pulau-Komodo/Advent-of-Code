fn main() {
	shared::print_answers(13, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	input
		.split("\n\n")
		.map(Pattern::from_str)
		.map(|pattern| {
			pattern.find_horizontal_reflection().unwrap_or(0) * 100
				+ pattern.find_vertical_reflection().unwrap_or(0)
		})
		.sum()
}

fn get_answer_2(input: &str) -> usize {
	input
		.split("\n\n")
		.map(Pattern::from_str)
		.map(|pattern| {
			pattern
				.find_horizontal_reflection_with_smudge()
				.unwrap_or(0) * 100
				+ pattern.find_vertical_reflection_with_smudge().unwrap_or(0)
		})
		.sum()
}

struct Pattern {
	rows: Vec<Vec<bool>>,
	columns: Vec<Vec<bool>>,
}

impl Pattern {
	fn from_str(str: &str) -> Self {
		let rows: Vec<Vec<_>> = str
			.lines()
			.map(|line| line.bytes().map(|byte| matches!(byte, b'#')).collect())
			.collect();
		let row_length = rows.first().unwrap().len();
		let columns = (0..row_length)
			.map(|column| rows.iter().map(|row| *row.get(column).unwrap()).collect())
			.collect();
		Self { rows, columns }
	}
	fn find_horizontal_reflection(&self) -> Option<usize> {
		find_reflection(&self.rows)
	}
	fn find_vertical_reflection(&self) -> Option<usize> {
		find_reflection(&self.columns)
	}
	fn find_horizontal_reflection_with_smudge(&self) -> Option<usize> {
		find_reflection_with_smudge(&self.rows)
	}
	fn find_vertical_reflection_with_smudge(&self) -> Option<usize> {
		find_reflection_with_smudge(&self.columns)
	}
}

fn find_reflection(pattern: &[Vec<bool>]) -> Option<usize> {
	for row in 1..pattern.len() {
		let mut eccentricity = 1;
		loop {
			let Some(above) = row
				.checked_sub(eccentricity)
				.and_then(|above| pattern.get(above))
			else {
				return Some(row);
			};
			let Some(below) = pattern.get(row + eccentricity - 1) else {
				return Some(row);
			};
			if above != below {
				break;
			}
			eccentricity += 1;
		}
	}
	None
}

fn find_reflection_with_smudge(pattern: &[Vec<bool>]) -> Option<usize> {
	for row in 1..pattern.len() {
		let mut eccentricity = 1;
		let mut smudge_was_found = false;
		'search: loop {
			let Some(above) = row
				.checked_sub(eccentricity)
				.and_then(|above| pattern.get(above))
			else {
				if smudge_was_found {
					return Some(row);
				} else {
					break;
				}
			};
			let Some(below) = pattern.get(row + eccentricity - 1) else {
				if smudge_was_found {
					return Some(row);
				} else {
					break;
				}
			};
			for (a, b) in above.iter().zip(below) {
				if a != b {
					if smudge_was_found {
						break 'search;
					} else {
						smudge_was_found = true;
					}
				}
			}
			eccentricity += 1;
		}
	}
	None
}
