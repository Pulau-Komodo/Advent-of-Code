use std::{
	collections::HashSet,
	ops::{Add, Sub},
};

fn main() {
	shared::print_answers(19, &[get_answers]);
}

fn get_answers(input: &str) -> String {
	let mut scanners = input.split("\r\n\r\n").map(Scanner::from_str);
	let first = scanners.next().unwrap();
	let mut scanners: Vec<_> = scanners.map(Some).collect();
	let mut placed_scanners = Vec::with_capacity(scanners.len());
	let mut recently_placed_scanners = Vec::with_capacity(scanners.len() / 4);
	recently_placed_scanners.push((Offset { x: 0, y: 0, z: 0 }, first));
	loop {
		let mut newly_found_scanners = Vec::with_capacity(scanners.len() / 4);
		for (placed_offset, placed_scanner) in recently_placed_scanners.iter() {
			for (index, scanner) in scanners.iter().enumerate().filter_map(|(index, scanner)| {
				if let Some(scanner) = scanner {
					Some((index, scanner))
				} else {
					None
				}
			}) {
				if !newly_found_scanners.iter().any(|(_, _, i)| *i == index) {
					if let Some((orientation, offset)) = placed_scanner.link(scanner) {
						newly_found_scanners.push((orientation, *placed_offset + offset, index));
					}
				}
			}
		}
		placed_scanners.append(&mut recently_placed_scanners);
		if newly_found_scanners.is_empty() {
			break;
		}
		for (orientation, offset, index) in newly_found_scanners {
			let mut found = None;
			std::mem::swap(&mut scanners[index], &mut found);
			println!("Placed {}", index);
			let mut found = found.unwrap();
			found.apply_orientation(orientation);
			recently_placed_scanners.push((offset, found));
		}
	}
	for scanner in placed_scanners.iter() {
		println!("{:?}", scanner.0)
	}
	let mut largest_distance = 0;
	for first in 0..placed_scanners.len() {
		for second in first + 1..placed_scanners.len() {
			let first = placed_scanners[first].0;
			let second = placed_scanners[second].0;
			let distance = Point::from(first) - Point::from(second);
			largest_distance = largest_distance.max(distance.manhattan_distance());
		}
	}
	let points: HashSet<_> = placed_scanners
		.into_iter()
		.flat_map(|(offset, scanner)| {
			scanner
				.beacons()
				.map(|point| point + offset)
				.collect::<Vec<_>>()
		})
		.collect();
	format!("1: {}, 2: {}", points.len(), largest_distance)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
	x: i16,
	y: i16,
	z: i16,
}

impl Point {
	fn new(x: i16, y: i16, z: i16) -> Self {
		Self { x, y, z }
	}
	fn from_str(str: &str) -> Self {
		let mut components = str.split(',').map(str::parse);
		let x = components.next().unwrap().unwrap();
		let y = components.next().unwrap().unwrap();
		let z = components.next().unwrap().unwrap();
		Self { x, y, z }
	}
	fn in_orientation(&self, orientation: u8) -> Self {
		let &Self { x, y, z } = self;
		match orientation {
			0 => Self::new(x, y, z),
			1 => Self::new(x, z, -y),
			2 => Self::new(x, -y, -z),
			3 => Self::new(x, -z, y),

			4 => Self::new(y, -x, z),
			5 => Self::new(y, z, x),
			6 => Self::new(y, x, -z),
			7 => Self::new(y, -z, -x),

			8 => Self::new(-x, -y, z),
			9 => Self::new(-x, -z, -y),
			10 => Self::new(-x, y, -z),
			11 => Self::new(-x, z, y),

			12 => Self::new(-y, x, z),
			13 => Self::new(-y, -z, x),
			14 => Self::new(-y, -x, -z),
			15 => Self::new(-y, z, -x),

			16 => Self::new(z, y, -x),
			17 => Self::new(z, x, y),
			18 => Self::new(z, -y, x),
			19 => Self::new(z, -x, -y),

			20 => Self::new(-z, -y, -x),
			21 => Self::new(-z, -x, y),
			22 => Self::new(-z, y, x),
			23.. => Self::new(-z, x, -y),
		}
	}
}

impl Sub for Point {
	type Output = Offset;
	fn sub(self, rhs: Self) -> Self::Output {
		Self::Output {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
		}
	}
}

impl Sub<Offset> for Point {
	type Output = Self;
	fn sub(self, rhs: Offset) -> Self::Output {
		Self::Output {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
		}
	}
}

impl Add<Offset> for Point {
	type Output = Self;
	fn add(self, rhs: Offset) -> Self::Output {
		Self::Output {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}

impl From<Offset> for Point {
	fn from(offset: Offset) -> Self {
		Self {
			x: offset.x,
			y: offset.y,
			z: offset.z,
		}
	}
}

#[derive(Debug, Clone, Copy)]
struct Offset {
	x: i16,
	y: i16,
	z: i16,
}

impl Offset {
	fn manhattan_distance(&self) -> i16 {
		self.x.abs() + self.y.abs() + self.z.abs()
	}
}

impl Add for Offset {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self::Output {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}

struct Scanner {
	beacons: Vec<Point>,
}

impl Scanner {
	fn from_str(str: &str) -> Self {
		let beacons = str.lines().skip(1).map(Point::from_str).collect();
		Self { beacons }
	}
	fn beacons_at_orientation(&self, orientation: u8) -> impl Iterator<Item = Point> + '_ {
		self.beacons
			.iter()
			.map(move |point| point.in_orientation(orientation))
	}
	fn beacons(&self) -> impl Iterator<Item = Point> + '_ {
		self.beacons.iter().copied()
	}
	fn link(&self, other: &Self) -> Option<(u8, Offset)> {
		for orientation in 0..24 {
			if let Some(offset) = self.link_at_orientation(other, orientation) {
				return Some((orientation, offset));
			}
		}
		None
	}
	fn link_at_orientation(&self, other: &Self, orientation: u8) -> Option<Offset> {
		for beacon in self.beacons() {
			for other_beacon in other.beacons_at_orientation(orientation).skip(11) {
				let offset = beacon - other_beacon;
				if self.link_at_orientation_and_offset(other, orientation, offset) {
					return Some(offset);
				}
			}
		}
		None
	}
	fn link_at_orientation_and_offset(
		&self,
		other: &Self,
		orientation: u8,
		offset: Offset,
	) -> bool {
		other
			.beacons_at_orientation(orientation)
			.filter(|&other_beacon| {
				self.beacons()
					.any(|beacon| beacon == (other_beacon + offset))
			})
			.count() >= 12 // Just assuming 12 will never match without being valid
	}
	fn apply_orientation(&mut self, orientation: u8) {
		self.beacons
			.iter_mut()
			.for_each(|point| *point = point.in_orientation(orientation));
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rotation() {
		let point = Point { x: 1, y: 2, z: 3 };
		for orientation in 0..24 {
			println!("{:?}", point.in_orientation(orientation));
		}
	}

	#[test]
	fn test_link() {
		let input = shared::read_file_special(19, "sample");
		let scanners: Vec<_> = input.split("\r\n\r\n").map(Scanner::from_str).collect();
		if let Some((orientation, offset)) = scanners[0].link(&scanners[1]) {
			println!("Found! At orientation {} and {:?}", orientation, offset);
			return;
		}
		panic!();
	}

	#[test]
	fn sample_input() {
		let input = shared::read_file_special(19, "sample");
		assert_eq!("1: 79, 2: 3621", get_answers(&input));
	}
}
