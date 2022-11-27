fn main() {
	shared::print_answers(1, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut state = State::default();
	for instruction in input.split(", ").map(Instruction::parse) {
		state.apply_instruction(instruction);
	}
	state.compute_distance()
}

fn get_answer_2(input: &str) -> u32 {
	let mut state = State::default();
	let mut visited = std::collections::HashSet::new();
	for instruction in input.split(", ").map(Instruction::parse) {
		for place in state.apply_instruction_and_list_visited(instruction) {
			if !visited.insert(place) {
				return place.0.unsigned_abs() + place.1.unsigned_abs();
			}
		}
	}
	panic!("Never visited a position twice.");
}

enum TurningDirection {
	Left,
	Right,
}

impl TurningDirection {
	fn parse(str: &str) -> Self {
		match str {
			"L" => Self::Left,
			"R" => Self::Right,
			_ => panic!("Invalid turning direction."),
		}
	}
}

struct Instruction {
	turning_direction: TurningDirection,
	distance: i32,
}

impl Instruction {
	fn parse(str: &str) -> Self {
		let (dir, num) = str.split_at(1);
		let turning_direction = TurningDirection::parse(dir);
		let distance = num.parse().expect("Could not parse step count.");
		Self {
			turning_direction,
			distance,
		}
	}
}

#[derive(Default, Clone, Copy)]
enum Orientation {
	#[default]
	North,
	East,
	South,
	West,
}

impl Orientation {
	fn rotate(&mut self, direction: TurningDirection) {
		*self = match (*self, direction) {
			(Self::North, TurningDirection::Right) => Self::East,
			(Self::East, TurningDirection::Right) => Self::South,
			(Self::South, TurningDirection::Right) => Self::West,
			(Self::West, TurningDirection::Right) => Self::North,
			(Self::North, TurningDirection::Left) => Self::West,
			(Self::East, TurningDirection::Left) => Self::North,
			(Self::South, TurningDirection::Left) => Self::East,
			(Self::West, TurningDirection::Left) => Self::South,
		}
	}
}

#[derive(Default)]
struct State {
	position: (i32, i32),
	orientation: Orientation,
}

impl State {
	fn apply_instruction(&mut self, instruction: Instruction) {
		self.orientation.rotate(instruction.turning_direction);
		match self.orientation {
			Orientation::North => self.position.1 += instruction.distance,
			Orientation::East => self.position.0 += instruction.distance,
			Orientation::South => self.position.1 -= instruction.distance,
			Orientation::West => self.position.0 -= instruction.distance,
		}
	}
	fn apply_instruction_and_list_visited(&mut self, instruction: Instruction) -> Vec<(i32, i32)> {
		let mut visited = Vec::new();
		self.orientation.rotate(instruction.turning_direction);
		for _ in 1..=instruction.distance {
			match self.orientation {
				Orientation::North => self.position.1 += 1,
				Orientation::East => self.position.0 += 1,
				Orientation::South => self.position.1 -= 1,
				Orientation::West => self.position.0 -= 1,
			}
			visited.push(self.position);
		}
		visited
	}
	fn compute_distance(&self) -> u32 {
		self.position.0.unsigned_abs() + self.position.1.unsigned_abs()
	}
}
