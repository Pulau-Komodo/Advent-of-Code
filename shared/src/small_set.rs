use crate::SmallMap;

pub struct SmallSet<T>
where
	T: Eq,
{
	map: SmallMap<T, ()>,
}

impl<T> SmallSet<T>
where
	T: Eq,
{
	pub fn new() -> Self {
		Self {
			map: SmallMap::new(),
		}
	}
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			map: SmallMap::with_capacity(capacity),
		}
	}
	pub fn len(&self) -> usize {
		self.map.len()
	}
	pub fn is_empty(&self) -> bool {
		self.map.is_empty()
	}
	pub fn contains(&self, value: &T) -> bool {
		self.map.contains_key(value)
	}
	/// Inserts a value into the map. Returns whether that value was newly inserted.
	pub fn insert(&mut self, value: T) -> bool {
		self.map.insert(value, ()).is_none()
	}
	pub fn remove(&mut self, value: &T) -> bool {
		self.map.remove(value).is_some()
	}
	pub fn clear(&mut self) {
		self.map.clear();
	}
	pub fn iter(&self) -> impl ExactSizeIterator<Item = &T> + DoubleEndedIterator<Item = &T> {
		self.map.keys()
	}
}

impl<T> Default for SmallSet<T>
where
	T: Eq,
{
	fn default() -> Self {
		Self::new()
	}
}

impl<T> std::fmt::Debug for SmallSet<T>
where
	T: std::fmt::Debug + Eq,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_set().entries(self.iter()).finish()
	}
}

impl<T> IntoIterator for SmallSet<T>
where
	T: Eq,
{
	type Item = T;

	type IntoIter = std::vec::IntoIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		self.map.keys.into_iter()
	}
}

impl<T> FromIterator<T> for SmallSet<T>
where
	T: Eq,
{
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let iter = iter.into_iter();
		let mut map = Self::with_capacity(iter.size_hint().0);
		for value in iter {
			map.insert(value);
		}
		map
	}
}
