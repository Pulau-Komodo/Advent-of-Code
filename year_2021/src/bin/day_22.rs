use std::ops::{Range, Sub};

fn main() {
	shared::print_answers(22, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let instructions = input.lines().take(20).map(Instruction::from_str);
	let mut all_on: Vec<Cuboid> = Vec::new();
	for Instruction { on, cuboid } in instructions {
		all_on = all_on
			.into_iter()
			.flat_map(|existing| existing - cuboid.clone())
			.collect();
		if on {
			all_on.push(cuboid);
		}
	}
	all_on.into_iter().map(|cuboid| cuboid.volume()).sum()
}

fn get_answer_2(input: &str) -> u64 {
	let instructions = input.lines().map(Instruction::from_str);
	let mut all_on: Vec<Cuboid> = Vec::new();
	for Instruction { on, cuboid } in instructions {
		all_on = all_on
			.into_iter()
			.flat_map(|existing| existing - cuboid.clone())
			.collect();
		if on {
			all_on.push(cuboid);
		}
	}
	all_on.into_iter().map(|cuboid| cuboid.volume()).sum()
}

trait Parse {
	fn from_str(str: &str) -> Self;
}

impl Parse for Range<i32> {
	fn from_str(str: &str) -> Self {
		let (start, end) = str[2..].split_once("..").unwrap();
		let start = start.parse().unwrap();
		let end: i32 = end.parse().unwrap();
		start..end + 1
	}
}

struct Instruction {
	on: bool,
	cuboid: Cuboid,
}

impl Instruction {
	fn from_str(str: &str) -> Self {
		let (state, ranges) = str.split_once(' ').unwrap();
		let on = state == "on";
		let cuboid = Cuboid::from_str(ranges);
		Self { on, cuboid }
	}
}

trait Overlap {
	fn overlap(&self, other: &Self) -> Option<Self>
	where
		Self: Sized;
}

impl Overlap for Range<i32> {
	fn overlap(&self, other: &Self) -> Option<Self> {
		let range = self.start.max(other.start)..self.end.min(other.end);
		if range.is_empty() {
			None
		} else {
			Some(range)
		}
	}
}

#[derive(Debug, Clone)]
struct Cuboid {
	x_range: Range<i32>,
	y_range: Range<i32>,
	z_range: Range<i32>,
}

impl Cuboid {
	fn new(x_range: Range<i32>, y_range: Range<i32>, z_range: Range<i32>) -> Self {
		Self {
			x_range,
			y_range,
			z_range,
		}
	}
	fn from_str(str: &str) -> Self {
		let mut ranges = str.split(',').map(Range::<i32>::from_str);
		Self {
			x_range: ranges.next().unwrap(),
			y_range: ranges.next().unwrap(),
			z_range: ranges.next().unwrap(),
		}
	}
	fn volume(&self) -> u64 {
		[&self.x_range, &self.y_range, &self.z_range]
			.iter()
			.map(|range| range.len() as u64)
			.product()
	}
	fn overlap(&self, other: &Self) -> Option<Self> {
		let cuboid = Self {
			x_range: self.x_range.overlap(&other.x_range)?,
			y_range: self.y_range.overlap(&other.y_range)?,
			z_range: self.z_range.overlap(&other.z_range)?,
		};
		Some(cuboid)
	}
}

impl Sub for Cuboid {
	type Output = Vec<Self>;
	fn sub(self, other: Self) -> Self::Output {
		if let Some(overlap) = self.overlap(&other) {
			let mut output = Vec::with_capacity(6);
			let range_above = overlap.z_range.end..self.z_range.end;
			if !range_above.is_empty() {
				output.push(Cuboid::new(
					self.x_range.clone(),
					self.y_range.clone(),
					range_above,
				));
			}
			let range_below = self.z_range.start..overlap.z_range.start;
			if !range_below.is_empty() {
				output.push(Cuboid::new(
					self.x_range.clone(),
					self.y_range.clone(),
					range_below,
				));
			}
			let range_right = overlap.y_range.end..self.y_range.end;
			if !range_right.is_empty() {
				output.push(Cuboid::new(
					self.x_range.clone(),
					range_right,
					overlap.z_range.clone(),
				));
			}
			let range_left = self.y_range.start..overlap.y_range.start;
			if !range_left.is_empty() {
				output.push(Cuboid::new(
					self.x_range.clone(),
					range_left,
					overlap.z_range.clone(),
				));
			}
			let range_behind = overlap.x_range.end..self.x_range.end;
			if !range_behind.is_empty() {
				output.push(Cuboid::new(
					range_behind,
					overlap.y_range.clone(),
					overlap.z_range.clone(),
				));
			}
			let range_ahead = self.x_range.start..overlap.x_range.start;
			if !range_ahead.is_empty() {
				output.push(Cuboid::new(range_ahead, overlap.y_range, overlap.z_range));
			}
			output
		} else {
			vec![self]
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_overlap() {
		let range = 3..8;
		assert_eq!(Some(3..5), range.overlap(&(1..5)));
		assert_eq!(Some(6..8), range.overlap(&(6..9)));
		assert_eq!(Some(3..8), range.overlap(&(1..9)));
		assert_eq!(None, range.overlap(&(0..3)));
		assert_eq!(Some(1..2), range.overlap(&(5..6)));
	}

	#[test]
	fn test_subtract() {
		let cuboid = Cuboid::new(0..100, 0..200, 0..300);
		let hole = Cuboid::new(90..91, 50..51, 1..2);
		let subtracted = cuboid - hole;
		println!("{:?}", subtracted)
	}
}
