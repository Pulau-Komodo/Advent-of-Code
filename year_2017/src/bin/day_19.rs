use shared::{Grid, Offset, Point};

fn main() {
	shared::print_answers(19, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> String {
	let grid = Grid::new(input.lines().map(|line| line.chars().map(Cell::from_char)));

	walk_path(&grid).0
}

fn get_answer_2(input: &str) -> String {
	let grid = Grid::new(input.lines().map(|line| line.chars().map(Cell::from_char)));

	format!("{}", walk_path(&grid).1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
	Nothing,
	Path,
	Corner,
	Letter(u8),
}

impl Cell {
	fn from_char(char: char) -> Self {
		match char {
			' ' => Self::Nothing,
			'|' | '-' => Self::Path,
			'+' => Self::Corner,
			l @ 'A'..='Z' => Self::Letter(l as u8),
			_ => panic!(),
		}
	}
}

fn walk_path(grid: &Grid<Cell>) -> (String, u32) {
	let mut position = (0..grid.width())
		.find_map(|x| {
			let point = Point::new(x, 0);
			(grid.get_point(point) == Cell::Path).then_some(point)
		})
		.unwrap();
	let mut direction = Offset::new(0, 1);
	let mut letters = String::new();
	let mut step_count = 0;

	loop {
		step_count += 1;
		position += direction;
		match grid.get_point(position) {
			Cell::Corner => {
				let Some(&new_position) = position
					.orthogonal_neighbours()
					.iter()
					.filter(|point| **point != position - direction)
					.find(|point| grid.get_point(**point) == Cell::Path)
				else {
					break;
				};
				step_count += 1;
				direction = new_position - position;
				position = new_position;
			}
			Cell::Letter(l) => letters.push(char::from(l)),
			Cell::Nothing => break,
			_ => (),
		}
	}

	(letters, step_count)
}
