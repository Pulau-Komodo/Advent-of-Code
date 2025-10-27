fn main() {
	shared::print_answers(18, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let mut grid = Grid::from_str(input);
	for _ in 0..100 {
		grid.step();
	}
	grid.on.len()
}

fn get_answer_2(input: &str) -> usize {
	let mut grid = Grid::from_str(input);
	grid.set_corners_always_on();
	for _ in 0..100 {
		grid.step();
	}
	grid.on.len()
}

#[derive(Clone, Copy, Debug)]
enum Offset {
	Up,
	None,
	Down,
}

const ADJACENT: [(Offset, Offset); 8] = {
	use Offset::*;
	[
		(Up, Up),
		(Up, None),
		(Up, Down),
		(None, Down),
		(Down, Down),
		(Down, None),
		(Down, Up),
		(None, Up),
	]
};

struct Grid {
	on: std::collections::HashSet<(u8, u8)>,
	corners_on: bool,
}

impl Grid {
	fn from_str(str: &str) -> Self {
		let on = str
			.lines()
			.enumerate()
			.flat_map(|(y, line)| line.char_indices().map(move |(x, char)| (x, y, char)))
			.filter_map(|(x, y, char)| (char == '#').then_some((x as u8, y as u8)))
			.collect();
		Self {
			on,
			corners_on: false,
		}
	}
	fn step(&mut self) {
		let to_check: std::collections::HashSet<(u8, u8)> = self
			.on
			.iter()
			.flat_map(|&coords| {
				ADJACENT
					.iter()
					.filter_map(move |&offset| get_nearby_coords(coords, offset))
					.chain(std::iter::once(coords))
			})
			.collect();
		self.on = to_check
			.into_iter()
			.filter(|&coords| self.should_be_on(coords))
			.collect();
		if self.corners_on {
			self.turn_corners_on();
		}
	}
	fn should_be_on(&self, coords: (u8, u8)) -> bool {
		let is_on = self.on.contains(&coords);
		let mut nearby_on = 0;
		let mut nearby_off = 0;
		for offset in ADJACENT {
			if nearby_on > 3 || nearby_off > 6 || !is_on && nearby_off > 5 {
				return false;
			}
			if let Some(true) = get_nearby_coords(coords, offset)
				.map(|nearby_coords| self.on.contains(&nearby_coords))
			{
				nearby_on += 1;
			} else {
				nearby_off += 1;
			}
		}
		nearby_on == 3 || is_on && nearby_on == 2
	}
	fn set_corners_always_on(&mut self) {
		self.corners_on = true;
		self.turn_corners_on();
	}
	fn turn_corners_on(&mut self) {
		self.on.insert((0, 0));
		self.on.insert((99, 0));
		self.on.insert((99, 99));
		self.on.insert((0, 99));
	}
}

fn get_nearby_coords((x, y): (u8, u8), (offset_x, offset_y): (Offset, Offset)) -> Option<(u8, u8)> {
	let x = match (offset_x, x) {
		(Offset::Down, n @ 1..) => n - 1,
		(Offset::None, n) => n,
		(Offset::Up, n @ 0..=98) => n + 1,
		_ => return None,
	};
	let y = match (offset_y, y) {
		(Offset::Down, n @ 1..) => n - 1,
		(Offset::None, n) => n,
		(Offset::Up, n @ 0..=98) => n + 1,
		_ => return None,
	};
	Some((x, y))
}

#[cfg(test)]
mod tests {
	use super::*;

	fn print(set: &std::collections::HashSet<(u8, u8)>) {
		for y in 0..100 {
			let line: String = (0..100)
				.map(|x| if set.contains(&(x, y)) { '#' } else { '.' })
				.collect();
			println!("{}", line);
		}
	}

	#[test]
	fn last_two_display() {
		let input = shared::read_file(18);
		let mut grid = Grid::from_str(&input);
		for _ in 0..99 {
			grid.step();
		}
		print(&grid.on);
		grid.step();
		println!("---");
		print(&grid.on);
	}
}
