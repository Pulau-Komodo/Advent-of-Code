use shared::Vec2;

fn main() {
	shared::print_answers(22, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let (map, instructions) = input.split_once("\n\n").unwrap();
	let map = Map::from_str(map);
	let mut state = State {
		position: map.find_start(),
		heading: Heading::East,
	};
	for instruction in InstructionsParser::new(instructions.as_bytes()) {
		state.progress(&instruction, &map);
	}
	state.password()
}

fn get_answer_2(input: &str) -> usize {
	let (map, instructions) = input.split_once("\n\n").unwrap();
	let map = Map::from_str(map);
	let cube = CubeData::new();
	let mut state = State {
		position: map.find_start(),
		heading: Heading::East,
	};
	for instruction in InstructionsParser::new(instructions.as_bytes()) {
		state.progress_cube(&instruction, &map, &cube);
	}
	state.password()
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
	TurnLeft,
	TurnRight,
	MoveForward(usize),
}

struct InstructionsParser<'l> {
	input: &'l [u8],
	pos: usize,
}

impl<'l> InstructionsParser<'l> {
	fn new(input: &'l [u8]) -> Self {
		Self { input, pos: 0 }
	}
}

impl<'l> Iterator for InstructionsParser<'l> {
	type Item = Instruction;
	fn next(&mut self) -> Option<Self::Item> {
		let instruction = match self.input.get(self.pos)? {
			b'L' => Instruction::TurnLeft,
			b'R' => Instruction::TurnRight,
			byte => {
				let mut num = (byte - b'0') as usize;
				while (b'0'..=b'9').contains(self.input.get(self.pos + 1).unwrap_or(&0)) {
					num *= 10;
					num += (self.input[self.pos + 1] - b'0') as usize;
					self.pos += 1;
				}
				Instruction::MoveForward(num)
			}
		};
		self.pos += 1;
		Some(instruction)
	}
}

#[derive(Debug, Clone, Copy)]
enum Tile {
	Void,
	Wall,
	Clear,
}

impl Tile {
	fn from_char(char: char) -> Self {
		match char {
			' ' => Self::Void,
			'#' => Self::Wall,
			'.' => Self::Clear,
			_ => panic!("Illegal character in input."),
		}
	}
}

struct Map {
	rows: Vec<Vec<Tile>>,
}

impl Map {
	fn from_str(str: &str) -> Self {
		let rows = [vec![Tile::Void]]
			.into_iter()
			.chain(str.lines().map(|line| {
				[Tile::Void]
					.into_iter()
					.chain(line.chars().map(Tile::from_char))
					.collect()
			}))
			.collect();
		Self { rows }
	}
	fn get_tile(&self, pos: Vec2<usize>) -> Tile {
		let Some(row) = self.rows.get(pos.y) else { return Tile::Void; };
		*row.get(pos.x).unwrap_or(&Tile::Void)
	}
	fn find_start(&self) -> Vec2<usize> {
		self.rows
			.iter()
			.enumerate()
			.flat_map(|(y, row)| {
				row.iter()
					.enumerate()
					.map(move |(x, tile)| (Vec2 { x, y }, tile))
			})
			.find_map(|(point, tile)| matches!(tile, Tile::Clear).then_some(point))
			.unwrap()
	}
}

#[derive(Debug, Clone, Copy)]
enum Heading {
	North,
	East,
	South,
	West,
}

impl Heading {
	fn turn_right(&mut self) {
		match self {
			Self::North => *self = Self::East,
			Self::East => *self = Self::South,
			Self::South => *self = Self::West,
			Self::West => *self = Self::North,
		}
	}
	fn turn_left(&mut self) {
		match self {
			Self::North => *self = Self::West,
			Self::East => *self = Self::North,
			Self::South => *self = Self::East,
			Self::West => *self = Self::South,
		}
	}
	fn into_offset(self) -> Vec2<usize> {
		match self {
			Self::North => Vec2::new(1, 0),
			Self::East => Vec2::new(2, 1),
			Self::South => Vec2::new(1, 2),
			Self::West => Vec2::new(0, 1),
		}
	}
}

struct State {
	position: Vec2<usize>,
	heading: Heading,
}

impl State {
	fn progress(&mut self, instruction: &Instruction, map: &Map) {
		let (offset, &amount) = match instruction {
			Instruction::TurnLeft => return self.heading.turn_left(),
			Instruction::TurnRight => return self.heading.turn_right(),
			Instruction::MoveForward(amount) => (self.heading.into_offset(), amount),
		};
		for _ in 0..amount {
			let mut new_position = self.position + offset - Vec2::ONE;
			match map.get_tile(new_position) {
				Tile::Void => {
					let mut search_position = new_position;
					loop {
						let next_search_position = search_position + Vec2::ONE - offset;
						if matches!(map.get_tile(next_search_position), Tile::Void) {
							break;
						}
						search_position = next_search_position;
					}
					match map.get_tile(search_position) {
						Tile::Void => unreachable!(),
						Tile::Wall => break,
						Tile::Clear => (),
					}
					new_position = search_position;
				}
				Tile::Wall => break,
				Tile::Clear => (),
			}
			self.position = new_position;
		}
	}
	fn progress_cube(&mut self, instruction: &Instruction, map: &Map, cube: &CubeData) {
		let &amount = match instruction {
			Instruction::TurnLeft => return self.heading.turn_left(),
			Instruction::TurnRight => return self.heading.turn_right(),
			Instruction::MoveForward(amount) => amount,
		};
		for _ in 0..amount {
			let offset = self.heading.into_offset();
			let mut new_position = self.position + offset - Vec2::ONE;
			match map.get_tile(new_position) {
				Tile::Void => {
					let face_position = (self.position - Vec2::ONE) % cube.face_size;
					let face = (self.position - Vec2::ONE) / cube.face_size;
					let (prospective_heading, prospective_position, face) = if face == cube.top {
						match self.heading {
							Heading::North => {
								let prospective_face_position = face_position.swap_xy();
								(Heading::East, prospective_face_position, cube.back)
							}
							Heading::West => {
								let prospective_face_position = Vec2::new(face_position.x, cube.face_size - face_position.y - 1);
								(Heading::East, prospective_face_position, cube.left)
							}
							_ => panic!("Should not have been able to hit void travelling {:?} on face {:?}.", self.heading, face),
						}
					} else if face == cube.front {
						match self.heading {
							Heading::West => {
								let prospective_face_position = face_position.swap_xy();
								(Heading::South, prospective_face_position, cube.left)
							}
							Heading::East => {
								let prospective_face_position = face_position.swap_xy();
								(Heading::North, prospective_face_position, cube.right)
							}
							_ => panic!("Should not have been able to hit void travelling {:?} on face {:?}.", self.heading, face),
						}
					} else if face == cube.right {
						match self.heading {
							Heading::North => {
								let prospective_face_position = Vec2::new(face_position.x, cube.face_size - face_position.y - 1);
								(Heading::North, prospective_face_position, cube.back)
							}
							Heading::East => {
								let prospective_face_position = Vec2::new(face_position.x, cube.face_size - face_position.y - 1);
								(Heading::West, prospective_face_position, cube.bottom)
							}
							Heading::South => {
								let prospective_face_position = face_position.swap_xy();
								(Heading::West, prospective_face_position, cube.front)
							}
							_ => panic!("Should not have been able to hit void travelling {:?} on face {:?}.", self.heading, face),
						}
					} else if face == cube.back {
						match self.heading {
							Heading::East => {
								let prospective_face_position = face_position.swap_xy();
								(Heading::North, prospective_face_position, cube.bottom)
							}
							Heading::South => {
								let prospective_face_position = Vec2::new(face_position.x, cube.face_size - face_position.y - 1);
								(Heading::South, prospective_face_position, cube.right)
							}
							Heading::West => {
								let prospective_face_position = face_position.swap_xy();
								(Heading::South, prospective_face_position, cube.top)
							}
							_ => panic!("Should not have been able to hit void travelling {:?} on face {:?}.", self.heading, face),
						}
					} else if face == cube.left {
						match self.heading {
							Heading::North => {
								let prospective_face_position = face_position.swap_xy();
								(Heading::East, prospective_face_position, cube.front)
							}
							Heading::West => {
								let prospective_face_position = Vec2::new(face_position.x, cube.face_size - face_position.y - 1);
								(Heading::East, prospective_face_position, cube.top)
							}
							_ => panic!("Should not have been able to hit void travelling {:?} on face {:?}.", self.heading, face),
						}
					} else if face == cube.bottom {
						match self.heading {
							Heading::East => {
								let prospective_face_position = Vec2::new(face_position.x, cube.face_size - face_position.y - 1);
								(Heading::West, prospective_face_position, cube.right)
							}
							Heading::South => {
								let prospective_face_position = face_position.swap_xy();
								(Heading::West, prospective_face_position, cube.back)
							}
							_ => panic!("Should not have been able to hit void travelling {:?} on face {:?}.", self.heading, face),
						}
					} else {
						panic!("Coordinates must match a face.")
					};
					let prospective_position =
						prospective_position + face * cube.face_size + Vec2::ONE;
					match map.get_tile(prospective_position) {
						Tile::Void => panic!("There should not be a void after switching cube sides. Switched from {:?} to {:?}.", self.position, prospective_position),
						Tile::Wall => break,
						Tile::Clear => (),
					}
					new_position = prospective_position;
					self.heading = prospective_heading;
				}
				Tile::Wall => break,
				Tile::Clear => (),
			}
			self.position = new_position;
		}
	}
	fn password(&self) -> usize {
		self.position.y * 1000
			+ self.position.x * 4
			+ match self.heading {
				Heading::North => 3,
				Heading::East => 0,
				Heading::South => 1,
				Heading::West => 2,
			}
	}
}

struct CubeData {
	top: Vec2<usize>,
	front: Vec2<usize>,
	right: Vec2<usize>,
	back: Vec2<usize>,
	left: Vec2<usize>,
	bottom: Vec2<usize>,
	face_size: usize,
}

impl CubeData {
	fn new() -> Self {
		Self {
			top: Vec2::new(1, 0),
			front: Vec2::new(1, 1),
			right: Vec2::new(2, 0),
			back: Vec2::new(0, 3),
			left: Vec2::new(0, 2),
			bottom: Vec2::new(1, 2),
			face_size: 50,
		}
	}
	fn _example() -> Self {
		Self {
			top: Vec2::new(2, 0),
			front: Vec2::new(2, 1),
			right: Vec2::new(3, 2),
			back: Vec2::new(0, 1),
			left: Vec2::new(1, 1),
			bottom: Vec2::new(2, 2),
			face_size: 4,
		}
	}
}
