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

fn parse_input(input: &str) -> Vec<u16> {
	input
		.lines()
		.map(|line| u16::from_str_radix(line, 2).unwrap())
		.collect()
}

fn gamma_rate(report: &[u16]) -> u16 {
	let mut output = 0;
	for i in 0..12 {
		if most_common_in_pos(report, i) {
			output += 1 << i;
		}
	}
	output
}

fn get_power_consumption(report: &[u16]) -> u32 {
	let gamma_rate = gamma_rate(report);
	let epsilon_rate = !gamma_rate & 0b0000_1111_1111_1111;
	gamma_rate as u32 * epsilon_rate as u32
}

fn get_life_support_rating(report: &[u16]) -> u32 {
	let oxygen_generator_rating = find_rating(report, true);
	let co2_scrubber_rating = find_rating(report, false);
	oxygen_generator_rating as u32 * co2_scrubber_rating as u32
}

fn find_rating(report: &[u16], most: bool) -> u16 {
	let mut filtered_report = report.to_vec();
	for i in (0..12).rev() {
		let most_common = most_common_in_pos(&filtered_report, i);
		let target = most == most_common;
		filtered_report = filtered_report
			.into_iter()
			.filter(|reading| (1 << i & reading != 0) == target)
			.collect();
		if filtered_report.len() == 1 {
			return *filtered_report.get(0).unwrap();
		}
	}
	panic!("Did not found a unique answer");
}

fn most_common_in_pos(report: &[u16], pos: u16) -> bool {
	let half_len = if report.len() % 2 == 0 {
		report.len() / 2
	} else {
		(report.len() + 1) / 2
	};
	let mut zeroes = 0;
	let mut ones = 0;
	for reading in report {
		if 1 << pos & reading != 0 {
			ones += 1;
		} else {
			zeroes += 1;
		}
		if ones >= half_len {
			return true;
		} else if zeroes > half_len {
			return false;
		}
	}
	ones >= half_len
}
