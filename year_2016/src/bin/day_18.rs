fn main() {
	shared::print_answers(18, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut row = TileRow::from_str(input);
	let mut safe_count = row.count_safe();
	for _ in 1..40 {
		row.next();
		safe_count += row.count_safe();
	}
	safe_count
}

fn get_answer_2(input: &str) -> u32 {
	let mut row = TileRow::from_str(input);
	let mut safe_count = row.count_safe();
	for _ in 1..400_000 {
		row.next();
		safe_count += row.count_safe();
	}
	safe_count
}

#[derive(Clone, Copy)]
enum Tile {
	Safe,
	Trapped,
}

struct TileRow {
	row: Vec<Tile>,
	buffer: Vec<Tile>,
}

impl TileRow {
	fn from_str(str: &str) -> Self {
		let row: Vec<_> = [Tile::Safe]
			.into_iter()
			.chain(str.trim_end().chars().map(|char| {
				if char == '.' {
					Tile::Safe
				} else {
					Tile::Trapped
				}
			}))
			.chain([Tile::Safe])
			.collect();
		let mut buffer = Vec::new();
		buffer.resize(row.len(), Tile::Safe);
		Self { row, buffer }
	}
	fn count_safe(&self) -> u32 {
		self.row
			.iter()
			.skip(1)
			.take(self.row.len() - 2)
			.filter(|tile| matches!(tile, Tile::Safe))
			.count() as u32
	}
	fn next(&mut self) {
		let len = self.buffer.len();
		for (index, tile) in self.buffer.iter_mut().enumerate().skip(1).take(len - 2) {
			if matches!(self.row[index - 1], Tile::Trapped)
				^ matches!(self.row[index + 1], Tile::Trapped)
			{
				*tile = Tile::Trapped;
			} else {
				*tile = Tile::Safe;
			}
		}
		std::mem::swap(&mut self.row, &mut self.buffer);
	}
}
