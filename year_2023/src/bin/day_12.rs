use std::{
	collections::HashMap,
	fmt::{Display, Write},
};

fn main() {
	shared::print_answers(12, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	input
		.lines()
		.map(ConditionRecord::from_line)
		.map(|record| record.possibility_count())
		.sum()
}

fn get_answer_2(input: &str) -> u64 {
	//return 0;
	input
		.lines()
		.map(ConditionRecord::from_line)
		.map(ConditionRecord::unfold)
		.map(|record| record.possibility_count())
		.sum()
}

struct ConditionRecord {
	record: Vec<Condition>,
	checksum: Vec<u8>,
}

impl ConditionRecord {
	fn from_line(line: &str) -> Self {
		let (record, checksum) = line.split_once(' ').unwrap();
		let record: Vec<_> = record.bytes().map(Condition::from_byte).collect();
		let checksum = checksum.split(',').map(|n| n.parse().unwrap()).collect();
		//_display_conditions(&record);
		Self { record, checksum }
	}
	fn possibility_count(&self) -> u64 {
		let mut history = HashMap::new();
		count_possible_configurations(&self.record, &self.checksum, &mut history)
	}
	fn unfold(self) -> Self {
		let record = self
			.record
			.iter()
			.chain(
				[Condition::Unknown]
					.iter()
					.chain(&self.record)
					.cycle()
					.take(self.record.len() * 4 + 4),
			)
			.copied()
			.collect();
		let checksum = self
			.checksum
			.iter()
			.cycle()
			.take(self.checksum.len() * 5)
			.copied()
			.collect();
		Self { record, checksum }
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Condition {
	/// `.`
	Operational,
	/// `#`
	Damaged,
	/// `?`
	Unknown,
}

impl Condition {
	fn from_byte(byte: u8) -> Self {
		match byte {
			b'.' => Self::Operational,
			b'#' => Self::Damaged,
			_ => Self::Unknown,
		}
	}
	fn could_be_operational(&self) -> bool {
		matches!(self, Condition::Operational | Condition::Unknown)
	}
	fn could_be_damaged(&self) -> bool {
		matches!(self, Condition::Damaged | Condition::Unknown)
	}
	fn is_damaged(&self) -> bool {
		matches!(self, Condition::Damaged)
	}
}

impl Display for Condition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let char = match self {
			Self::Operational => '.',
			Self::Damaged => '#',
			Self::Unknown => '?',
		};
		f.write_char(char)
	}
}

fn _display_conditions(conditions: &[Condition]) {
	for condition in conditions {
		print!("{condition}");
	}
	println!()
}

fn count_possible_configurations<'a>(
	conditions: &'a [Condition],
	checksum: &'a [u8],
	history: &mut HashMap<(&'a [Condition], &'a [u8]), u64>,
) -> u64 {
	let Some(clue) = checksum.first() else {
		if conditions.iter().all(Condition::could_be_operational) {
			return 1; // Done and valid.
		} else {
			return 0; // Done and not valid.
		}
	};
	if min_length(checksum) > conditions.len() {
		return 0; // Cut it short here; it's never going to fit.
	}
	let Some(mut cursor) = conditions.iter().position(Condition::could_be_damaged) else {
		return 0; // All intact, so cut short.
	};
	if let Some(value) = history.get(&(conditions, checksum)) {
		return *value;
	}
	let clue = *clue as usize;
	let mut acc = 0;
	loop {
		let end_index = cursor + clue;
		if end_index > conditions.len() {
			break; // We are too far right.
		}
		if conditions[cursor..end_index]
			.iter()
			.all(Condition::could_be_damaged)
			&& conditions
				.get(end_index)
				.map_or(true, Condition::could_be_operational)
		{
			acc += count_possible_configurations(
				conditions.get(end_index + 1..).unwrap_or_default(),
				&checksum[1..],
				history,
			);
		}
		if conditions[cursor].is_damaged() {
			break; // Can't move more away from left.
		}
		cursor += 1;
	}
	history.insert((conditions, checksum), acc);
	acc
}

fn min_length(checksum: &[u8]) -> usize {
	checksum.iter().map(|n| *n as usize + 1).sum::<usize>() - 1
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_cases() {
		let inputs = [
			"???.### 1,1,3",
			".??..??...?##. 1,1,3",
			"?#?#?#?#?#?#?#? 1,3,1,6",
			"????.#...#... 4,1,1",
			"????.######..#####. 1,6,5",
			"?###???????? 3,2,1",
		];
		let answers_part_1 = [1, 4, 1, 1, 4, 10];
		let answers_part_2 = [1, 16384, 1, 16, 2500, 506250];
		for (my_answer, right_answer) in inputs
			.iter()
			.copied()
			.map(ConditionRecord::from_line)
			.map(|record| record.possibility_count())
			.zip(answers_part_1)
		{
			assert_eq!(my_answer, right_answer);
		}
		for (my_answer, right_answer) in inputs
			.iter()
			.copied()
			.map(ConditionRecord::from_line)
			.map(ConditionRecord::unfold)
			.map(|record| record.possibility_count())
			.zip(answers_part_2)
		{
			assert_eq!(my_answer, right_answer);
		}
	}
	#[test]
	fn part_1() {
		let answer = get_answer_1(&shared::read_file(12));
		assert_eq!(7633, answer);
	}
}
