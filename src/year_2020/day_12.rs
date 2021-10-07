enum Instruction {
	North(i64),
	South(i64),
	East(i64),
	West(i64),
	Forward(i64),
	TurnLeft,
	TurnRight,
	TurnAround,
}

fn parse_instructions(input: String) -> Vec<Instruction> {
	input
		.lines()
		.map(|line| {
			use Instruction::*;
			let (letter, number) = line.split_at(1);
			let number = number.parse().unwrap();
			match (letter, number) {
				("N", n) => North(n),
				("S", n) => South(n),
				("E", n) => East(n),
				("W", n) => West(n),
				("F", n) => Forward(n),
				("L", 90) | ("R", 270) => TurnLeft,
				("R", 90) | ("L", 270) => TurnRight,
				("L", 180) | ("R", 180) => TurnAround,
				("R", _) | ("L", _) => panic!("Oops, invalid assumption about turns"),
				_ => panic!("Invalid input"),
			}
		})
		.collect()
}

enum Direction {
	North,
	South,
	East,
	West,
}

fn execute_instructions(instructions: &[Instruction]) -> (i64, i64) {
	let mut x = 0;
	let mut y = 0;
	let mut direction = Direction::East;
	for instruction in instructions {
		use Instruction::*;
		match instruction {
			North(n) => y += n,
			South(n) => y -= n,
			East(n) => x += n,
			West(n) => x -= n,
			Forward(n) => match direction {
				Direction::North => y += n,
				Direction::South => y -= n,
				Direction::East => x += n,
				Direction::West => x -= n,
			},
			TurnLeft => {
				direction = match direction {
					Direction::North => Direction::West,
					Direction::South => Direction::East,
					Direction::East => Direction::North,
					Direction::West => Direction::South,
				}
			}
			TurnRight => {
				direction = match direction {
					Direction::North => Direction::East,
					Direction::South => Direction::West,
					Direction::East => Direction::South,
					Direction::West => Direction::North,
				}
			}
			TurnAround => {
				direction = match direction {
					Direction::North => Direction::South,
					Direction::South => Direction::North,
					Direction::East => Direction::West,
					Direction::West => Direction::East,
				}
			}
		}
	}
	(x, y)
}

fn execute_instructions_2(instructions: &[Instruction]) -> (i64, i64) {
	let mut ship_x = 0;
	let mut ship_y = 0;
	let mut waypoint_x = 10;
	let mut waypoint_y = 1;
	for instruction in instructions {
		use Instruction::*;
		match instruction {
			North(n) => waypoint_y += n,
			South(n) => waypoint_y -= n,
			East(n) => waypoint_x += n,
			West(n) => waypoint_x -= n,
			Forward(n) => {
				ship_x += waypoint_x * n;
				ship_y += waypoint_y * n;
			}
			TurnLeft => {
				let temp = waypoint_y;
				waypoint_y = waypoint_x;
				waypoint_x = -temp;
			}
			TurnRight => {
				let temp = waypoint_y;
				waypoint_y = -waypoint_x;
				waypoint_x = temp;
			}
			TurnAround => {
				waypoint_x *= -1;
				waypoint_y *= -1;
			}
		}
	}
	(ship_x, ship_y)
}

pub fn get_answers(input: String) -> String {
	let instructions = parse_instructions(input);
	let (x, y) = execute_instructions(&instructions);
	let sum = x.abs() + y.abs();
	let (x, y) = execute_instructions_2(&instructions);
	let sum_2 = x.abs() + y.abs();
	format!("1: {}, 2: {}", sum, sum_2)
}
