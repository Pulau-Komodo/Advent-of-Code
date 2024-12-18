use std::{
	fmt::{Debug, Display, Write},
	ops::{Index, IndexMut},
};

use crate::{FlatPoint, Offset, Point, Vec2};

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct Grid<T> {
	cells: Vec<T>,
	width: usize,
}

impl<T> Grid<T> {
	pub fn new(cells: impl IntoIterator<Item = impl IntoIterator<Item = T>>) -> Self {
		Self::new_internal(cells, std::convert::identity)
	}
	/// A convenience constructor that assumes the input should be separated by lines and chars, because that is by far the most common case.
	pub fn from_chars<F>(str: &str, map: F) -> Self
	where
		F: FnMut(char) -> T,
	{
		Self::new_internal(str.lines().map(|line| line.chars()), map)
	}
	fn new_internal<Input, F>(
		cells: impl IntoIterator<Item = impl IntoIterator<Item = Input>>,
		map: F,
	) -> Self
	where
		F: FnMut(Input) -> T,
	{
		let mut height = 0;
		let cells: Vec<_> = cells
			.into_iter()
			.flat_map(|inner| {
				height += 1;
				inner
			})
			.map(map)
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
	pub fn height(&self) -> usize {
		self.size() / self.width
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
	pub fn iter_with_points<N>(&self) -> impl DoubleEndedIterator<Item = (Point<N>, &T)>
	where
		N: From<usize>,
	{
		self.cells.iter().enumerate().map(|(index, cell)| {
			let x = (index % self.width).into();
			let y = (index / self.width).into();
			(Point { x, y }, cell)
		})
	}
	pub fn map<F, U>(self, f: F) -> Grid<U>
	where
		F: FnMut(T) -> U,
	{
		Grid {
			cells: self.cells.into_iter().map(f).collect(),
			width: self.width,
		}
	}
	pub fn map_ref<F, U>(&self, f: F) -> Grid<U>
	where
		F: FnMut(&T) -> U,
	{
		Grid {
			cells: self.cells.iter().map(f).collect(),
			width: self.width,
		}
	}
	pub fn contains_point(&self, point: Point<usize>) -> bool {
		point.x < self.width() && point.y < self.height()
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
		let mut grid = Self::new(cells);
		grid.add_margin(filler);
		grid
	}
	pub fn with_margin_from_chars<F>(str: &str, filler: T, map: F) -> Self
	where
		F: FnMut(char) -> T,
	{
		let mut grid = Self::from_chars(str, map);
		grid.add_margin(filler);
		grid
	}
	pub fn add_margin(&mut self, filler: T) {
		let mut cells = Vec::with_capacity((self.height() + 2) * (self.width() + 2));

		cells.extend((0..self.width() + 2).map(|_| filler.clone()));

		let height = self.height();
		let mut old_cells = std::mem::take(&mut self.cells).into_iter();
		for _ in 0..height {
			cells.extend(
				[filler.clone()]
					.into_iter()
					.chain((&mut old_cells).take(self.width()))
					.chain([filler.clone()]),
			);
		}

		cells.extend((0..self.width() + 2).map(|_| filler.clone()));

		self.cells = cells;
		self.width += 2;
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
	pub fn write<W, F, R>(&self, mut w: W, conversion: F) -> Result<(), std::fmt::Error>
	where
		W: Write,
		F: Fn(&T) -> R,
		R: Display,
	{
		for (i, cell) in self.iter().enumerate() {
			if (i + 1) % self.width == 0 {
				w.write_fmt(format_args!("{}\n", conversion(cell)))?;
			} else {
				w.write_fmt(format_args!("{}", conversion(cell)))?;
			}
		}
		Ok(())
	}
}

impl Grid<bool> {
	pub fn print_small(&self) {
		const CHARS: [char; 4] = [' ', '▀', '▄', '█'];
		for line in (0..self.height()).step_by(2).map(|y| {
			(0..self.width())
				.map(move |x| Point::new(x, y))
				.map(|point| {
					[Offset::new(0, 0), Offset::Y]
						.map(|offset| point + offset)
						.map(|point| {
							(self.contains_point(point))
								.then(|| self.get_point(point))
								.unwrap_or(true)
						})
				})
		}) {
			for window in line {
				let index = window
					.into_iter()
					.rev()
					.fold(0, |acc, e| acc << 1 | (e as usize));
				print!("{}", CHARS[index ^ 3]);
			}
			println!();
		}
	}
}

impl Grid<bool> {
	pub fn print_tiny(&self) {
		#[rustfmt::skip]
		const CHARS: [char; 16] = [
			' ', '▘', '▝', '▀',
			'▖', '▌', '▞', '▛',
			'▗', '▚', '▐', '▜',
			'▄', '▙', '▟', '█',
		];
		for line in (0..self.height()).step_by(2).map(|y| {
			(0..self.width())
				.step_by(2)
				.map(move |x| Point::new(x, y))
				.map(|point| {
					[Offset::new(0, 0), Offset::X, Offset::Y, Offset::new(1, 1)]
						.map(|offset| point + offset)
						.map(|point| {
							(self.contains_point(point))
								.then(|| self.get_point(point))
								.unwrap_or(true)
						})
				})
		}) {
			for window in line {
				let index = window
					.into_iter()
					.rev()
					.fold(0, |acc, e| acc << 1 | (e as usize));
				print!("{}", CHARS[index ^ 15]);
			}
			println!();
		}
	}
}

impl Grid<bool> {
	pub fn print_braille(&self) {
		#[rustfmt::skip]
		/// ```
		/// 1 4
		/// 2 5
		/// 3 6
		/// 7 8
		/// ```
		const CHARS: [char; 256] = [
			'⠀','⠁','⠂','⠃','⠄','⠅','⠆','⠇','⠈','⠉','⠊','⠋','⠌','⠍','⠎','⠏',
			'⠐','⠑','⠒','⠓','⠔','⠕','⠖','⠗','⠘','⠙','⠚','⠛','⠜','⠝','⠞','⠟',
			'⠠','⠡','⠢','⠣','⠤','⠥','⠦','⠧','⠨','⠩','⠪','⠫','⠬','⠭','⠮','⠯',
			'⠰','⠱','⠲','⠳','⠴','⠵','⠶','⠷','⠸','⠹','⠺','⠻','⠼','⠽','⠾','⠿',
			'⡀','⡁','⡂','⡃','⡄','⡅','⡆','⡇','⡈','⡉','⡊','⡋','⡌','⡍','⡎','⡏',
			'⡐','⡑','⡒','⡓','⡔','⡕','⡖','⡗','⡘','⡙','⡚','⡛','⡜','⡝','⡞','⡟',
			'⡠','⡡','⡢','⡣','⡤','⡥','⡦','⡧','⡨','⡩','⡪','⡫','⡬','⡭','⡮','⡯',
			'⡰','⡱','⡲','⡳','⡴','⡵','⡶','⡷','⡸','⡹','⡺','⡻','⡼','⡽','⡾','⡿',
			'⢀','⢁','⢂','⢃','⢄','⢅','⢆','⢇','⢈','⢉','⢊','⢋','⢌','⢍','⢎','⢏',
			'⢐','⢑','⢒','⢓','⢔','⢕','⢖','⢗','⢘','⢙','⢚','⢛','⢜','⢝','⢞','⢟',
			'⢠','⢡','⢢','⢣','⢤','⢥','⢦','⢧','⢨','⢩','⢪','⢫','⢬','⢭','⢮','⢯',
			'⢰','⢱','⢲','⢳','⢴','⢵','⢶','⢷','⢸','⢹','⢺','⢻','⢼','⢽','⢾','⢿',
			'⣀','⣁','⣂','⣃','⣄','⣅','⣆','⣇','⣈','⣉','⣊','⣋','⣌','⣍','⣎','⣏',
			'⣐','⣑','⣒','⣓','⣔','⣕','⣖','⣗','⣘','⣙','⣚','⣛','⣜','⣝','⣞','⣟',
			'⣠','⣡','⣢','⣣','⣤','⣥','⣦','⣧','⣨','⣩','⣪','⣫','⣬','⣭','⣮','⣯',
			'⣰','⣱','⣲','⣳','⣴','⣵','⣶','⣷','⣸','⣹','⣺','⣻','⣼','⣽','⣾','⣿',
		];
		for line in (0..self.height()).step_by(4).map(|y| {
			(0..self.width())
				.step_by(2)
				.map(move |x| Point::new(x, y))
				.map(|point| {
					[
						Offset::new(0, 0),
						Offset::new(0, 1),
						Offset::new(0, 2),
						Offset::new(1, 0),
						Offset::new(1, 1),
						Offset::new(1, 2),
						Offset::new(0, 3),
						Offset::new(1, 3),
					]
					.map(|offset| point + offset)
					.map(|point| {
						(self.contains_point(point))
							.then(|| self.get_point(point))
							.unwrap_or(true)
					})
				})
		}) {
			for window in line {
				let index = window
					.into_iter()
					.rev()
					.fold(0, |acc, e| acc << 1 | (e as usize));
				print!("{}", CHARS[index ^ 255]);
			}
			println!();
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
	fn grid_from_chars() {
		let grid = Grid::from_chars("123\n456\n789\nabc", std::convert::identity);
		assert_eq!(
			grid.cells,
			Vec::from(['1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c'])
		);
		assert_eq!(grid.width, 3);
	}
	#[test]
	fn grid_margin_from_chars() {
		let grid = Grid::with_margin_from_chars("AB\nCD", ' ', std::convert::identity);
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
