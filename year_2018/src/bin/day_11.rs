use std::fmt::Display;

use shared::{Grid, IntoCartesianProduct, Offset, Point};

fn main() {
	shared::print_answers(11, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> Answer {
	let serial_number = input.trim().parse().unwrap();

	let (x, y) = (0..300 - 3)
		.cartesian_product(0..300 - 3)
		.map(|(x, y)| {
			(
				(x, y),
				(0..3)
					.cartesian_product(0..3)
					.map(|(x_2, y_2)| get_power_level(x + x_2, y + y_2, serial_number))
					.sum::<i8>(),
			)
		})
		.max_by_key(|(_coords, sum)| *sum)
		.unwrap()
		.0;
	Answer::Part1(x as u16, y as u16)
}

fn get_answer_2(input: &str) -> Answer {
	let serial_number = input.trim().parse().unwrap();
	let grid =
		Grid::new((0..300).map(|y| (0..300).map(move |x| get_power_level(x, y, serial_number))));

	let (point, size) = (1..=30)
		.flat_map(|size| {
			let grid = &grid;
			(0_usize..300 - size)
				.cartesian_product(0..300 - size)
				.map(move |(x, y)| {
					let point = Point::new(x, y);
					(
						(point, size),
						(0..size)
							.cartesian_product(0..size)
							.map(|(x_2, y_2)| {
								let offset = Offset::new(x_2, y_2);
								grid.get_point(point + offset) as i16
							})
							.sum::<i16>(),
					)
				})
		})
		.max_by_key(|(_point, sum)| *sum)
		.unwrap()
		.0;
	Answer::Part2(point.x as u16, point.y as u16, size as u16)
}

fn get_power_level(x: u32, y: u32, serial_number: u32) -> i8 {
	let rack_id = x + 10;
	let power_level = (rack_id * y + serial_number) * rack_id;
	(power_level / 100 % 10) as i8 - 5
}

enum Answer {
	Part1(u16, u16),
	Part2(u16, u16, u16),
}

impl Display for Answer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Part1(x, y) => write!(f, "{x},{y}"),
			Self::Part2(x, y, size) => write!(f, "{x},{y},{size}"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example() {
		assert_eq!(get_power_level(3, 5, 8), 4);
		assert_eq!(get_power_level(122, 79, 57), -5);
		assert_eq!(get_power_level(217, 196, 39), 0);
		assert_eq!(get_power_level(101, 153, 71), 4);
	}
}
