use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct SmallMap<K, V>
where
	K: Eq,
{
	pub(super) keys: Vec<K>,
	values: Vec<V>,
}

impl<K, V> SmallMap<K, V>
where
	K: Eq,
{
	pub fn new() -> Self {
		Self {
			keys: Vec::new(),
			values: Vec::new(),
		}
	}
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			keys: Vec::with_capacity(capacity),
			values: Vec::with_capacity(capacity),
		}
	}
	pub fn len(&self) -> usize {
		self.keys.len()
	}
	pub fn is_empty(&self) -> bool {
		self.keys.is_empty()
	}
	pub fn get(&self, k: &K) -> Option<&V> {
		Some(&self.values[self.keys.iter().position(|key| key == k)?])
	}
	pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
		Some(&mut self.values[self.keys.iter().position(|key| key == k)?])
	}
	/// Inserts a key-value pair into the map.
	///
	/// If the map did not have this key present, None is returned.
	///
	/// If the map did have this key present, the value is updated, and the old value is returned. The key is not updated, though; this matters for types that can be == without being identical.
	pub fn insert(&mut self, k: K, mut v: V) -> Option<V> {
		if let Some(old_value) = self.get_mut(&k) {
			std::mem::swap(old_value, &mut v);
			Some(v)
		} else {
			self.keys.push(k);
			self.values.push(v);
			None
		}
	}
	pub fn get_mut_or_insert(&mut self, k: K, v: V) -> &mut V {
		if let Some(position) = self.keys.iter().position(|key| key == &k) {
			&mut self.values[position]
		} else {
			let len = self.values.len();
			self.keys.push(k);
			self.values.push(v);
			&mut self.values[len]
		}
	}
	pub fn remove(&mut self, k: &K) -> Option<V> {
		let index = self.keys.iter().position(|key| key == k)?;
		let _key = self.keys.swap_remove(index);
		let value = self.values.swap_remove(index);
		Some(value)
	}
	pub fn contains_key(&self, k: &K) -> bool {
		self.keys.iter().any(|key| k == key)
	}
	pub fn clear(&mut self) {
		self.keys.clear();
		self.values.clear();
	}
	pub fn keys(&self) -> impl ExactSizeIterator<Item = &K> + DoubleEndedIterator<Item = &K> {
		self.keys.iter()
	}
	pub fn values(&self) -> impl ExactSizeIterator<Item = &V> + DoubleEndedIterator<Item = &V> {
		self.values.iter()
	}
	pub fn iter(
		&self,
	) -> impl ExactSizeIterator<Item = (&K, &V)> + DoubleEndedIterator<Item = (&K, &V)> {
		self.keys().zip(self.values())
	}
}

impl<K, V> Index<K> for SmallMap<K, V>
where
	K: Eq,
{
	type Output = V;

	fn index(&self, index: K) -> &Self::Output {
		self.get(&index).unwrap()
	}
}

impl<K, V> IndexMut<K> for SmallMap<K, V>
where
	K: Eq,
{
	fn index_mut(&mut self, index: K) -> &mut Self::Output {
		self.get_mut(&index).unwrap()
	}
}

impl<K, V> Default for SmallMap<K, V>
where
	K: Eq,
{
	fn default() -> Self {
		Self::new()
	}
}

impl<K, V> std::fmt::Debug for SmallMap<K, V>
where
	K: std::fmt::Debug + Eq,
	V: std::fmt::Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_map().entries(self.iter()).finish()
	}
}

impl<K, V> IntoIterator for SmallMap<K, V>
where
	K: Eq,
{
	type Item = (K, V);

	type IntoIter = std::iter::Zip<std::vec::IntoIter<K>, std::vec::IntoIter<V>>;

	fn into_iter(self) -> Self::IntoIter {
		self.keys.into_iter().zip(self.values)
	}
}

impl<K, V> FromIterator<(K, V)> for SmallMap<K, V>
where
	K: Eq,
{
	fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
		let iter = iter.into_iter();
		let mut map = Self::with_capacity(iter.size_hint().0);
		for (key, value) in iter {
			map.insert(key, value);
		}
		map
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_mut_insert() {
		let mut map = SmallMap::new();
		map.insert('c', 3);
		*map.get_mut_or_insert('c', 500) += 1;
		*map.get_mut_or_insert('d', 10) *= 10;
		assert_eq!(map.get(&'c'), Some(&4));
		assert_eq!(map.get(&'d'), Some(&100));
		assert_eq!(map.get(&'a'), None);
	}
	#[test]
	fn test_insert() {
		let mut map = SmallMap::new();
		assert_eq!(map.insert('a', 1), None);
		assert_eq!(map.get(&'b'), None);
		assert_eq!(map.insert('b', 2), None);
		assert_eq!(map.get(&'a'), Some(&1));
		assert_eq!(map.insert('a', 3), Some(1));
		assert_eq!(map.insert('c', 4), None);
		assert_eq!(map.insert('c', 5), Some(4));
		assert_eq!(map.get(&'a'), Some(&3));
		assert_eq!(map.get(&'b'), Some(&2));
		assert_eq!(map.get(&'c'), Some(&5));
		assert_eq!(map.get(&'d'), None);
	}
}
