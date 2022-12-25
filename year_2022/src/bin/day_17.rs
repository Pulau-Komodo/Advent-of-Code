use std::collections::HashMap;

fn main() {
	shared::print_answers(17, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let mut jets = input.bytes().map(Push::from_byte).enumerate().cycle();
	let mut shapes = Shapes::new();
	let mut chamber = Chamber::new();
	chamber.drop_shapes(2022, &mut shapes, &mut jets);
	chamber.total_height()
}

fn get_answer_2(input: &str) -> u64 {
	let mut jets = input.bytes().map(Push::from_byte).enumerate().cycle();
	let jet_count = input.as_bytes().len();
	let mut shapes = Shapes::new();
	let target = 1_000_000_000_000;
	let mut history = HashMap::new();
	let mut chamber = Chamber::new();
	let stable_cycle_size;
	let stable_cycle_height;
	let mut shapes_dropped = 0;
	loop {
		if let Some((previous_height, previous_dropped)) = history.insert(
			chamber.fingerprint(),
			(chamber.total_height(), shapes_dropped),
		) {
			stable_cycle_size = shapes_dropped - previous_dropped;
			stable_cycle_height = chamber.total_height() - previous_height;
			break;
		}
		shapes_dropped += chamber.drop_shapes_for_n_jets(jet_count, &mut shapes, &mut jets);
	}
	let target_left = target - shapes_dropped;
	let remainder = target_left % stable_cycle_size;
	let stable_cycles_left = target_left / stable_cycle_size;
	chamber.drop_shapes(remainder as usize, &mut shapes, &mut jets);

	chamber.total_height() + stable_cycles_left * stable_cycle_height
}

enum Push {
	Left,
	Right,
}

impl Push {
	fn from_byte(byte: u8) -> Self {
		match byte {
			b'<' => Self::Left,
			_ => Self::Right,
		}
	}
}

#[derive(Clone)]
struct Chamber {
	rows: Vec<u8>,
	culled_height: u64,
	jet_index: usize,
	falling_shape: Option<(Shape, usize)>,
}

impl Chamber {
	fn new() -> Self {
		Self {
			rows: Vec::new(),
			culled_height: 0,
			jet_index: 0,
			falling_shape: None,
		}
	}
	fn height(&self) -> usize {
		self.rows.len()
	}
	fn total_height(&self) -> u64 {
		self.culled_height + self.height() as u64
	}
	fn fingerprint(&self) -> (usize, [usize; 7], Option<(Shape, usize)>) {
		(self.jet_index, self.height_profile(), self.falling_shape)
	}
	fn height_profile(&self) -> [usize; 7] {
		let mut height_profile = [self.height(); 7];
		let mut found = 0;
		for (index, row) in self.rows.iter().rev().enumerate() {
			for pos in 0..7 {
				if 1 << pos & row != 0 {
					height_profile[6 - pos] = height_profile[6 - pos].min(index);
					found |= 1 << pos;
				}
			}
			if found == 0b1111111 {
				break;
			}
		}
		height_profile
	}
	fn drop_shapes<S, J>(&mut self, count: usize, shapes: &mut S, mut jets: &mut J)
	where
		S: Iterator<Item = Shape>,
		J: Iterator<Item = (usize, Push)>,
	{
		for (mut shape, mut fallen) in std::mem::take(&mut self.falling_shape)
			.into_iter()
			.chain(shapes.map(|x| (x, 0)))
			.take(count)
		{
			for (index, jet) in &mut jets {
				let new_shape = shape.with_push(jet);
				if !self.collides(new_shape, fallen) {
					shape = new_shape;
				}
				if fallen + 1 >= self.height() + 4 || self.collides(shape, fallen + 1) {
					self.jet_index = index;
					break;
				}
				fallen += 1;
			}
			self.add_shape(shape, fallen);
		}
	}
	fn drop_shapes_for_n_jets<S, J>(&mut self, count: usize, shapes: &mut S, jets: &mut J) -> u64
	where
		S: Iterator<Item = Shape>,
		J: Iterator<Item = (usize, Push)>,
	{
		let mut dropped = 0;
		for (index, jet) in jets.take(count) {
			self.jet_index = index;
			let (mut shape, fallen) = self
				.falling_shape
				.get_or_insert_with(|| (shapes.next().unwrap(), 0));
			let new_shape = shape.with_push(jet);
			let fallen = *fallen;
			if !self.collides(new_shape, fallen) {
				shape = new_shape;
			}
			if fallen + 1 >= self.height() + 4 || self.collides(shape, fallen + 1) {
				self.add_shape(shape, fallen);
				dropped += 1;
				self.falling_shape = None;
				continue;
			}
			self.falling_shape = self.falling_shape.map(|(_, _)| (shape, fallen + 1));
		}
		dropped
	}
	fn add_shape(&mut self, shape: Shape, fallen: usize) {
		if self.height() > 1_000_000 {
			self.cull();
		}
		let rock_outside_chamber = (shape.height + 3).saturating_sub(fallen);
		self.rows.reserve(rock_outside_chamber);
		for _ in 0..rock_outside_chamber {
			self.rows.push(0);
		}
		for (row, rock) in self
			.rows
			.iter_mut()
			.rev()
			.skip(fallen.saturating_sub(shape.height + 3))
			.zip(shape.rock.into_iter().take(shape.height).rev())
		{
			*row |= rock;
		}
	}
	fn collides(&self, shape: Shape, fallen: usize) -> bool {
		let rock_outside_chamber = (shape.height + 3).saturating_sub(fallen);
		self.rows
			.iter()
			.rev()
			.skip(fallen.saturating_sub(shape.height + 3))
			.zip(
				shape
					.rock
					.into_iter()
					.take(shape.height)
					.rev()
					.skip(rock_outside_chamber),
			)
			.any(|(row, shape)| shape & row != 0)
	}
	fn cull(&mut self) {
		let old_height = self.height();
		if let Some(index) = self.rows.iter().rev().position(|row| *row == 0b1111111) {
			let index = self.height() - index;
			self.rows = self.rows[index..].to_vec();
		}
		self.culled_height += (old_height - self.height()) as u64;
	}
	fn _print(&self) {
		for bitmask in self.rows.iter().rev() {
			print!("║");
			for n in (0..7).rev() {
				if 1 << n & bitmask == 0 {
					print!(" ");
				} else {
					print!("█");
				}
			}
			println!("║");
		}
	}
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Shape {
	rock: [u8; 4],
	height: usize,
}

impl Shape {
	fn from_str(str: &str) -> Self {
		let mut rock = [0; 4];
		let mut height = 0;
		let mut width = 0;
		for (index, bitmask) in str
			.lines()
			.rev()
			.map(|line| {
				width = line.as_bytes().len();
				let mut bitmask = 0;
				for byte in line.bytes() {
					bitmask <<= 1;
					if byte == b'#' {
						bitmask |= 1;
					}
				}
				bitmask
			})
			.enumerate()
		{
			rock[index] = bitmask;
			height += 1;
		}
		for bitmask in &mut rock {
			*bitmask <<= 5 - width;
		}
		Self { rock, height }
	}
	fn with_push(mut self, push: Push) -> Self {
		let original = self;
		for bitmask in self.rock.iter_mut().take(self.height) {
			match push {
				Push::Left => {
					if 1 << 6 & *bitmask != 0 {
						return original;
					} else {
						*bitmask <<= 1;
					}
				}
				Push::Right => {
					if 1 << 0 & *bitmask != 0 {
						return original;
					} else {
						*bitmask >>= 1;
					}
				}
			}
		}
		self
	}
	fn _print(&self) {
		for bitmask in self.rock.iter().rev() {
			print!("┃");
			for n in (0..7).rev() {
				if 1 << n & bitmask == 0 {
					print!(" ");
				} else {
					print!("█");
				}
			}
			println!("┃");
		}
	}
}

struct Shapes {
	shapes: Vec<Shape>,
	current_shape: usize,
}

impl Shapes {
	fn new() -> Self {
		let shapes = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##"
		.split("\n\n")
		.map(Shape::from_str)
		.collect();
		Self {
			shapes,
			current_shape: 0,
		}
	}
}

impl Iterator for Shapes {
	type Item = Shape;
	fn next(&mut self) -> Option<Self::Item> {
		let shape = self.current_shape;
		self.current_shape += 1;
		self.current_shape %= self.shapes.len();
		Some(self.shapes[shape])
	}
}
