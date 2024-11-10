use shared::{Grid, Point, Range};

fn main() {
	shared::print_answers(17, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let lines: Vec<_> = input.lines().map(Line::from_line).collect();
	let (min_x, max_x, max_y) = lines.iter().fold(
		(usize::MAX, usize::MIN, usize::MIN),
		|(min_x, max_x, max_y), line| {
			(
				min_x.min(line.min_x()),
				max_x.max(line.max_x()),
				max_y.max(line.max_y()),
			)
		},
	);
	let width = max_x - min_x + 2;
	let mut grid = Grid::empty(width, max_y, ' ');
	for line in lines {
		for mut point in line.iter() {
			point.x -= min_x;
			*grid.get_point_mut(point) = '█';
		}
	}
	let mut string = String::new();
	grid.write(&mut string, |char| *char).unwrap();
	std::fs::write("output.txt", string).unwrap();
	0
}

fn get_answer_2(_input: &str) -> usize {
	0
}

#[derive(Debug, Clone, Copy)]
enum Line {
	Horizontal { y: usize, x: Range<usize> },
	Vertical { x: usize, y: Range<usize> },
}

impl Line {
	fn from_line(line: &str) -> Self {
		let (fixed_str, range) = line.split_once(", ").unwrap();
		let (start, end) = range.split_once("..").unwrap();
		let fixed = fixed_str[2..].parse().unwrap();
		let range = Range::new(
			start[2..].parse().unwrap(),
			end.parse::<usize>().unwrap() + 1,
		);
		if fixed_str.as_bytes()[0] == b'y' {
			Self::Horizontal { y: fixed, x: range }
		} else {
			Self::Vertical { x: fixed, y: range }
		}
	}
	fn iter(self) -> Box<dyn Iterator<Item = Point<usize>>> {
		match self {
			Self::Horizontal { y, x } => Box::new(x.into_iter().map(move |x| Point::new(x, y))),
			Self::Vertical { x, y } => Box::new(y.into_iter().map(move |y| Point::new(x, y))),
		}
	}
	fn min_x(self) -> usize {
		match self {
			Self::Horizontal { y: _, x } => x.start,
			Self::Vertical { x, y: _ } => x,
		}
	}
	fn max_x(self) -> usize {
		match self {
			Self::Horizontal { y: _, x } => x.end - 1,
			Self::Vertical { x, y: _ } => x,
		}
	}
	fn max_y(self) -> usize {
		match self {
			Self::Horizontal { y, x: _ } => y,
			Self::Vertical { x: _, y } => y.end,
		}
	}
}
