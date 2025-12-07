use shared::{SmallMap, SmallSet};

fn main() {
	shared::print_answers(7, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let mut split_count = 0;
	let mut beams = SmallSet::new();
	for line in input.lines().step_by(2) {
		for (index, byte) in line.bytes().enumerate() {
			if byte == b'S' {
				beams.insert(index);
			} else if byte == b'^' && beams.remove(&index) {
				beams.insert(index - 1);
				beams.insert(index + 1);
				split_count += 1;
			}
		}
	}
	split_count
}

fn get_answer_2(input: &str) -> u64 {
	let mut timelines = SmallMap::new();
	for line in input.lines().step_by(2) {
		for (index, byte) in line.bytes().enumerate() {
			if byte == b'S' {
				timelines.insert(index, 1);
			} else if byte == b'^'
				&& let Some(count) = timelines.remove(&index)
			{
				*timelines.get_mut_or_insert(index - 1, 0) += count;
				*timelines.get_mut_or_insert(index + 1, 0) += count;
			}
		}
	}
	timelines.values().sum()
}
