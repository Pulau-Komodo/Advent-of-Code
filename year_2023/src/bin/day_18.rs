use shared::Point;

fn main() {
	shared::print_answers(18, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let instructions: Vec<_> = input.lines().map(Instruction::from_line).collect();
	let (heights, lines) = process_instructions(instructions);
	get_surface(heights, lines)
}

fn get_answer_2(input: &str) -> u64 {
	let instructions: Vec<_> = input.lines().map(Instruction::from_line_v2).collect();
	let (heights, lines) = process_instructions(instructions);
	get_surface(heights, lines)
}

#[derive(Debug, Clone, Copy)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	fn from_byte(byte: u8) -> Self {
		match byte {
			b'U' | b'3' => Self::Up,
			b'D' | b'1' => Self::Down,
			b'L' | b'2' => Self::Left,
			b'R' | b'0' => Self::Right,
			_ => panic!("Unexpected direction byte"),
		}
	}
	fn is_vertical(self) -> bool {
		matches!(self, Self::Up | Self::Down)
	}
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
	direction: Direction,
	distance: i64,
}

impl Instruction {
	fn from_line(line: &str) -> Self {
		let (direction, rest) = line.split_once(' ').unwrap();
		let (distance, _rest) = rest.split_once(' ').unwrap();
		Self {
			direction: Direction::from_byte(direction.as_bytes()[0]),
			distance: distance.parse().unwrap(),
		}
	}
	fn from_line_v2(line: &str) -> Self {
		let (_, line) = line.rsplit_once(' ').unwrap();
		Self {
			direction: Direction::from_byte(line.as_bytes()[7]),
			distance: i64::from_str_radix(&line[2..7], 16).unwrap(),
		}
	}
}

// Hardcoded: my data goes clockwise

#[derive(Debug, Clone, Copy)]
struct VerticalLine {
	x: i64,
	y_low: i64,
	y_high: i64,
	is_upward: bool,
}

fn process_instructions(instructions: Vec<Instruction>) -> (Vec<i64>, Vec<VerticalLine>) {
	let mut position = Point::new(0, 0);
	let mut lines = Vec::new();
	let mut heights = Vec::new();
	// Hardcoded: loop starts with up and ends with right
	for i in 0..instructions.len() {
		let instruction = instructions[i];

		if instruction.direction.is_vertical() {
			let prev_direction = if i == 0 {
				*instructions.last().unwrap()
			} else {
				instructions[i - 1]
			}
			.direction;
			let next_direction = instructions[(i + 1) % instructions.len()].direction;
			let x = position.x;
			let y_low = match (prev_direction, instruction.direction, next_direction) {
				(Direction::Left, Direction::Up, _) => position.y,
				(Direction::Right, Direction::Up, _) => position.y + 1,
				(_, Direction::Down, Direction::Left) => position.y - instruction.distance,
				(_, Direction::Down, Direction::Right) => position.y - instruction.distance + 1,
				_ => panic!("Unexpected combination of directions"),
			};
			let y_high = match (prev_direction, instruction.direction, next_direction) {
				(_, Direction::Up, Direction::Left) => position.y + instruction.distance - 1,
				(_, Direction::Up, Direction::Right) => position.y + instruction.distance,
				(Direction::Left, Direction::Down, _) => position.y - 1,
				(Direction::Right, Direction::Down, _) => position.y,
				_ => panic!("Unexpected combination of directions"),
			};
			lines.push(VerticalLine {
				x,
				y_low,
				y_high,
				is_upward: matches!(instruction.direction, Direction::Up),
			});
			heights.push(position.y);
		}

		match instruction.direction {
			Direction::Up => position.y += instruction.distance,
			Direction::Down => position.y -= instruction.distance,
			Direction::Left => position.x -= instruction.distance,
			Direction::Right => position.x += instruction.distance,
		}
	}
	lines.sort_by_key(|line| line.x);
	heights.sort_unstable();
	heights.dedup();
	(heights, lines)
}

fn get_surface(heights: Vec<i64>, lines: Vec<VerticalLine>) -> u64 {
	let mut surface = 0;
	for height in *heights.first().unwrap()..=*heights.last().unwrap() {
		// To do: don't actually check every individual height
		let mut start = None;
		for line in &lines {
			if (line.y_low..=line.y_high).contains(&height) {
				if line.is_upward {
					start = Some(line.x);
				} else {
					surface += (line.x
						- start.unwrap_or_else(|| {
							panic!(
								"No line left of line {} {}..={}, upward: {:?}",
								line.x, line.y_low, line.y_high, line.is_upward
							)
						}) + 1) as u64;
					start = None;
				}
			}
		}
	}
	surface
}
