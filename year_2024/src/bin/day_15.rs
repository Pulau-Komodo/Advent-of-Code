use shared::{Direction, Grid, Offset, Point};

fn main() {
	shared::print_answers(15, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let (map, moves) = input.split_once("\n\n").unwrap();

	let mut robot_position = find_robot(map);
	let mut grid = Grid::new(map.lines().map(|line| line.bytes().map(Tile::from_byte)));

	for robot_move in moves
		.lines()
		.flat_map(|line| line.bytes())
		.map(Direction::from_byte)
	{
		let new_position = apply_offset(robot_position, robot_move.into_offset());
		match grid.get_point(new_position) {
			Tile::Wall => (),
			Tile::Empty => robot_position = new_position,
			Tile::Box => {
				if let Some(space) = find_empty_space(new_position, robot_move, &grid) {
					*grid.get_point_mut(space) = Tile::Box;
					*grid.get_point_mut(new_position) = Tile::Empty;
					robot_position = new_position;
				}
			}
		}
	}

	grid.iter_with_points::<usize>()
		.filter(|(_, tile)| matches!(tile, Tile::Box))
		.map(|(point, _)| point.x + point.y * 100)
		.sum()
}

fn get_answer_2(input: &str) -> usize {
	let (map, moves) = input.split_once("\n\n").unwrap();

	let mut robot_position = find_robot(map);
	robot_position.x *= 2;
	let mut grid = Grid::new(
		map.lines()
			.map(|line| line.bytes().flat_map(TileV2::from_byte)),
	);

	for robot_move in moves
		.lines()
		.flat_map(|line| line.bytes())
		.map(Direction::from_byte)
	{
		let new_position = apply_offset(robot_position, robot_move.into_offset());
		match grid.get_point(new_position) {
			TileV2::Wall => continue,
			TileV2::Empty => (),
			// I handle this case separately because it's simpler, but...
			TileV2::BoxLeft | TileV2::BoxRight
				if robot_move == Direction::Left || robot_move == Direction::Right =>
			{
				let Some(space) = find_empty_space_v2(new_position, robot_move, &grid) else {
					continue;
				};
				*grid.get_point_mut(space) = if robot_move == Direction::Left {
					TileV2::BoxLeft
				} else {
					TileV2::BoxRight
				};
				flip_boxes(new_position, space, robot_move, &mut grid);
				*grid.get_point_mut(new_position) = TileV2::Empty;
			}
			box_part @ TileV2::BoxLeft | box_part @ TileV2::BoxRight => {
				let other_part = if matches!(box_part, TileV2::BoxLeft) {
					apply_offset(new_position, Direction::Right.into_offset())
				} else {
					apply_offset(new_position, Direction::Left.into_offset())
				};
				let Some(boxes_to_move) =
					find_boxes_to_move([new_position, other_part], robot_move, &grid)
				else {
					continue;
				};
				move_boxes(boxes_to_move, robot_move, &mut grid);
			}
		}
		robot_position = new_position;
	}

	grid.iter_with_points::<usize>()
		.filter(|(_, tile)| matches!(tile, TileV2::BoxLeft))
		.map(|(point, _)| point.x + point.y * 100)
		.sum()
}

#[derive(Debug, Clone, Copy)]
enum Tile {
	Empty,
	Wall,
	Box,
}

impl Tile {
	fn from_byte(byte: u8) -> Self {
		match byte {
			b'.' | b'@' => Self::Empty,
			b'#' => Self::Wall,
			b'O' => Self::Box,
			_ => panic!(),
		}
	}
}

fn find_empty_space(
	starting_point: Point<usize>,
	direction: Direction,
	grid: &Grid<Tile>,
) -> Option<Point<usize>> {
	let mut check_space = starting_point;
	loop {
		check_space = apply_offset(check_space, direction.into_offset());
		let check_tile = grid.get_point(check_space);
		match check_tile {
			Tile::Wall => return None,
			Tile::Empty => return Some(check_space),
			Tile::Box => (),
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum TileV2 {
	Empty,
	Wall,
	BoxLeft,
	BoxRight,
}

impl TileV2 {
	fn from_byte(byte: u8) -> [Self; 2] {
		match byte {
			b'.' | b'@' => [Self::Empty, Self::Empty],
			b'#' => [Self::Wall, Self::Wall],
			b'O' => [Self::BoxLeft, Self::BoxRight],
			_ => panic!(),
		}
	}
}

fn find_empty_space_v2(
	starting_point: Point<usize>,
	direction: Direction,
	grid: &Grid<TileV2>,
) -> Option<Point<usize>> {
	let mut check_space = starting_point;
	loop {
		check_space = apply_offset(check_space, direction.into_offset());
		let check_tile = grid.get_point(check_space);
		match check_tile {
			TileV2::Wall => return None,
			TileV2::Empty => return Some(check_space),
			TileV2::BoxLeft | TileV2::BoxRight => (),
		}
	}
}

fn find_boxes_to_move(
	parts: [Point<usize>; 2],
	direction: Direction,
	grid: &Grid<TileV2>,
) -> Option<Vec<Vec<Point<usize>>>> {
	let mut boxes_to_move = Vec::from([Vec::from(parts)]);

	loop {
		let mut new_frontier = Vec::new();
		for box_part in boxes_to_move.last().unwrap() {
			let next_point = apply_offset(*box_part, direction.into_offset());
			let next_tile = grid.get_point(next_point);
			match next_tile {
				TileV2::Empty => (),
				TileV2::Wall => return None,
				TileV2::BoxLeft => new_frontier.extend([
					next_point,
					apply_offset(next_point, Direction::Right.into_offset()),
				]),
				TileV2::BoxRight => new_frontier.extend([
					next_point,
					apply_offset(next_point, Direction::Left.into_offset()),
				]),
			}
		}
		if new_frontier.is_empty() {
			break;
		}
		new_frontier.sort_unstable_by_key(|point| (point.x, point.y));
		new_frontier.dedup();
		boxes_to_move.push(new_frontier);
	}
	Some(boxes_to_move)
}

fn flip_boxes(
	start: Point<usize>,
	end: Point<usize>,
	direction: Direction,
	grid: &mut Grid<TileV2>,
) {
	let mut flip_space = start;
	loop {
		flip_space = apply_offset(flip_space, direction.into_offset());
		if flip_space == end {
			break;
		}
		let flip_tile = grid.get_point_mut(flip_space);
		if matches!(flip_tile, TileV2::BoxLeft) {
			*flip_tile = TileV2::BoxRight;
		} else {
			*flip_tile = TileV2::BoxLeft;
		}
	}
}

fn move_boxes(boxes: Vec<Vec<Point<usize>>>, direction: Direction, grid: &mut Grid<TileV2>) {
	for row in boxes.into_iter().rev() {
		for box_part in row {
			let spot = grid.get_point_mut(box_part);
			let part = *spot;
			*spot = TileV2::Empty;
			*grid.get_point_mut(apply_offset(box_part, direction.into_offset())) = part;
		}
	}
}

fn find_robot(map: &str) -> Point<usize> {
	map.lines()
		.enumerate()
		.flat_map(|(y, line)| {
			line.bytes()
				.enumerate()
				.map(move |(x, byte)| (byte == b'@').then_some(Point::new(x, y)))
		})
		.find_map(std::convert::identity)
		.unwrap()
}

fn apply_offset(point: Point<usize>, offset: Offset<i32>) -> Point<usize> {
	Point::new(
		(point.x as i32 + offset.x) as usize,
		(point.y as i32 + offset.y) as usize,
	)
}
