use std::collections::{HashMap, HashSet};

use shared::Vec2;

fn main() {
	shared::print_answers(23, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut elf_map = ElfMap::from_str(input);
	for _ in 0..10 {
		elf_map.advance();
	}
	let (min, max) = get_extremes(&elf_map.elves);
	let rectangle = (max.x.abs_diff(min.x) + 1) * (max.y.abs_diff(min.y) + 1);
	rectangle - elf_map.elves.len() as u32
}

fn get_answer_2(input: &str) -> u32 {
	let mut elf_map = ElfMap::from_str(input);
	while elf_map.advance() {}
	elf_map.round + 1
}

struct ElfMap {
	elves: HashSet<Vec2<i32>>,
	considered: HashMap<Vec2<i32>, Option<Vec2<i32>>>,
	round: u32,
}

impl ElfMap {
	fn from_str(str: &str) -> Self {
		let elves: HashSet<_> = str
			.lines()
			.enumerate()
			.flat_map(|(y, line)| {
				line.chars().enumerate().filter_map(move |(x, char)| {
					(char == '#').then_some(Vec2::new(x as i32, y as i32))
				})
			})
			.collect();
		let elf_count = elves.len();
		Self {
			elves,
			round: 0,
			considered: HashMap::with_capacity(elf_count),
		}
	}
	fn advance(&mut self) -> bool {
		for &elf in &self.elves {
			let mut occupied: [Option<bool>; 8] = [None; 8];
			let mut considering = None;
			for adjacent in Adjacent::new(self.round).take(4) {
				if adjacent.iter().all(|&index| {
					!*occupied[index]
						.get_or_insert_with(|| self.elves.contains(&(elf + OFFSETS[index])))
				}) {
					considering = Some(elf + OFFSETS[adjacent[1]]);
					break;
				}
			}
			if let Some(considering) = considering {
				if occupied
					.iter()
					.any(|occupied| matches!(occupied, Some(true)))
					|| occupied.iter().enumerate().any(|(index, occupied)| {
						occupied.is_none() && self.elves.contains(&(elf + OFFSETS[index]))
					}) {
					self.considered
						.entry(considering)
						.and_modify(|proposing_elf| *proposing_elf = None)
						.or_insert(Some(elf));
				}
			}
		}
		let mut any_moved = false;
		for (destination, elf) in self
			.considered
			.drain()
			.filter_map(|(dest, elf)| elf.map(|elf| (dest, elf)))
		{
			self.elves.remove(&elf);
			self.elves.insert(destination);
			any_moved = true;
		}
		self.round += 1;
		any_moved
	}
	fn _print(&self) {
		let (min, max) = get_extremes(&self.elves);
		print!(" ");
		for x in min.x..=max.x {
			print!("{}", x.rem_euclid(10))
		}
		println!();
		for y in min.y..=max.y {
			print!("{}", y.rem_euclid(10));
			for x in min.x..=max.x {
				if self.elves.contains(&Vec2::new(x, y)) {
					print!("█");
				} else {
					print!(" ");
				}
			}
			println!();
		}
	}
}

/// ```txt
/// 0 1 2
/// 7   3
/// 6 5 4
/// ```
const OFFSETS: [Vec2<i32>; 8] = [
	Vec2::new(-1, -1),
	Vec2::new(0, -1),
	Vec2::new(1, -1),
	Vec2::new(1, 0),
	Vec2::new(1, 1),
	Vec2::new(0, 1),
	Vec2::new(-1, 1),
	Vec2::new(-1, 0),
];

struct Adjacent {
	index: u8,
}

impl Adjacent {
	fn new(round: u32) -> Self {
		Self {
			index: (round % 4) as u8,
		}
	}
}

impl Iterator for Adjacent {
	type Item = [usize; 3];
	fn next(&mut self) -> Option<Self::Item> {
		let item = match self.index {
			0 => [0, 1, 2],
			1 => [4, 5, 6],
			2 => [6, 7, 0],
			3 => [2, 3, 4],
			_ => unreachable!(),
		};
		self.index += 1;
		self.index %= 4;
		Some(item)
	}
}

fn get_extremes<'l, T>(values: T) -> (Vec2<i32>, Vec2<i32>)
where
	T: IntoIterator<Item = &'l Vec2<i32>>,
{
	values.into_iter().fold(
		(Vec2::new(i32::MAX, i32::MAX), Vec2::new(i32::MIN, i32::MIN)),
		|(min, max), elf| {
			(
				Vec2::new(min.x.min(elf.x), min.y.min(elf.y)),
				Vec2::new(max.x.max(elf.x), max.y.max(elf.y)),
			)
		},
	)
}
