use std::{
	fmt::Debug,
	iter::Product,
	ops::{Add, AddAssign, Sub},
	str::FromStr,
};

use crate::{internal::one, Vec2};

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point<T> {
	pub x: T,
	pub y: T,
}

impl<T> Point<T> {
	pub const fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}

impl<T> Point<T>
where
	T: Copy + Default + Add<Output = T> + Sub<Output = T> + Product,
{
	/// Yields the 4 orthogonally neighbouring `Point`s on a square grid, in order of up, left, right, bottom.
	///
	/// This assumes a grid with `(x, y)` like
	/// ```txt
	/// (0, 0) (1, 0) (2, 0)
	/// (0, 1) (1, 1) (2, 1)
	/// (0, 2) (1, 2) (2, 2)
	/// ```
	///
	/// Due to a lack of numeric trait bounds, this could technically be called with non-numeric `T`s, but the outcome will likely not make sense.
	///
	/// Will overflow or panic if `T` is unsigned and the `Point` had any zero in it.
	pub fn orthogonal_neighbours(self) -> [Self; 4] {
		let Self { x, y } = self;
		let one = one();
		[
			(x, y - one), // up
			(x - one, y), // left
			(x + one, y), // right
			(x, y + one), // down
		]
		.map(|(x, y)| Self { x, y })
	}
	/// Yields the 8 touching neighbouring `Point`s on a square grid, in reading order (left to right, top to bottom).
	///
	/// This assumes a grid with `(x, y)` like
	/// ```txt
	/// (0, 0) (1, 0) (2, 0)
	/// (0, 1) (1, 1) (2, 1)
	/// (0, 2) (1, 2) (2, 2)
	/// ```
	///
	/// Due to a lack of numeric trait bounds, this could technically be called with non-numeric `T`s, but the outcome will likely not make sense.
	///
	/// Will overflow or panic if `T` is unsigned and the `Point` had any zero in it.
	pub fn neighbours(self) -> [Self; 8] {
		let Self { x, y } = self;
		let one = one();
		[
			(x - one, y - one), // upper left
			(x, y - one),       // up
			(x + one, y - one), // upper right
			(x - one, y),       // left
			(x + one, y),       // right
			(x - one, y + one), // lower left
			(x, y + one),       // down
			(x + one, y + one), // lower right
		]
		.map(|(x, y)| Self { x, y })
	}
}

impl<T> Point<T>
where
	T: Ord + Sub<Output = T> + Copy,
{
	pub fn abs_diff(self, other: Self) -> Offset<T> {
		let x = if self.x > other.x {
			self.x - other.x
		} else {
			other.x - self.x
		};
		let y = if self.y > other.y {
			self.y - other.y
		} else {
			other.y - self.y
		};
		Offset { x, y }
	}
}

impl<T> Point<T>
where
	T: FromStr,
	<T as FromStr>::Err: Debug,
{
	/// Parses the string as a `Point`, assuming an `x,y` input format.
	pub fn from_comma_separated(s: &str) -> Self {
		let (x, y) = s.split_once(',').expect("No comma in string.");
		Point {
			x: x.parse().expect("Could not parse x."),
			y: y.parse().expect("Could not parse y."),
		}
	}
}

impl<T> Add<Offset<T>> for Point<T>
where
	T: Add<Output = T>,
{
	type Output = Self;
	fn add(self, rhs: Offset<T>) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl<T> AddAssign<Offset<T>> for Point<T>
where
	T: AddAssign,
{
	fn add_assign(&mut self, rhs: Offset<T>) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T> Sub<Offset<T>> for Point<T>
where
	T: Sub<Output = T>,
{
	type Output = Point<T>;
	fn sub(self, rhs: Offset<T>) -> Self::Output {
		Point {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl<T> Sub<Point<T>> for Point<T>
where
	T: Sub<Output = T>,
{
	type Output = Offset<T>;
	fn sub(self, rhs: Self) -> Self::Output {
		Offset {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Offset<T> {
	pub x: T,
	pub y: T,
}

impl<T> Offset<T> {
	pub const fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}

impl Offset<usize> {
	pub const X: Self = Self { x: 1, y: 0 };
	pub const Y: Self = Self { x: 0, y: 1 };
}

impl<T> Offset<T>
where
	T: Add<Output = T> + Copy,
{
	pub fn component_sum(self) -> T {
		self.x + self.y
	}
}

/// A point the way it is represented on a flattened grid, i.e. a grid that was made into a single list of cells.
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FlatPoint {
	pub index: usize,
}

impl FlatPoint {
	/// Makes a `FlatPoint` from a `Point`, assuming the given `grid_width`.
	///
	/// `(0, 0)` will be index 0, and rows (`Point`s that share a `y`) are kept sequential.
	pub fn from_point<T: Into<usize>>(point: Point<T>, grid_width: usize) -> Self {
		if cfg!(debug_assertions) {
			let x: usize = point.x.into();
			if x >= grid_width {
				panic!("x is {x} but grid width is {grid_width}");
			}
			let index = point.y.into() * grid_width + x;
			Self { index }
		} else {
			let index = point.y.into() * grid_width + point.x.into();
			Self { index }
		}
	}
	/// Makes a `FlatPoint` into a `Point`, assuming the given `grid_width`.
	///
	/// `(0, 0)` will be index 0, and rows (`Point`s that share a `y`) were kept sequential.
	pub fn into_point<T: From<usize>>(self, grid_width: usize) -> Point<T> {
		let x = self.index % grid_width;
		let y = self.index / grid_width;
		Point {
			x: x.into(),
			y: y.into(),
		}
	}
	/// Makes a `FlatPoint` from a `Vec2`, assuming the given `grid_width`.
	///
	/// `(0, 0)` will be index 0, and rows (`Vec2`s that share a `y`) are kept sequential.
	pub fn from_vec2<T: Into<usize>>(vec: Vec2<T>, grid_width: usize) -> Self {
		if cfg!(debug_assertions) {
			let x: usize = vec.x.into();
			if x >= grid_width {
				panic!("x is {x} but grid width is {grid_width}");
			}
			let index = vec.y.into() * grid_width + x;
			Self { index }
		} else {
			let index = vec.y.into() * grid_width + vec.x.into();
			Self { index }
		}
	}
	/// Makes a `FlatPoint` into a `Vec2`, assuming the given `grid_width`.
	///
	/// `(0, 0)` will be index 0, and rows (`Vec2`s that share a `y`) were kept sequential.
	pub fn into_vec2<T: From<usize>>(self, grid_width: usize) -> Vec2<T> {
		let x = self.index % grid_width;
		let y = self.index / grid_width;
		Vec2 {
			x: x.into(),
			y: y.into(),
		}
	}
	/// Yields the 4 orthogonally neighbouring `FlatPoint`s on a square grid, in order of up, left, right, bottom.
	///
	/// This assumes a grid with `(x, y)` like
	/// ```txt
	/// (0, 0) (1, 0) (2, 0)
	/// (0, 1) (1, 1) (2, 1)
	/// (0, 2) (1, 2) (2, 2)
	/// ```
	///
	/// Will overflow or panic if the `FlatPoint` was at the top of the grid, and give bogus results (wrapped around or out of bounds) if it was at the other edges.
	pub fn orthogonal_neighbours(self, grid_width: usize) -> [Self; 4] {
		let Self { index } = self;
		[index - grid_width, index - 1, index + 1, index + grid_width].map(|index| Self { index })
	}
	/// Yields the 8 touching neighbouring `Point`s on a square grid, in reading order (left to right, top to bottom).
	///
	/// This assumes a grid with `(x, y)` like
	/// ```txt
	/// (0, 0) (1, 0) (2, 0)
	/// (0, 1) (1, 1) (2, 1)
	/// (0, 2) (1, 2) (2, 2)
	/// ```
	///
	/// Will overflow or panic if the `FlatPoint` was at the top of the grid, and give bogus results (wrapped around or out of bounds) if it was at the other edges.
	pub fn neighbours(self, grid_width: usize) -> [Self; 8] {
		let Self { index } = self;
		[
			index - grid_width - 1,
			index - grid_width,
			index - grid_width + 1,
			index - 1,
			index + 1,
			index + grid_width - 1,
			index + grid_width,
			index + grid_width + 1,
		]
		.map(|index| Self { index })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const GRID: [[u8; 5]; 5] = [
		[0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0],
		[0, 1, 2, 3, 0],
		[0, 4, 0, 5, 0],
		[0, 6, 7, 8, 0],
	];
	const POINT: Point<usize> = Point { x: 2, y: 3 };

	#[test]
	fn neighbours() {
		let neighbours = POINT.neighbours().map(|point| GRID[point.y][point.x]);
		assert_eq!(neighbours, [1, 2, 3, 4, 5, 6, 7, 8]);
	}
	#[test]
	fn orthogonal_neighbours() {
		let neighbours = POINT
			.orthogonal_neighbours()
			.map(|point| GRID[point.y][point.x]);
		assert_eq!(neighbours, [2, 4, 5, 7]);
	}
	#[test]
	fn flat_neighbours() {
		let flat_point = FlatPoint::from_point(POINT, GRID[0].len());
		let flat_grid: Vec<_> = GRID.iter().flatten().collect();
		let neighbours = flat_point
			.neighbours(GRID[0].len())
			.map(|point| *flat_grid[point.index]);
		assert_eq!(neighbours, [1, 2, 3, 4, 5, 6, 7, 8]);
	}
	#[test]
	fn flat_orthogonal_neighbours() {
		let flat_point = FlatPoint::from_point(POINT, GRID[0].len());
		let flat_grid: Vec<_> = GRID.iter().flatten().collect();
		let neighbours = flat_point
			.orthogonal_neighbours(GRID[0].len())
			.map(|point| *flat_grid[point.index]);
		assert_eq!(neighbours, [2, 4, 5, 7]);
	}
}
