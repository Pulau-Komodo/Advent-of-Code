use shared::{Direction, Grid, Point};

fn main() {
	shared::print_answers(13, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> Point<usize> {
	let grid = make_grid(input);
	let mut carts = get_carts(input);
	loop {
		for index in 0..carts.len() {
			let (pos, cart) = &mut carts[index];
			match grid.get_point(*pos) {
				Tile::Nothing => {}
				Tile::TurnBottomLeftTopRight => match cart.direction {
					Direction::Up | Direction::Down => cart.direction.turn_right_mut(),
					Direction::Left | Direction::Right => cart.direction.turn_left_mut(),
				},
				Tile::TurnTopLeftBottomRight => match cart.direction {
					Direction::Up | Direction::Down => cart.direction.turn_left_mut(),
					Direction::Left | Direction::Right => cart.direction.turn_right_mut(),
				},
				Tile::Intersection => {
					match cart.turn_direction {
						TurnDirection::Left => cart.direction.turn_left_mut(),
						TurnDirection::Straight => (),
						TurnDirection::Right => cart.direction.turn_right_mut(),
					}
					cart.turn_direction = cart.turn_direction.next();
				}
			}
			match cart.direction {
				Direction::Up => pos.y -= 1,
				Direction::Right => pos.x += 1,
				Direction::Down => pos.y += 1,
				Direction::Left => pos.x -= 1,
			}
			let pos = *pos;
			if carts
				.iter()
				.enumerate()
				.any(|(other_index, (other_pos, _))| index != other_index && pos == *other_pos)
			{
				return pos;
			}
		}
		carts.sort_by_key(|(pos, _cart)| (pos.y, pos.x));
	}
}

fn get_answer_2(input: &str) -> Point<usize> {
	let grid = make_grid(input);
	let mut carts = get_carts(input);
	loop {
		let mut remove = Vec::new();
		for index in 0..carts.len() {
			if remove.contains(&index) {
				continue;
			}
			let (pos, cart) = &mut carts[index];
			match grid.get_point(*pos) {
				Tile::Nothing => {}
				Tile::TurnBottomLeftTopRight => match cart.direction {
					Direction::Up | Direction::Down => cart.direction.turn_right_mut(),
					Direction::Left | Direction::Right => cart.direction.turn_left_mut(),
				},
				Tile::TurnTopLeftBottomRight => match cart.direction {
					Direction::Up | Direction::Down => cart.direction.turn_left_mut(),
					Direction::Left | Direction::Right => cart.direction.turn_right_mut(),
				},
				Tile::Intersection => {
					match cart.turn_direction {
						TurnDirection::Left => cart.direction.turn_left_mut(),
						TurnDirection::Straight => (),
						TurnDirection::Right => cart.direction.turn_right_mut(),
					}
					cart.turn_direction = cart.turn_direction.next();
				}
			}
			match cart.direction {
				Direction::Up => pos.y -= 1,
				Direction::Right => pos.x += 1,
				Direction::Down => pos.y += 1,
				Direction::Left => pos.x -= 1,
			}
			let pos = *pos;
			if let Some((other_index, _)) = carts
				.iter()
				.enumerate()
				.find(|(other_index, (other_pos, _))| index != *other_index && pos == *other_pos)
			{
				remove.push(index);
				remove.push(other_index);
			}
		}
		remove.sort();
		for index in remove.into_iter().rev() {
			carts.remove(index);
		}
		if carts.len() == 1 {
			return carts[0].0;
		}
		carts.sort_by_key(|(pos, _cart)| (pos.y, pos.x));
	}
}

#[derive(Debug, Clone, Copy)]
enum TurnDirection {
	Left,
	Straight,
	Right,
}

impl TurnDirection {
	fn next(self) -> Self {
		match self {
			Self::Left => Self::Straight,
			Self::Straight => Self::Right,
			Self::Right => Self::Left,
		}
	}
}

#[derive(Debug, Clone, Copy)]
struct Cart {
	direction: Direction,
	turn_direction: TurnDirection,
}

impl Cart {
	fn try_from_byte(byte: u8) -> Option<Self> {
		let direction = match byte {
			b'^' => Direction::Up,
			b'>' => Direction::Right,
			b'v' => Direction::Down,
			b'<' => Direction::Left,
			_ => return None,
		};
		Some(Self {
			direction,
			turn_direction: TurnDirection::Left,
		})
	}
}

#[derive(Debug, Clone, Copy)]
enum Tile {
	Nothing,
	TurnBottomLeftTopRight,
	TurnTopLeftBottomRight,
	Intersection,
}

fn make_grid(input: &str) -> Grid<Tile> {
	let grid = Grid::new(input.lines().map(|line| {
		line.bytes().map(move |byte| match byte {
			b' ' | b'|' | b'-' | b'^' | b'>' | b'v' | b'<' => Tile::Nothing,
			b'/' => Tile::TurnBottomLeftTopRight,
			b'\\' => Tile::TurnTopLeftBottomRight,
			b'+' => Tile::Intersection,
			_ => panic!(),
		})
	}));
	grid
}

fn get_carts(input: &str) -> Vec<(Point<usize>, Cart)> {
	let mut carts = Vec::new();
	for (x, y, byte) in input
		.lines()
		.enumerate()
		.flat_map(|(y, line)| line.bytes().enumerate().map(move |(x, byte)| (x, y, byte)))
	{
		if let Some(cart) = Cart::try_from_byte(byte) {
			let point = Point::new(x, y);
			carts.push((point, cart));
		}
	}
	carts
}
