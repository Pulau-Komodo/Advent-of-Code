fn main() {
	shared::print_answers(2, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut horizontal_pos = 0;
	let mut depth = 0;
	for instruction in parse_input(input) {
		match instruction {
			Command::Forward(x) => horizontal_pos += x,
			Command::Down(x) => depth += x,
			Command::Up(x) => depth -= x,
		}
	}
	horizontal_pos * depth
}

fn get_answer_2(input: &str) -> u32 {
	let mut aim: i32 = 0;
	let mut horizontal_pos = 0;
	let mut depth: i32 = 0;
	for instruction in parse_input(input) {
		match instruction {
			Command::Forward(x) => {
				horizontal_pos += x;
				depth += aim * x as i32
			}
			Command::Down(x) => aim += x as i32,
			Command::Up(x) => aim -= x as i32,
		}
	}
	horizontal_pos * depth as u32
}

enum Command {
	Forward(u32),
	Down(u32),
	Up(u32),
}

fn parse_input(input: &str) -> impl Iterator<Item = Command> + '_ {
	input.lines().map(parse_line)
}

fn parse_line(line: &str) -> Command {
	let (direction, amount) = line.split_once(" ").unwrap();
	let amount = amount.parse().unwrap();
	match direction {
		"forward" => Command::Forward(amount),
		"down" => Command::Down(amount),
		"up" => Command::Up(amount),
		_ => panic!("Unexpected input"),
	}
}
