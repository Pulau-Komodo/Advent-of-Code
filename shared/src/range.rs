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

impl IntoIterator for Range<u8> {
	type Item = u8;
	type IntoIter = std::ops::Range<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.start..self.end
	}
}
impl IntoIterator for Range<u16> {
	type Item = u16;
	type IntoIter = std::ops::Range<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.start..self.end
	}
}
impl IntoIterator for Range<u32> {
	type Item = u32;
	type IntoIter = std::ops::Range<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.start..self.end
	}
}
impl IntoIterator for Range<u64> {
	type Item = u64;
	type IntoIter = std::ops::Range<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.start..self.end
	}
}
impl IntoIterator for Range<u128> {
	type Item = u128;
	type IntoIter = std::ops::Range<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.start..self.end
	}
}
impl IntoIterator for Range<i8> {
	type Item = i8;
	type IntoIter = std::ops::Range<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.start..self.end
	}
}
impl IntoIterator for Range<i16> {
	type Item = i16;
	type IntoIter = std::ops::Range<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.start..self.end
	}
}
impl IntoIterator for Range<i32> {
	type Item = i32;
	type IntoIter = std::ops::Range<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.start..self.end
	}
}
impl IntoIterator for Range<i64> {
	type Item = i64;
	type IntoIter = std::ops::Range<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.start..self.end
	}
}
impl IntoIterator for Range<i128> {
	type Item = i128;
	type IntoIter = std::ops::Range<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.start..self.end
	}
}
impl IntoIterator for Range<usize> {
	type Item = usize;
	type IntoIter = std::ops::Range<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.start..self.end
	}
}
impl IntoIterator for Range<isize> {
	type Item = isize;
	type IntoIter = std::ops::Range<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.start..self.end
	}
}
