use std::{
	fmt::{Debug, Display},
	ops::{Index, IndexMut},
};

use crate::{FlatPoint, Point, Vec2};

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct Grid<T> {
	cells: Vec<T>,
	width: usize,
}

impl<T> Grid<T> {
	pub fn new(cells: impl IntoIterator<Item = impl IntoIterator<Item = T>>) -> Self {
		let mut height = 0;
		let cells: Vec<_> = cells
			.into_iter()
			.flat_map(|inner| {
				height += 1;
				inner
			})
			.collect();
		let width = cells.len() / height;
		Self { cells, width }
	}
	/// # Panics
	/// Panics if the point is out of bounds.
	pub fn get_flat_point_ref(&self, point: FlatPoint) -> &T {
		&self.cells[point.index]
	}
	/// # Panics
	/// Panics if the point is out of bounds.
	pub fn get_flat_point_mut(&mut self, point: FlatPoint) -> &mut T {
		&mut self.cells[point.index]
	}
	/// # Panics
	/// Panics or gives wrong results if the point is out of bounds.
	pub fn get_point_ref<P: Into<usize>>(&self, point: Point<P>) -> &T {
		self.get_flat_point_ref(FlatPoint::from_point(point, self.width))
	}
	/// # Panics
	/// Panics or gives wrong results if the point is out of bounds.
	pub fn get_point_mut<P: Into<usize>>(&mut self, point: Point<P>) -> &mut T {
		self.get_flat_point_mut(FlatPoint::from_point(point, self.width))
	}
	/// # Panics
	/// Panics or gives wrong results if the point is out of bounds.
	pub fn get_vec2_ref<P: Into<usize>>(&self, point: Vec2<P>) -> &T {
		self.get_flat_point_ref(FlatPoint::from_vec2(point, self.width))
	}
	/// # Panics
	/// Panics or gives wrong results if the point is out of bounds.
	pub fn get_vec2_mut<P: Into<usize>>(&mut self, point: Vec2<P>) -> &mut T {
		self.get_flat_point_mut(FlatPoint::from_vec2(point, self.width))
	}
	pub fn width(&self) -> usize {
		self.width
	}
	pub fn size(&self) -> usize {
		self.cells.len()
	}
	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.cells.iter()
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.cells.iter_mut()
	}
}

impl<T: Clone> Grid<T> {
	pub fn empty(width: usize, height: usize, filler: T) -> Self {
		let mut cells = Vec::with_capacity(width * height);
		for _ in 0..width * height {
			cells.push(filler.clone());
		}
		Self { cells, width }
	}
	pub fn with_margin(
		cells: impl IntoIterator<Item = impl IntoIterator<Item = T>>,
		filler: T,
	) -> Self {
		let mut height = 0;
		let temp_cells: Vec<_> = cells
			.into_iter()
			.flat_map(|inner| {
				height += 1;
				[filler.clone()]
					.into_iter()
					.chain(inner)
					.chain([filler.clone()])
			})
			.collect();
		let width = temp_cells.len() / height;
		let mut cells = Vec::with_capacity(temp_cells.len() + width * 2);
		for _ in 0..width {
			cells.push(filler.clone());
		}
		cells.extend(temp_cells);
		for _ in 0..width {
			cells.push(filler.clone());
		}
		Self { cells, width }
	}
	/// # Panics
	/// Panics if the point is out of bounds.
	pub fn get_flat_point(&self, point: FlatPoint) -> T {
		self.cells[point.index].clone()
	}
	/// # Panics
	/// Panics or gives wrong results if the point is out of bounds.
	pub fn get_point<P: Into<usize>>(&self, point: Point<P>) -> T {
		self.get_flat_point(FlatPoint::from_point(point, self.width))
	}
	/// # Panics
	/// Panics or gives wrong results if the point is out of bounds.
	pub fn get_vec2<P: Into<usize>>(&self, vec: Vec2<P>) -> T {
		self.get_flat_point(FlatPoint::from_vec2(vec, self.width))
	}
	pub fn print_with<F, R>(&self, conversion: F)
	where
		F: Fn(&T) -> R,
		R: Display,
	{
		for (i, cell) in self.iter().enumerate() {
			if (i + 1) % self.width == 0 {
				println!("{}", conversion(cell));
			} else {
				print!("{}", conversion(cell));
			}
		}
	}
}

impl<T, P> Index<Point<P>> for Grid<T>
where
	P: Into<usize>,
{
	type Output = T;
	fn index(&self, index: Point<P>) -> &Self::Output {
		self.get_point_ref(index)
	}
}

impl<T, P: Into<usize>> IndexMut<Point<P>> for Grid<T> {
	fn index_mut(&mut self, index: Point<P>) -> &mut Self::Output {
		self.get_point_mut(index)
	}
}

impl<T> Index<FlatPoint> for Grid<T> {
	type Output = T;
	fn index(&self, index: FlatPoint) -> &Self::Output {
		self.get_flat_point_ref(index)
	}
}

impl<T> IndexMut<FlatPoint> for Grid<T> {
	fn index_mut(&mut self, index: FlatPoint) -> &mut Self::Output {
		self.get_flat_point_mut(index)
	}
}

impl<T, P> Index<Vec2<P>> for Grid<T>
where
	P: Into<usize>,
{
	type Output = T;
	fn index(&self, index: Vec2<P>) -> &Self::Output {
		self.get_vec2_ref(index)
	}
}

impl<T, P> IndexMut<Vec2<P>> for Grid<T>
where
	P: Into<usize>,
{
	fn index_mut(&mut self, index: Vec2<P>) -> &mut Self::Output {
		self.get_vec2_mut(index)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn grid_construction() {
		let text = "123\n456\n789\nabc";
		let grid = Grid::new(text.lines().map(|line| line.chars()));
		assert_eq!(
			grid.cells,
			Vec::from(['1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c'])
		);
		assert_eq!(grid.width, 3);
	}
	#[test]
	fn grid_margin() {
		let grid = Grid::with_margin([['A', 'B'], ['C', 'D']], ' ');
		assert_eq!(
			grid.cells,
			Vec::from([
				' ', ' ', ' ', ' ', ' ', 'A', 'B', ' ', ' ', 'C', 'D', ' ', ' ', ' ', ' ', ' '
			])
		);
		assert_eq!(grid.width, 4);
	}
	#[test]
	fn cell_access() {
		let grid = Grid::with_margin([['A', 'B'], ['C', 'D']], ' ');
		assert_eq!(grid[FlatPoint { index: 5 }], 'A');
		assert_eq!(grid[Point::<usize> { x: 1, y: 1 }], 'A');
		assert_eq!(grid.get_point(Point::<usize> { x: 1, y: 1 }), 'A');
	}
}
