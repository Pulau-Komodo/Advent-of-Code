use std::ops::Range;

fn main() {
	shared::print_answers(23, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let layout = Layout::from_str(input, false);
	find_cheapest_path(layout).unwrap()
}

fn get_answer_2(input: &str) -> u32 {
	let layout = Layout::from_str(input, true);
	find_cheapest_path(layout).unwrap()
}

fn find_cheapest_path(layout: Layout) -> Option<u32> {
	let (valid_moves_into_room, valid_moves_into_hallway) = layout.valid_moves();
	valid_moves_into_room
		.filter_map(|m| {
			let mut clone = layout.clone();
			let cost = clone.perform_move(m);
			if clone.is_completed() {
				Some(cost)
			} else {
				find_cheapest_path(clone).map(|added_cost| cost + added_cost)
			}
		})
		.min()
		.or_else(|| {
			valid_moves_into_hallway
				.filter_map(|m| {
					let mut clone = layout.clone();
					let cost = clone.perform_move(m);
					if clone.is_completed() {
						Some(cost)
					} else {
						find_cheapest_path(clone).map(|added_cost| cost + added_cost)
					}
				})
				.min()
		})
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Letter {
	A = 0,
	B = 1,
	C = 2,
	D = 3,
}

impl Letter {
	fn from_char(char: char) -> Self {
		match char {
			'A' => Self::A,
			'B' => Self::B,
			'C' => Self::C,
			'D' => Self::D,
			_ => panic!("Invalid letter"),
		}
	}
}

const ROOM_LOCATIONS: [usize; 4] = [2, 4, 6, 8];

#[derive(Debug, Clone)]
struct Layout {
	hallway: [Option<Letter>; 11],
	rooms: [Vec<Letter>; 4],
	room_size: usize,
}

impl Layout {
	fn from_str(str: &str, part_two: bool) -> Self {
		let hallway = [None; 11];
		let room_size = if part_two { 4 } else { 2 };
		let mut rooms = [
			vec![Letter::A; room_size],
			vec![Letter::A; room_size],
			vec![Letter::A; room_size],
			vec![Letter::A; room_size],
		];
		let chain = if part_two {
			vec!["  #D#C#B#A#", "  #D#B#A#C#"]
		} else {
			Vec::with_capacity(0)
		};
		for (y, line) in str
			.lines()
			.skip(2)
			.take(1)
			.chain(chain)
			.chain(str.lines().skip(3).take(1))
			.enumerate()
		{
			for (x, char) in line.chars().skip(3).step_by(2).take(4).enumerate() {
				rooms[x][room_size - y - 1] = Letter::from_char(char);
			}
		}
		Self {
			hallway,
			rooms,
			room_size,
		}
	}
	fn perform_move(&mut self, Move { from, to }: Move) -> u32 {
		match (from, to) {
			(Place::Room(from_room), Place::Room(to_room)) => {
				let distance = difference(ROOM_LOCATIONS[from_room], ROOM_LOCATIONS[to_room])
					+ 1 + 2 * self.room_size
					- self.rooms[from_room].len()
					- self.rooms[to_room].len();
				let letter = self.rooms[from_room].pop().unwrap();
				let cost = 10_u32.pow(letter as u32);
				self.rooms[to_room].push(letter);
				cost * distance as u32
			}
			(Place::Room(room), Place::Hallway(hallway)) => {
				let distance = difference(ROOM_LOCATIONS[room], hallway) + 1 + self.room_size
					- self.rooms[room].len();
				let letter = self.rooms[room].pop().unwrap();
				let cost = 10_u32.pow(letter as u32);
				self.hallway[hallway] = Some(letter);
				cost * distance as u32
			}
			(Place::Hallway(hallway), Place::Room(room)) => {
				let distance = difference(ROOM_LOCATIONS[room], hallway) + self.room_size
					- self.rooms[room].len();
				let letter = self.hallway[hallway].unwrap();
				self.hallway[hallway] = None;
				let cost = 10_u32.pow(letter as u32);
				self.rooms[room].push(letter);
				cost * distance as u32
			}
			_ => panic!(),
		}
	}
	fn valid_moves(
		&self,
	) -> (
		impl Iterator<Item = Move> + '_,
		impl Iterator<Item = Move> + '_,
	) {
		use Place::{Hallway, Room};
		(
			(0..4)
				.flat_map(|from_room| {
					(0..4).map(move |to_room| Move {
						from: Room(from_room),
						to: Room(to_room),
					})
				})
				.chain((0..11).flat_map(|hallway| {
					(0..4).map(move |room| Move {
						from: Hallway(hallway),
						to: Room(room),
					})
				}))
				.filter(move |m| self.is_valid_move(*m)),
			(0..11)
				.flat_map(|hallway| {
					(0..4).map(move |room| Move {
						from: Room(room),
						to: Hallway(hallway),
					})
				})
				.filter(move |m| self.is_valid_move(*m)),
		)
	}
	fn is_valid_move(&self, Move { from, to }: Move) -> bool {
		use Place::{Hallway, Room};
		match (from, to) {
			(Hallway(_), Hallway(_)) => false,
			(_, Hallway(hallway)) if ROOM_LOCATIONS.contains(&hallway) => false,
			(Room(room), Hallway(hallway)) => {
				!self.rooms[room].is_empty()
					&& self.rooms[room]
						.iter()
						.any(|&letter| letter as usize != room)
					&& range(ROOM_LOCATIONS[room], hallway)
						.all(|index| self.hallway[index].is_none())
			}
			(Hallway(hallway), Room(room)) => {
				self.hallway[hallway]
					.map(|letter| letter as usize == room)
					.unwrap_or(false) && self.rooms[room].iter().all(|&item| item as usize == room)
					&& range(ROOM_LOCATIONS[room], hallway)
						.filter(|&index| index != hallway)
						.all(|index| self.hallway[index].is_none())
			}
			(Room(from_room), Room(to_room)) => {
				self.rooms[from_room]
					.iter()
					.any(|&item| item as usize != from_room)
					&& self.rooms[from_room]
						.last()
						.map(|&letter| letter as usize == to_room)
						.unwrap_or(false) && self.rooms[to_room]
					.iter()
					.all(|&item| item as usize == to_room)
					&& range(ROOM_LOCATIONS[from_room], ROOM_LOCATIONS[to_room])
						.all(|index| self.hallway[index].is_none())
			}
		}
	}
	fn is_completed(&self) -> bool {
		self.rooms.iter().enumerate().all(|(index, room)| {
			room.len() == self.room_size && room.iter().all(|&letter| letter as usize == index)
		})
	}
	fn _print(&self) {
		let hallway: String = self
			.hallway
			.iter()
			.map(|cell| match cell {
				None => ' ',
				Some(Letter::A) => 'A',
				Some(Letter::B) => 'B',
				Some(Letter::C) => 'C',
				Some(Letter::D) => 'D',
			})
			.collect();
		println!("{}", hallway);
		for i in (0..self.room_size).rev() {
			let line: String = self
				.rooms
				.iter()
				.map(|room| match room.get(i) {
					None => " #",
					Some(Letter::A) => "A#",
					Some(Letter::B) => "B#",
					Some(Letter::C) => "C#",
					Some(Letter::D) => "D#",
				})
				.collect();
			println!(" #{}", line)
		}
	}
}

fn range(a: usize, b: usize) -> Range<usize> {
	if a < b {
		a..b + 1
	} else {
		b..a + 1
	}
}

fn difference(a: usize, b: usize) -> usize {
	if a > b {
		a - b
	} else {
		b - a
	}
}

#[derive(Debug, Clone, Copy)]
enum Place {
	Room(usize),
	Hallway(usize),
}

#[derive(Debug, Clone, Copy)]
struct Move {
	from: Place,
	to: Place,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_completed() {
		let layout = Layout {
			hallway: [None; 11],
			rooms: [
				vec![Letter::A; 4],
				vec![Letter::B; 4],
				vec![Letter::C; 4],
				vec![Letter::D; 4],
			],
			room_size: 4,
		};
		layout._print();
		assert!(layout.is_completed());
	}
	#[test]
	fn test_easy_solution() {
		let layout = Layout {
			hallway: [
				Some(Letter::A),
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
			],
			rooms: [
				vec![Letter::A; 3],
				vec![Letter::B; 4],
				vec![Letter::C; 4],
				vec![Letter::D; 4],
			],
			room_size: 4,
		};
		layout._print();
		let cost = find_cheapest_path(layout);
		assert_eq!(cost, Some(3));
	}
}
