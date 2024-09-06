use std::array;

use shared::{SmallMap, Vec3};

fn main() {
	shared::print_answers(20, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	input
		.lines()
		.map(Particle::from_line)
		.map(|p| {
			[p.acceleration, p.velocity, p.position]
				.map(Vec3::abs)
				.map(Vec3::component_sum)
		})
		.enumerate()
		.min_by_key(|(_, avp)| *avp)
		.unwrap()
		.0
}

fn get_answer_2(input: &str) -> usize {
	let mut particles: Vec<_> = input.lines().map(Particle::from_line).collect();
	let mut positions = SmallMap::new();
	let mut to_remove = Vec::new();
	let mut collision_free_cycles = 0;
	for _ in 0.. {
		collision_free_cycles += 1;
		particles.iter_mut().for_each(|particle| particle.update());
		for (index, position) in particles
			.iter()
			.map(|particle| particle.position)
			.enumerate()
		{
			if let Some(i) = positions.insert(position, index) {
				collision_free_cycles = 0;
				to_remove.push(i);
				to_remove.push(index);
			}
		}
		positions.clear();
		to_remove.dedup();
		for index in to_remove.drain(..).rev() {
			particles.swap_remove(index);
		}
		if collision_free_cycles > 1000 {
			// There is no guarantee whatsoever that 1000 cycles without collisions means they're done happening, but I got away with this assumption so I'm not going to put in some mathematical confirmation nothing will ever collide again. You can prove a particle will never collide with another by proving the difference for at least one positional vector component can never shrink again. It shouldn't be too hard, but I am on to day 21.
			break;
		}
	}
	particles.len()
}

struct Particle {
	position: Vec3<i32>,
	velocity: Vec3<i32>,
	acceleration: Vec3<i32>,
}

impl Particle {
	fn from_line(line: &str) -> Self {
		let mut split = line.split(", ");
		let vectors: [Vec3<i32>; 3] = array::from_fn(|_| {
			let substring = split.next().unwrap();
			Vec3::from_comma_separated(&substring[3..substring.len() - 1])
		});
		Self {
			position: vectors[0],
			velocity: vectors[1],
			acceleration: vectors[2],
		}
	}
	fn update(&mut self) {
		self.velocity += self.acceleration;
		self.position += self.velocity;
	}
}
