fn main() {
	year_2020::print_answers(20, &[get_answer_1, get_answer_2]);
}

#[derive(Clone, Copy, Debug)]
enum Content {
	Empty,
	Something,
	Monster,
}

impl std::default::Default for Content {
	fn default() -> Self {
		Self::Empty
	}
}

#[derive(Default, Clone, Copy, Debug)]
struct Side {
	id: u16,
	structure: [bool; 10],
	unique: bool,
}

impl Side {
	fn from_array(array: [bool; 10]) -> Self {
		Self {
			id: number_side(array),
			structure: array,
			unique: false,
		}
	}
	fn matches(&self, other: &Self) -> bool {
		let mut other_structure = other.structure;
		other_structure.reverse();
		self.structure == other_structure
	}
}

#[derive(Default, Clone, Copy, Debug)]
struct Tile {
	id: u16,
	sides: [Side; 4],
	content: [[Content; 8]; 8],
	flipped: bool,
	rotations: u8,
	unique_sides: u8,
}

impl Tile {
	fn from_str(str: &str) -> Self {
		let mut lines = str.lines();
		let id = lines
			.next()
			.unwrap()
			.strip_prefix("Tile ")
			.unwrap()
			.strip_suffix(':')
			.unwrap()
			.parse()
			.unwrap();
		let mut top = [false; 10];
		let mut right = [false; 10];
		let mut bottom = [false; 10];
		let mut left = [false; 10];
		let mut content = [[Content::Empty; 8]; 8];
		for (line_index, line) in lines.enumerate() {
			for (char_index, char) in line.chars().enumerate() {
				if line_index == 0 {
					top[char_index] = char == '#';
				} else if line_index == 9 {
					bottom[9 - char_index] = char == '#';
				}
				if char_index == 0 {
					left[9 - line_index] = char == '#';
				} else if char_index == 9 {
					right[line_index] = char == '#';
				}
				if (1..9).contains(&line_index) && (1..9).contains(&char_index) && char == '#' {
					content[line_index - 1][char_index - 1] = Content::Something;
				}
			}
		}
		let top = Side::from_array(top);
		let right = Side::from_array(right);
		let bottom = Side::from_array(bottom);
		let left = Side::from_array(left);
		Tile {
			id,
			sides: [top, right, bottom, left],
			content,
			flipped: false,
			rotations: 0,
			unique_sides: 0,
		}
	}
	fn rotate(&mut self) {
		let [top, right, bottom, left] = self.sides;
		self.sides = [left, top, right, bottom];
		if !self.flipped {
			if self.rotations == 3 {
				self.rotations = 0;
			} else {
				self.rotations += 1;
			}
		} else if self.rotations == 0 {
			self.rotations = 3;
		} else {
			self.rotations -= 1;
		}
	}
	fn flip(&mut self) {
		let [mut top, mut right, mut bottom, mut left] = self.sides;
		top.structure.reverse();
		right.structure.reverse();
		bottom.structure.reverse();
		left.structure.reverse();
		self.sides = [top, left, bottom, right];
		self.flipped = !self.flipped;
	}
	fn rotate_content(&mut self) {
		let mut new_content = [[Content::Empty; 8]; 8];
		for (y, row) in self.content.iter().enumerate() {
			for (x, content) in row.iter().enumerate() {
				new_content[x][7 - y] = *content;
			}
		}
		self.content = new_content;
	}
	fn update_content_orientation(&mut self) {
		for _ in 0..self.rotations {
			self.rotate_content();
		}
		self.rotations = 0;
		if self.flipped {
			self.content.iter_mut().for_each(|row| row.reverse());
			self.flipped = false;
		}
	}
	fn shares_side_with(&self, other: &Self) -> bool {
		for side in self.sides {
			for other_side in other.sides {
				if side.id == other_side.id {
					return true;
				}
			}
		}
		false
	}
	fn _print(&self) {
		println!("Tile {}", self.id);
		for line in self.content {
			let line: String = line
				.iter()
				.map(|content| match content {
					Content::Empty => '.',
					Content::Something => '#',
					Content::Monster => 'O',
				})
				.collect();
			println!("{}", line);
		}
	}
}

fn number_side(side: [bool; 10]) -> u16 {
	let mut number = 0;
	for element in side {
		number <<= 1;
		if element {
			number += 1;
		}
	}
	let mut flipped_number = 0;
	for element in side.iter().rev() {
		flipped_number <<= 1;
		if *element {
			flipped_number += 1;
		}
	}
	number.min(flipped_number)
}

fn parse_tiles(tiles: &str) -> Vec<Tile> {
	tiles.split("\r\n\r\n").map(Tile::from_str).collect()
}

fn set_uniques(tiles: &mut [Tile]) {
	let mut side_count = std::collections::HashMap::with_capacity(312);
	for tile in tiles.iter() {
		for side in tile.sides {
			*side_count.entry(side.id).or_insert(0) += 1;
		}
	}
	for tile in tiles.iter_mut() {
		for side in tile.sides.iter_mut() {
			if side_count.get(&side.id) == Some(&1) {
				side.unique = true;
				tile.unique_sides += 1;
			}
		}
	}
}

fn get_answer_1(input: &str) -> String {
	let mut tiles = parse_tiles(input);
	set_uniques(&mut tiles);
	let product: u64 = tiles
		.iter()
		.filter(|tile| tile.sides.iter().filter(|side| side.unique).count() == 2)
		.map(|tile| tile.id as u64)
		.product();
	format!("{}", product)
}

fn arrange_grid<const SIZE: usize>(mut tiles: Vec<Tile>) -> [[Tile; SIZE]; SIZE] {
	let max_index: usize = SIZE - 1;
	let mut grid = [[Tile::default(); SIZE]; SIZE];
	let top_left_index = tiles
		.iter()
		.position(|tile| tile.sides.iter().filter(|side| side.unique).count() == 2)
		.unwrap();
	let mut top_left = tiles.remove(top_left_index);
	loop {
		if top_left.sides[0].unique && top_left.sides[3].unique {
			break;
		}
		top_left.rotate();
	}
	top_left.update_content_orientation();
	//println!("Top left tile is {}", top_left.id);
	grid[0][0] = top_left;
	for y in 0..SIZE {
		for x in 0..SIZE {
			if (0, 0) == (x, y) {
				continue;
			}
			let tile_index = tiles.iter_mut().position(|tile| {
				let corner = (max_index, 0) == (x, y)
					|| (0, max_index) == (x, y)
					|| (max_index, max_index) == (x, y);
				let side = !corner && (x == 0 || x == max_index || y == 0 || y == max_index);
				if corner && tile.unique_sides != 2
					|| side && tile.unique_sides != 1
					|| !side && !corner && tile.unique_sides != 0
				{
					return false;
				}
				let (other_tile, existing_side, new_side) = if x == 0 {
					(grid[y - 1][x], 2, 0)
				} else {
					(grid[y][x - 1], 1, 3)
				};
				if !tile.shares_side_with(&other_tile) {
					return false;
				}
				//println!("Considering {}", tile.id);
				let side_to_match = other_tile.sides[existing_side];
				for _ in 0..2 {
					for _ in 0..4 {
						if tile.sides[new_side].matches(&side_to_match) {
							return true;
						}
						tile.rotate();
					}
					tile.flip();
				}
				false
			});
			let mut tile = if let Some(tile_index) = tile_index {
				tiles.remove(tile_index)
			} else {
				Tile::default()
			};
			//println!("Placed tile {}", tile.id);
			tile.update_content_orientation();
			grid[y][x] = tile;
		}
	}
	//println!("Unplaced tiles: {} ({:?})", tiles.len(), tiles.iter().map(|tile| tile.id).collect::<Vec<_>>());
	grid
}

fn merge_content<const SIZE: usize, const CANVAS_SIZE: usize>(
	grid: [[Tile; SIZE]; SIZE],
) -> [[Content; CANVAS_SIZE]; CANVAS_SIZE] {
	let mut canvas = [[Content::Empty; CANVAS_SIZE]; CANVAS_SIZE];
	for (tile_y, tile_row) in grid.iter().enumerate() {
		for (tile_x, tile) in tile_row.iter().enumerate() {
			for (content_y, content_row) in tile.content.iter().enumerate() {
				for (content_x, content) in content_row.iter().enumerate() {
					let x = tile_x * 8 + content_x;
					let y = tile_y * 8 + content_y;
					canvas[y][x] = *content;
				}
			}
		}
	}
	canvas
}

struct SeaMonster {
	coordinates: std::collections::HashSet<(u8, u8)>,
	width: usize,
	height: usize,
}

impl SeaMonster {
	fn from_str(str: &str) -> Self {
		let coordinates = str
			.lines()
			.enumerate()
			.map(|(y, line)| {
				line.chars()
					.enumerate()
					.filter_map(move |(x, char)| match char {
						'#' => Some((x as u8, y as u8)),
						_ => None,
					})
			})
			.flatten()
			.collect();
		let width = str.lines().next().unwrap().chars().count();
		let height = str.lines().count();
		SeaMonster {
			coordinates,
			width,
			height,
		}
	}
	fn rotate(&mut self) {
		std::mem::swap(&mut self.height, &mut self.width);
		self.coordinates = self
			.coordinates
			.iter()
			.map(|(x, y)| (self.width as u8 - 1 - *y, *x))
			.collect();
	}
	fn flip(&mut self) {
		self.coordinates = self
			.coordinates
			.iter()
			.map(|(x, y)| (self.width as u8 - 1 - *x, *y))
			.collect();
	}
	fn _print(&self) {
		println!("height: {}, width: {}", self.height, self.width);
		for y in 0..self.height {
			let line: String = (0..self.width)
				.map(|x| {
					if self.coordinates.contains(&(x as u8, y as u8)) {
						'O'
					} else {
						'.'
					}
				})
				.collect();
			println!("{}", line);
		}
	}
}

fn mark_sea_monsters<const SIZE: usize>(image: &mut [[Content; SIZE]], mut monster: SeaMonster) {
	for _ in 0..2 {
		for _ in 0..4 {
			for y in 0..SIZE - monster.height + 1 {
				for x in 0..SIZE - monster.width + 1 {
					if monster.coordinates.iter().all(|(monster_x, monster_y)| {
						match image[y + *monster_y as usize][x + *monster_x as usize] {
							Content::Empty => false,
							Content::Something => true,
							Content::Monster => true,
						}
					}) {
						//println!("Monster found");
						for (monster_x, monster_y) in monster.coordinates.iter() {
							image[y + *monster_y as usize][x + *monster_x as usize] =
								Content::Monster;
						}
					}
				}
			}
			//println!("Rotating monster");
			monster.rotate();
		}
		//println!("Flipping monster");
		monster.flip();
	}
}

fn count_somethings<const SIZE: usize>(image: &[[Content; SIZE]]) -> u16 {
	image
		.iter()
		.map(|line| {
			line.iter()
				.filter(|content| matches!(content, Content::Something))
				.count() as u16
		})
		.sum()
}

fn _print_image<const SIZE: usize>(image: &[[Content; SIZE]]) {
	for line in image {
		let line: String = line
			.iter()
			.map(|content| match content {
				Content::Empty => '.',
				Content::Something => '#',
				Content::Monster => 'O',
			})
			.collect();
		println!("{}", line);
	}
}

fn get_answer_2(input: &str) -> String {
	let mut tiles = parse_tiles(input);
	set_uniques(&mut tiles);
	let grid = arrange_grid::<12>(tiles);
	let mut image = merge_content::<12, 96>(grid);
	let monster = year_2020::read_file_special(20, "monster");
	let monster = SeaMonster::from_str(&monster);
	mark_sea_monsters(&mut image, monster);
	//_print_image(&image);
	let roughness = count_somethings(&image);
	format!("{}", roughness)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn count_sides() {
		let input = year_2020::read_file(20);
		let tiles = parse_tiles(&input);
		println!("{} tiles", tiles.len());
		let mut side_map = std::collections::HashMap::<u16, u8>::new();
		let mut sides = 0;
		for tile in tiles {
			for side in tile.sides {
				sides += 1;
				*side_map.entry(side.id).or_insert(0) += 1;
			}
		}
		println!("{} sides", sides);
		println!("{:?}", side_map);
	}
	#[test]
	fn test_translation() {
		let mut tile = Tile::from_str("Tile 0:\n##########\n0###....##\n###......0\n0#.......0\n#....#...#\n0...#....#\n#..#....##\n0......#.0\n#.....#..0\n0000000000");
		tile._print();
		tile.rotate();
		tile.update_content_orientation();
		tile._print();
		tile.rotate();
		tile.update_content_orientation();
		tile._print();
		tile.rotate();
		tile.rotate();
		tile.update_content_orientation();
		tile._print();
		tile.flip();
		tile.update_content_orientation();
		tile._print();
		tile.rotate();
		tile.rotate();
		tile.flip();
		tile.update_content_orientation();
		tile._print();
	}
	#[test]
	fn rotate_monster() {
		let monster = year_2020::read_file_special(20, "monster");
		let mut monster = SeaMonster::from_str(&monster);
		monster._print();
		monster.rotate();
		monster._print();
	}
	#[test]
	fn matching() {
		let mut tile = Tile::from_str("Tile 0:\n##########\n.###....##\n###.......\n.#........\n#....#...#\n....#....#\n#..#....##\n.......#..\n#.....#...\n..........");
		tile.flip();
		tile.rotate();
		tile.rotate();
		tile.update_content_orientation();
		let mut tile_two = Tile::from_str("Tile 1:\n#.#####.#.\n####....#.\n###.......\n##.......#\n#....#...#\n....#....#\n#..#....#.\n#......#..\n#.....#..#\n........##");
		'outer: for _ in 0..2 {
			for _ in 0..4 {
				if tile_two.sides[3].matches(&tile.sides[1]) {
					println!(
						"{:?} matches {:?} after {} rotation(s) and {} flip(s)",
						tile_two.sides[3].structure,
						tile.sides[1].structure,
						tile.rotations,
						tile.flipped
					);
					break 'outer;
				}
				tile_two.rotate();
			}
			tile_two.flip();
		}
		tile_two.update_content_orientation();
		tile._print();
		tile_two._print();
	}
	#[test]
	fn sample_input() {
		let input = year_2020::read_file(20);
		let mut tiles = parse_tiles(&input);
		set_uniques(&mut tiles);
		let grid = arrange_grid::<3>(tiles);
		let image = merge_content::<3, 24>(grid);
		for line in image {
			let line: String = line
				.iter()
				.map(|content| match content {
					Content::Empty => '.',
					Content::Something => '#',
					Content::Monster => 'O',
				})
				.collect();
			println!("{}", line);
		}
	}
}
