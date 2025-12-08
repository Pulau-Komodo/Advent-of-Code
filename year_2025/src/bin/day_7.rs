fn main() {
	shared::print_answers(7, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let mut split_count = 0;

	let mut lines = input.lines().step_by(2);
	let first_line = lines.next().unwrap();
	let mut beams = vec![false; first_line.len()];
	let start = first_line.bytes().position(|byte| byte == b'S').unwrap();
	beams[start] = true;
	let mut new_beams = vec![false; first_line.len()];

	for line in lines {
		for (index, byte) in line.bytes().enumerate() {
			if byte == b'^' && beams[index] {
				new_beams[index - 1] = true;
				new_beams[index + 1] = true;
				split_count += 1;
			} else {
				new_beams[index] |= beams[index];
			}
		}
		std::mem::swap(&mut beams, &mut new_beams);
		new_beams.fill(false);
	}
	split_count
}

fn get_answer_2(input: &str) -> u64 {
	let mut lines = input.lines().step_by(2);
	let first_line = lines.next().unwrap();
	let mut timelines = vec![0; first_line.len()];
	let start = first_line.bytes().position(|byte| byte == b'S').unwrap();
	timelines[start] = 1;
	let mut new_timelines = vec![0; first_line.len()];

	for line in lines {
		for (index, byte) in line.bytes().enumerate() {
			if byte == b'^' {
				new_timelines[index - 1] += timelines[index];
				new_timelines[index + 1] += timelines[index];
			} else {
				new_timelines[index] += timelines[index];
			}
		}
		std::mem::swap(&mut timelines, &mut new_timelines);
		new_timelines.fill(0);
	}
	timelines.into_iter().sum()
}
