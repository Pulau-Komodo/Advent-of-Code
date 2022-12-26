use std::collections::HashSet;

use shared::{wrapping_add, wrapping_sub, Vec2};

fn main() {
	shared::print_answers(24, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let blizzards = Blizzards::from_str(input);
	let start = Vec2::new(1, 0);
	let goal = Vec2::new(blizzards.width() as u8, blizzards.height() as u8 + 1);
	let mut frontier = Vec::from([State::new(start, &blizzards)]);
	let mut new_frontier = Vec::new();
	let mut positions = HashSet::new();
	for step in 1.. {
		for state in frontier.drain(..) {
			for new_state in state.possible_new_states(step) {
				if new_state.position == goal {
					return step;
				}
				if !positions.contains(&new_state.position) {
					positions.insert(new_state.position);
					new_frontier.push(new_state);
				}
			}
		}
		if new_frontier.is_empty() {
			panic!("Could not find a path.")
		}
		positions.clear();
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
	unreachable!();
}

fn get_answer_2(input: &str) -> u32 {
	let blizzards = Blizzards::from_str(input);
	let start = Vec2::new(1, 0);
	let end = Vec2::new(blizzards.width() as u8, blizzards.height() as u8 + 1);
	let mut goal = end;
	let mut end_visited = false;
	let mut frontier = Vec::from([State::new(start, &blizzards)]);
	let mut new_frontier = Vec::new();
	let mut positions = HashSet::new();
	for step in 1.. {
		let mut finished = false;
		for state in frontier.drain(..) {
			for new_state in state.possible_new_states(step) {
				if new_state.position == goal {
					if end_visited && goal == end {
						return step;
					}
					finished = true;
				}
				if !positions.contains(&new_state.position) {
					positions.insert(new_state.position);
					new_frontier.push(new_state);
				}
			}
		}
		if finished {
			frontier.clear();
			new_frontier.clear();
			if goal == end {
				goal = start;
				end_visited = true;
				new_frontier.push(State::new(end, &blizzards));
			} else {
				goal = end;
				new_frontier.push(State::new(start, &blizzards));
			}
		} else if new_frontier.is_empty() {
			panic!("Could not find a path.")
		}
		positions.clear();
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
	unreachable!();
}

#[derive(Debug, Default)]
struct BlizzardLine {
	increasing: HashSet<u8>,
	decreasing: HashSet<u8>,
}

#[derive(Debug)]
struct Blizzards {
	horizontal: Vec<BlizzardLine>,
	vertical: Vec<BlizzardLine>,
}

impl Blizzards {
	fn from_str(str: &str) -> Self {
		let mut horizontal = Vec::with_capacity(str.lines().count());
		let mut vertical = Vec::with_capacity(str.lines().next().unwrap().chars().count());
		horizontal.push(BlizzardLine::default());
		vertical.push(BlizzardLine::default());
		for (x, y, char) in str
			.lines()
			.enumerate()
			.flat_map(|(y, line)| line.char_indices().map(move |(x, char)| (x, y, char)))
		{
			if x + 1 > vertical.len() {
				vertical.push(BlizzardLine::default());
			}
			if y + 1 > horizontal.len() {
				horizontal.push(BlizzardLine::default());
			}
			match char {
				'<' => horizontal[y].decreasing.insert(x as u8),
				'>' => horizontal[y].increasing.insert(x as u8),
				'^' => vertical[x].decreasing.insert(y as u8),
				'v' => vertical[x].increasing.insert(y as u8),
				_ => false,
			};
		}
		Self {
			horizontal,
			vertical,
		}
	}
	fn height(&self) -> u32 {
		self.horizontal.len() as u32 - 2
	}
	fn width(&self) -> u32 {
		self.vertical.len() as u32 - 2
	}
	fn _print(&self, step: u32) {
		for y in 0..self.horizontal.len() {
			for x in 0..self.vertical.len() {
				if x == 0 || x == self.vertical.len() - 1 {
					print!("#");
				} else if y == 0 {
					if x == 1 {
						print!(".");
					} else {
						print!("#");
					}
				} else if y == self.horizontal.len() - 1 {
					if x == self.vertical.len() - 2 {
						print!(".");
					} else {
						print!("#");
					}
				} else {
					let horizontal_step = step % self.width();
					let increasing_x =
						wrapping_sub(x as u32, horizontal_step, 1..self.width() + 1) as u8;
					let decreasing_x =
						wrapping_add(x as u32, horizontal_step, 1..self.width() + 1) as u8;
					let horizontal_blizzards = &self.horizontal[y];
					let vertical_step = step % self.height();
					let increasing_y =
						wrapping_sub(y as u32, vertical_step, 1..self.height() + 1) as u8;
					let decreasing_y =
						wrapping_add(y as u32, vertical_step, 1..self.height() + 1) as u8;
					let vertical_blizzards = &self.vertical[x];

					let mut overlap = 0;
					let mut char = '.';
					if horizontal_blizzards.increasing.contains(&increasing_x) {
						overlap += 1;
						char = '>';
					};
					if horizontal_blizzards.decreasing.contains(&decreasing_x) {
						overlap += 1;
						char = '<';
					};
					if vertical_blizzards.increasing.contains(&increasing_y) {
						overlap += 1;
						char = 'v';
					};
					if vertical_blizzards.decreasing.contains(&decreasing_y) {
						overlap += 1;
						char = '^';
					};
					if overlap > 1 {
						print!("{}", overlap);
					} else {
						print!("{}", char);
					}
				}
			}
			println!();
		}
	}
}

#[derive(Clone, Copy)]
struct State<'l> {
	position: Vec2<u8>,
	blizzards: &'l Blizzards,
}

impl<'l> State<'l> {
	fn new(position: Vec2<u8>, blizzards: &'l Blizzards) -> Self {
		Self {
			position,
			blizzards,
		}
	}
	fn try_move_right(mut self) -> Option<Self> {
		(self.position.x < self.blizzards.width() as u8 && self.position.y > 0).then(|| {
			self.position.x += 1;
			self
		})
	}
	fn try_move_down(mut self) -> Option<Self> {
		(self.position.y < self.blizzards.height() as u8
			|| self.position.y == self.blizzards.height() as u8
				&& self.position.x == self.blizzards.width() as u8)
			.then(|| {
				self.position.y += 1;
				self
			})
	}
	fn try_move_left(mut self) -> Option<Self> {
		(self.position.x > 1).then(|| {
			self.position.x -= 1;
			self
		})
	}
	fn try_move_up(mut self) -> Option<Self> {
		(self.position.y > 1 || self.position.x == 1 && self.position.y > 0).then(|| {
			self.position.y -= 1;
			self
		})
	}
	fn possible_new_states(self, step: u32) -> impl Iterator<Item = State<'l>> {
		self.try_move_right()
			.into_iter()
			.chain(self.try_move_down())
			.chain(self.try_move_left())
			.chain(self.try_move_up())
			.chain([self])
			.filter(move |state| !state.in_blizzard(step))
	}
	fn in_blizzard(&self, step: u32) -> bool {
		let horizontal_step = step % self.blizzards.width();
		let Vec2 { x, y } = self.position;
		let increasing_x =
			wrapping_sub(x as u32, horizontal_step, 1..self.blizzards.width() + 1) as u8;
		let decreasing_x =
			wrapping_add(x as u32, horizontal_step, 1..self.blizzards.width() + 1) as u8;
		let horizontal_blizzards = &self.blizzards.horizontal[y as usize];
		let vertical_step = step % self.blizzards.height();
		let increasing_y =
			wrapping_sub(y as u32, vertical_step, 1..self.blizzards.height() + 1) as u8;
		let decreasing_y =
			wrapping_add(y as u32, vertical_step, 1..self.blizzards.height() + 1) as u8;
		let vertical_blizzards = &self.blizzards.vertical[x as usize];

		horizontal_blizzards.increasing.contains(&increasing_x)
			|| horizontal_blizzards.decreasing.contains(&decreasing_x)
			|| vertical_blizzards.increasing.contains(&increasing_y)
			|| vertical_blizzards.decreasing.contains(&decreasing_y)
	}
}
