enum Direction {
	East,
	SouthEast,
	SouthWest,
	West,
	NorthWest,
	NorthEast,
}

fn iterate_input(input: &str) -> impl Iterator<Item = Vec<Direction>> + '_ {
	use Direction::*;
	input.lines().map(|line| {
		let mut previous_char = '.';
		line.chars()
			.filter_map(|char| {
				let direction = match (previous_char, char) {
					('s', 'e') => Some(SouthEast),
					('s', 'w') => Some(SouthWest),
					('n', 'w') => Some(NorthWest),
					('n', 'e') => Some(NorthEast),
					(_, 'e') => Some(East),
					(_, 'w') => Some(West),
					_ => None,
				};
				previous_char = char;
				direction
			})
			.collect()
	})
}

fn find_coordinates(instructions: &[Direction]) -> (i32, i32) {
	let mut position = (0, 0);
	for step in instructions {
		match step {
			Direction::East => position.0 += 1,
			Direction::SouthEast => position.1 += 1,
			Direction::SouthWest => position = (position.0 - 1, position.1 + 1),
			Direction::West => position.0 -= 1,
			Direction::NorthWest => position.1 -= 1,
			Direction::NorthEast => position = (position.0 + 1, position.1 - 1),
		}
	}
	position
}

struct TileFloor {
	black_tiles: std::collections::HashSet<(i32, i32)>,
}

impl TileFloor {
	fn from_str(str: &str) -> Self {
		let mut black_tiles = std::collections::HashSet::new();
		for instructions in iterate_input(str) {
			let coordinates = find_coordinates(&instructions);
			if black_tiles.contains(&coordinates) {
				black_tiles.remove(&coordinates);
			} else {
				black_tiles.insert(coordinates);
			}
		}
		TileFloor { black_tiles }
	}
	fn progress_day(&mut self) {
		const ADJECENT: [(i32, i32); 6] = [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)];
		let (min_x, min_y, max_x, max_y) = self.black_tiles.iter().fold(
			(i32::MAX, i32::MAX, i32::MIN, i32::MIN),
			|(min_x, min_y, max_x, max_y), coordinates| {
				(
					min_x.min(coordinates.0),
					min_y.min(coordinates.1),
					max_x.max(coordinates.0),
					max_y.max(coordinates.1),
				)
			},
		);
		let (min_x, min_y, max_x, max_y) = (min_x - 1, min_y - 1, max_x + 1, max_y + 1);
		let mut new_black_tiles = std::collections::HashSet::new();
		for y in min_y..=max_y {
			for x in min_x..=max_x {
				let mut nearby_count = 0;
				for adjacent in ADJECENT {
					if self.black_tiles.contains(&(x + adjacent.0, y + adjacent.1)) {
						nearby_count += 1;
					}
					if nearby_count > 2 {
						break;
					}
				}
				if nearby_count == 2 || nearby_count == 1 && self.black_tiles.contains(&(x, y)) {
					new_black_tiles.insert((x, y));
				}
			}
		}
		self.black_tiles = new_black_tiles;
	}
}

pub fn get_answers(input: String) -> String {
	let mut floor = TileFloor::from_str(&input);
	let initial_count = floor.black_tiles.len();
	for _ in 0..100 {
		floor.progress_day();
	}
	format!("1: {}, 2: {}", initial_count, floor.black_tiles.len())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample_input() {
		let input = "sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew".to_string();
		assert_eq!(get_answers(input), "1: 10, 2: 2208".to_string());
	}
}
