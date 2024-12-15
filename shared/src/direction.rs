use std::{iter::Product, ops::Neg};

use crate::{internal::one, Offset};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl Direction {
	pub fn from_byte(byte: u8) -> Self {
		match byte {
			b'^' => Direction::Up,
			b'>' => Direction::Right,
			b'v' => Direction::Down,
			b'<' => Direction::Left,
			_ => panic!(
				"Unexpected byte for direction: {byte} ({}). Expected bytes {} ({}), {} ({}), {} ({}) or {} ({}).",
				<u8 as Into<char>>::into(byte),
				b'^',
				'^',
				b'>',
				'>',
				b'v',
				'v',
				b'<',
				'<',
			),
		}
	}
	pub fn turn_right(self) -> Self {
		match self {
			Self::Up => Self::Right,
			Self::Right => Self::Down,
			Self::Down => Self::Left,
			Self::Left => Self::Up,
		}
	}
	pub fn turn_left(self) -> Self {
		match self {
			Self::Up => Self::Left,
			Self::Right => Self::Up,
			Self::Down => Self::Right,
			Self::Left => Self::Down,
		}
	}
	pub fn reverse(self) -> Self {
		match self {
			Self::Up => Self::Down,
			Self::Right => Self::Left,
			Self::Down => Self::Up,
			Self::Left => Self::Right,
		}
	}
	pub fn turn_right_mut(&mut self) {
		*self = self.turn_right();
	}
	pub fn turn_left_mut(&mut self) {
		*self = self.turn_left();
	}
	pub fn reverse_mut(&mut self) {
		*self = self.reverse();
	}
	pub fn into_offset<T>(self) -> Offset<T>
	where
		T: Default + Product + Neg<Output = T>,
	{
		match self {
			Self::Up => Offset {
				x: T::default(),
				y: -one::<T>(),
			},
			Self::Right => Offset {
				x: one(),
				y: T::default(),
			},
			Self::Down => Offset {
				x: T::default(),
				y: one(),
			},
			Self::Left => Offset {
				x: -one::<T>(),
				y: T::default(),
			},
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction8 {
	Up,
	UpRight,
	Right,
	DownRight,
	Down,
	DownLeft,
	Left,
	UpLeft,
}

impl Direction8 {
	pub fn each() -> [Self; 8] {
		[
			Self::Up,
			Self::UpRight,
			Self::Right,
			Self::DownRight,
			Self::Down,
			Self::DownLeft,
			Self::Left,
			Self::UpLeft,
		]
	}
	pub fn into_offset<T>(self) -> Offset<T>
	where
		T: Default + Product + Neg<Output = T>,
	{
		match self {
			Self::Up => Offset {
				x: T::default(),
				y: -one::<T>(),
			},
			Self::UpRight => Offset {
				x: one::<T>(),
				y: -one::<T>(),
			},
			Self::Right => Offset {
				x: one::<T>(),
				y: T::default(),
			},
			Self::DownRight => Offset {
				x: one::<T>(),
				y: one::<T>(),
			},
			Self::Down => Offset {
				x: T::default(),
				y: one::<T>(),
			},
			Self::DownLeft => Offset {
				x: -one::<T>(),
				y: one::<T>(),
			},
			Self::Left => Offset {
				x: -one::<T>(),
				y: T::default(),
			},
			Self::UpLeft => Offset {
				x: -one::<T>(),
				y: -one::<T>(),
			},
		}
	}
}
