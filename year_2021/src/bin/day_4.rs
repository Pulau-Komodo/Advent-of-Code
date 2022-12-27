fn main() {
	shared::print_answers(4, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let (draws, mut boards) = parse_input(input);
	for draw in draws {
		for board in boards.iter_mut() {
			let was_marked = board.mark_number(draw);
			if was_marked && board.has_bingo() {
				let score = board.score();
				return score as u32 * draw as u32;
			}
		}
	}
	panic!();
}

fn get_answer_2(input: &str) -> u32 {
	let (draws, boards) = parse_input(input);
	let mut boards: Vec<Option<BingoBoard>> = boards.into_iter().map(Some).collect();
	let mut finished_boards = 0;
	let board_count = boards.len();
	for draw in draws {
		for wrapped_board in boards.iter_mut().filter(|board| board.is_some()) {
			let board = wrapped_board.as_mut().unwrap();
			let was_marked = board.mark_number(draw);
			if was_marked && board.has_bingo() {
				finished_boards += 1;
				if finished_boards == board_count {
					let score = board.score();
					return score as u32 * draw as u32;
				}
				*wrapped_board = None;
			}
		}
	}
	panic!();
}

struct BingoBoard {
	rows: [[Option<u8>; 5]; 5],
	columns: [[Option<u8>; 5]; 5],
}

impl BingoBoard {
	fn from_str(str: &str) -> Self {
		let mut rows = [[None; 5]; 5];
		let mut columns = [[None; 5]; 5];
		str.lines().enumerate().for_each(|(y, line)| {
			for (x, number) in line.split_whitespace().enumerate() {
				let number: u8 = number.parse().unwrap();
				rows[y][x] = Some(number);
				columns[x][y] = Some(number);
			}
		});
		Self { rows, columns }
	}
	/// Mark a number and return whether the board had that number
	fn mark_number(&mut self, draw: u8) -> bool {
		let mut had_number = false;
		for number in self.rows.iter_mut().flat_map(|row| row.iter_mut()) {
			if *number == Some(draw) {
				*number = None;
				had_number = true;
				break;
			}
		}
		if had_number {
			for number in self.columns.iter_mut().flat_map(|column| column.iter_mut()) {
				if *number == Some(draw) {
					*number = None;
					break;
				}
			}
		}
		had_number
	}
	fn has_bingo(&self) -> bool {
		self.rows
			.iter()
			.chain(self.columns.iter())
			.any(|&set| set.iter().all(|&number| number.is_none()))
	}
	fn score(&self) -> u16 {
		self.rows
			.iter()
			.flat_map(|row| row.iter())
			.filter_map(|&number| number.map(|value| value as u16))
			.sum()
	}
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<BingoBoard>) {
	let mut sections = input.split("\r\n\r\n");
	let draws = sections
		.next()
		.unwrap()
		.split(',')
		.map(str::parse)
		.collect::<Result<Vec<_>, _>>()
		.unwrap();
	let boards = sections.map(BingoBoard::from_str).collect();
	(draws, boards)
}
