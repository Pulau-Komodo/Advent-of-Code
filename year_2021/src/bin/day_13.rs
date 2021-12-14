fn main() {
	shared::print_answers(13, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let (points, mut folds) = parse_input(input);
	let fold = folds.next().unwrap();
	points
		.map(|mut point| {
			point.fold(&fold);
			point
		})
		.collect::<std::collections::HashSet<_>>()
		.len() as u32
}

fn get_answer_2(input: &str) -> u32 {
	let (points, folds) = parse_input(input);
	let folds: Vec<_> = folds.collect();
	let mut highest_x = 0;
	let mut highest_y = 0;
	let map = points
		.map(|mut point| {
			folds.iter().for_each(|fold| point.fold(fold));
			highest_x = highest_x.max(point.x);
			highest_y = highest_y.max(point.y);
			point
		})
		.collect::<std::collections::HashSet<_>>();
	for y in 0..=highest_y {
		let line: String = (0..=highest_x)
			.map(|x| {
				if map.contains(&Point { x, y }) {
					'█'
				} else {
					' '
				}
			})
			.collect();
		println!("{}", line);
	}
	map.len() as u32
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
	x: u16,
	y: u16,
}

impl Point {
	fn from_str(str: &str) -> Self {
		let (x, y) = str.split_once(',').unwrap();
		Self {
			x: x.parse().unwrap(),
			y: y.parse().unwrap(),
		}
	}
	fn fold(&mut self, fold: &Fold) {
		match *fold {
			Fold::Horizontal(fold_y) => {
				if self.y > fold_y {
					self.y = fold_y * 2 - self.y;
				}
			}
			Fold::Vertical(fold_x) => {
				if self.x > fold_x {
					self.x = fold_x * 2 - self.x;
				}
			}
		}
	}
}

enum Fold {
	Horizontal(u16),
	Vertical(u16),
}

impl Fold {
	fn from_str(str: &str) -> Self {
		let (axis, placement) = str.split_once('=').unwrap();
		let placement = placement.parse().unwrap();
		match axis {
			"fold along y" => Self::Horizontal(placement),
			_ => Self::Vertical(placement),
		}
	}
}

fn parse_input(
	input: &str,
) -> (
	impl Iterator<Item = Point> + '_,
	impl Iterator<Item = Fold> + '_,
) {
	let (points, folds) = input.split_once("\r\n\r\n").unwrap();
	let points = points.lines().map(Point::from_str);
	let folds = folds.lines().map(Fold::from_str);
	(points, folds)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_input() {
		let input = "6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\r\n\r\nfold along y=7\nfold along x=5";
		assert_eq!(get_answer_2(input), 16);
	}
}
