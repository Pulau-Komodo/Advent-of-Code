use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Range<T> {
	pub start: T,
	pub end: T,
}

impl<T> Range<T> {
	pub fn new(start: T, end: T) -> Self {
		Self { start, end }
	}
}

impl<T> Range<T>
where
	T: Sub<Output = T> + Clone,
{
	pub fn len(&self) -> T {
		self.end.clone() - self.start.clone()
	}
}

impl<T> Range<T>
where
	T: PartialOrd,
{
	pub fn contains(&self, value: &T) -> bool {
		value >= &self.start && value < &self.end
	}
}

impl<T> From<std::ops::Range<T>> for Range<T> {
	fn from(value: std::ops::Range<T>) -> Self {
		Self::new(value.start, value.end)
	}
}

impl<T> IntoIterator for Range<T>
where
	std::ops::Range<T>: Iterator<Item = T>,
{
	type Item = T;
	type IntoIter = std::ops::Range<T>;

	fn into_iter(self) -> Self::IntoIter {
		self.start..self.end
	}
}
