fn main() {
	shared::print_answers(3, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let report = parse_input(input);
	get_power_consumption(&report)
}

fn get_answer_2(input: &str) -> u32 {
	let report = parse_input(input);
	get_life_support_rating(&report)
}

fn parse_input(input: &str) -> Vec<[bool; 12]> {
	input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> [bool; 12] {
	let mut output = [false; 12];
	for (index, char) in input.char_indices() {
		if char == '1' {
			output[index] = true;
		}
	}
	output
}

fn most_common(report: &[[bool; 12]]) -> [bool; 12] {
	let half_length = report.len() / 2;
	let true_counts = report.iter().fold([0_usize; 12], |mut acc, reading| {
		for (index, bit) in reading.iter().enumerate() {
			if *bit {
				acc[index] += 1;
			}
		}
		acc
	});
	let mut output = [false; 12];
	for (index, count) in true_counts.iter().enumerate() {
		output[index] = *count >= half_length;
	}
	output
}

fn get_power_consumption(report: &[[bool; 12]]) -> u32 {
	let most_common = most_common(report);
	let mut gamma_rate = 0;
	let mut epsilon_rate = 0;
	for (index, &on) in most_common.iter().rev().enumerate() {
		if on {
			gamma_rate += 2_u32.pow(index as u32);
		} else {
			epsilon_rate += 2_u32.pow(index as u32);
		}
	}
	gamma_rate * epsilon_rate
}

fn get_life_support_rating(report: &[[bool; 12]]) -> u32 {
	let oxygen_generator_entry = find_entry(report, true);
	let co2_scrubber_entry = find_entry(report, false);
	make_number(&oxygen_generator_entry) * make_number(&co2_scrubber_entry)
}

fn find_entry(report: &[[bool; 12]], most: bool) -> [bool; 12] {
	let mut filtered_report = report.to_vec();
	for index in 0..12 {
		let most_common = most_common_in_pos(&filtered_report, index);
		filtered_report = filtered_report
			.into_iter()
			.filter(|reading| (reading[index] == most_common) == most)
			.collect();
		if filtered_report.len() == 1 {
			return *filtered_report.get(0).unwrap();
		}
	}
	panic!();
}

fn most_common_in_pos(report: &[[bool; 12]], pos: usize) -> bool {
	report.iter().filter(|reading| reading[pos]).count() * 2 >= report.len()
}

fn make_number(reading: &[bool; 12]) -> u32 {
	reading
		.iter()
		.rev()
		.enumerate()
		.fold(0, |acc, (index, &on)| {
			if on {
				acc + 2_u32.pow(index as u32)
			} else {
				acc
			}
		})
}
