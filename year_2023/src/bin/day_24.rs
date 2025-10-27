use shared::{Vec2, Vec3};

fn main() {
	shared::print_answers(24, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> i64 {
	let window = 200000000000000.0..=400000000000000.0;
	// let window = 7.0..=27.0;
	let hailstones = input.lines().map(Hailstone::from_line).collect::<Vec<_>>();
	let mut intersection_count = 0;
	for a in 0..hailstones.len() - 1 {
		for b in a + 1..hailstones.len() {
			let Some(intersection) = hailstones[a].intersect_2d(&hailstones[b]) else {
				continue;
			};
			//println!("{:?}", intersection);
			if window.contains(&intersection.x) && window.contains(&intersection.y) {
				intersection_count += 1;
			}
		}
	}
	intersection_count
}

fn get_answer_2(input: &str) -> i64 {
	let hailstones = input
		.lines()
		.map(HailstoneInt::from_line)
		.collect::<Vec<_>>();
	'outer: for line in hailstones[0].colliding_hailstones(hailstones[1]) {
		for hailstone in &hailstones[2..] {
			if line.collide(*hailstone).is_some() {
				println!("Found collision.");
			} else {
				continue 'outer;
			}
		}
		println!("{:?}", line);
		return line.position.x + line.position.y + line.position.x;
	}
	panic!("Found no solution");
}

#[derive(Debug, PartialEq)]
struct Hailstone {
	position: Vec3<f64>,
	velocity: Vec3<f64>,
}

impl Hailstone {
	fn from_line(line: &str) -> Self {
		let (position, velocity) = line.split_once(" @ ").unwrap();
		let mut position = position.split(", ").map(|n| n.trim().parse().unwrap());
		let position = Vec3 {
			x: position.next().unwrap(),
			y: position.next().unwrap(),
			z: position.next().unwrap(),
		};
		let mut velocity = velocity
			.split(", ")
			.map(|n| n.trim().parse::<f64>().unwrap());
		let velocity = Vec3 {
			x: velocity.next().unwrap() * 1000.0,
			y: velocity.next().unwrap() * 1000.0,
			z: velocity.next().unwrap() * 1000.0,
		};
		Self { position, velocity }
	}
	fn intersect_2d(&self, other: &Self) -> Option<Vec2<f64>> {
		let x_1 = self.position.x;
		let y_1 = self.position.y;
		let self_offset = self.position + self.velocity;
		let x_2 = self_offset.x;
		let y_2 = self_offset.y;
		let x_3 = other.position.x;
		let y_3 = other.position.y;
		let other_offset = other.position + other.velocity;
		let x_4 = other_offset.x;
		let y_4 = other_offset.y;

		let divisor = (x_1 - x_2) * (y_3 - y_4) - (y_1 - y_2) * (x_3 - x_4);
		if divisor == 0.0 {
			// Possibly parallel, and parallel lines can still overlap
			let difference = self.position.truncate() - other.position.truncate();
			let a = difference / self.velocity.truncate();
			let b = difference / other.velocity.truncate();
			if (a.x - 1.1..a.x + 1.1).contains(&a.y) || (b.x - 1.1..b.x + 1.1).contains(&b.y) {
				println!("Found overlapping lines");
				return Some(Vec2::new(0.0, 0.0));
			}
			return None;
		}
		let x = ((x_1 * y_2 - y_1 * x_2) * (x_3 - x_4) - (x_1 - x_2) * (x_3 * y_4 - y_3 * x_4))
			/ divisor;
		let y = ((x_1 * y_2 - y_1 * x_2) * (y_3 - y_4) - (y_1 - y_2) * (x_3 * y_4 - y_3 * x_4))
			/ divisor;

		let intersection = Vec2 { x, y };
		let self_offset_intersection = intersection - self.position.truncate();
		let other_offset_intersection = intersection - other.position.truncate();

		if self.velocity.x.signum() != self_offset_intersection.x.signum()
			|| self.velocity.y.signum() != self_offset_intersection.y.signum()
			|| other.velocity.x.signum() != other_offset_intersection.x.signum()
			|| other.velocity.y.signum() != other_offset_intersection.y.signum()
		{
			return None;
		}
		Some(intersection)
	}
}

#[derive(Debug, Clone, Copy)]
struct HailstoneInt {
	position: Vec3<i64>,
	velocity: Vec3<i64>,
}

impl HailstoneInt {
	fn from_line(line: &str) -> Self {
		let (position, velocity) = line.split_once(" @ ").unwrap();
		let [position, velocity] = [position, velocity].map(|vector_text| {
			let mut iter = vector_text.split(", ").map(|n| n.trim().parse().unwrap());
			Vec3 {
				x: iter.next().unwrap(),
				y: iter.next().unwrap(),
				z: iter.next().unwrap(),
			}
		});
		Self { position, velocity }
	}
	fn collide(&self, other: Self) -> Option<Vec3<i64>> {
		let mut position = self.position;
		let mut other_position = other.position;
		let velocity_offset = self.velocity - other.velocity;
		loop {
			let offset = other_position - position;
			if offset.signum() != velocity_offset.signum() {
				return None;
			}
			position += self.velocity;
			other_position += other.velocity;
			if position == other_position {
				return Some(position);
			}
		}
	}
	fn colliding_hailstones(self, other: Self) -> impl Iterator<Item = HailstoneInt> {
		(0..)
			.inspect(|a| {
				if a % 1_000_000 == 0 {
					println!("a: {a}");
				}
			})
			.flat_map(|a| (0..=a).map(move |b| (a, b)))
			.inspect(|(a, b)| {
				if a == b {
					println!("a and b: {a}");
				}
			})
			.map(move |(a, b)| {
				let pos_a = self.position + self.velocity * a;
				let pos_b = other.position + other.velocity * b;
				let velocity = pos_b - pos_a;
				let position = pos_a - velocity;
				HailstoneInt { position, velocity }
			})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn collision() {
		let a = HailstoneInt {
			position: Vec3::new(-30, -25, 20),
			velocity: Vec3::new(6, 5, -4),
		};
		let b = HailstoneInt {
			position: Vec3::new(10, -10, 5),
			velocity: Vec3::new(-2, 2, -1),
		};
		assert_eq!(a.collide(b), Some(Vec3::new(0, 0, 0)));
		let c = HailstoneInt {
			position: Vec3::new(10, -10, 5),
			velocity: Vec3::new(-2, 2, -2),
		};
		assert_eq!(a.collide(c), None);
	}
}
