fn main() {
	shared::print_answers(6, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let (time, distance) = input.split_once('\n').unwrap();
	time.split_ascii_whitespace()
		.zip(distance.split_ascii_whitespace())
		.skip(1)
		.map(|(time, distance)| Race::new(time, distance))
		.map(|race| race.number_of_ways_to_win())
		.product()
}

fn get_answer_2(input: &str) -> u64 {
	let (time, distance) = input.split_once('\n').unwrap();
	let time = parse_into_number_ignoring_other_chars(time);
	let distance = parse_into_number_ignoring_other_chars(distance);
	let race = Race { time, distance };
	race.number_of_ways_to_win()
}

struct Race {
	time: u64,
	distance: u64,
}

impl Race {
	fn new(time: &str, distance: &str) -> Self {
		Self {
			time: time.parse().unwrap(),
			distance: distance.parse().unwrap(),
		}
	}
	fn number_of_ways_to_win(&self) -> u64 {
		(1..self.time)
			.map(|time| (self.time - time) * time)
			.filter(|&distance| distance > self.distance)
			.count() as u64
	}
}

fn parse_into_number_ignoring_other_chars(str: &str) -> u64 {
	str.bytes().fold(0, |acc, byte| match byte {
		b'0'..=b'9' => acc * 10 + (byte - b'0') as u64,
		_ => acc,
	})
}
