use std::{iter::Product, ops::Neg};

use crate::{internal::one, Offset};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl Direction {
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
