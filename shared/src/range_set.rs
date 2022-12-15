use std::{
	iter::Sum,
	ops::{Add, Range, Sub},
};

#[derive(Debug, Default)]
pub struct RangeSet<T> {
	ranges: Vec<Range<T>>,
}

impl<T> RangeSet<T> {
	pub fn new() -> Self {
		Self { ranges: Vec::new() }
	}
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			ranges: Vec::with_capacity(capacity),
		}
	}
	pub fn count(&self) -> usize {
		self.ranges.len()
	}
	pub fn clear(&mut self) {
		self.ranges.clear()
	}
}

impl<T> RangeSet<T>
where
	T: Copy,
{
	pub fn start(&self) -> Option<T> {
		self.ranges.first().map(|range| range.start)
	}
	pub fn end(&self) -> Option<T> {
		self.ranges.last().map(|range| range.end)
	}
}

impl<T> RangeSet<T>
where
	T: Ord + Copy,
{
	pub fn insert(&mut self, mut range: Range<T>) {
		for index in 0..self.ranges.len() {
			if range.is_empty() {
				return;
			}
			let existing_range = self.ranges.get_mut(index).unwrap();
			if range.end < existing_range.start {
				self.ranges.insert(index, range);
				return;
			} else if range.start > existing_range.end {
				continue;
			}
			existing_range.start = existing_range.start.min(range.start);
			range.start = existing_range.end.max(range.start);
		}
		if !range.is_empty() {
			self.ranges.push(range);
		}
	}
	pub fn consolidate(&mut self) {
		for i in (1..self.ranges.len()).rev() {
			let second = self.ranges[i].clone();
			let first = self.ranges.get_mut(i - 1).unwrap();
			if first.end == second.start {
				first.end = second.end;
				self.ranges.remove(i);
			}
		}
	}
	pub fn gaps(&self) -> impl Iterator<Item = Range<T>> + '_ {
		self.ranges.windows(2).filter_map(|ranges| {
			if let [a, b] = ranges {
				(a.end < b.start).then_some(a.end..b.start)
			} else {
				None
			}
		})
	}
}

impl<T> RangeSet<T>
where
	T: Copy + Add<Output = T> + Sub<Output = T> + Sum,
{
	pub fn len_sum(&self) -> T {
		self.ranges
			.iter()
			.map(|range| range.end - range.start)
			.sum()
	}
}

impl<T> FromIterator<Range<T>> for RangeSet<T>
where
	T: Ord + Copy,
{
	fn from_iter<I: IntoIterator<Item = Range<T>>>(iter: I) -> Self {
		let iter = iter.into_iter();
		let mut ranges = Vec::with_capacity(iter.size_hint().0);
		for item in iter {
			ranges.push(item);
		}
		Self { ranges }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn construction() {
		let ranges = [
			20..30,
			25..27,
			26..31,
			0..10,
			5..15,
			0..10,
			5..9,
			100..99,
			1..11,
			40..60,
			35..65,
			34..35,
		];
		let mut range_set = RangeSet::with_capacity(10);
		for range in ranges {
			range_set.insert(range);
		}
		assert_eq!(
			range_set.ranges,
			[0..10, 10..15, 20..30, 30..31, 34..60, 60..65].to_vec()
		);
		assert_eq!(range_set.len_sum(), 57);
	}
	#[test]
	fn consolidation() {
		let mut range_set: RangeSet<_> = [0..10, 10..15, 20..30, 30..31, 34..60, 60..65]
			.into_iter()
			.collect();
		assert_eq!(range_set.len_sum(), 57);
		range_set.consolidate();
		assert_eq!(range_set.ranges, [0..15, 20..31, 34..65]);
		assert_eq!(range_set.len_sum(), 57);
	}
	#[test]
	fn gaps() {
		let mut range_set: RangeSet<_> = [0..10, 10..15, 20..30, 30..31, 34..60, 60..65]
			.into_iter()
			.collect();
		let gaps: Vec<_> = range_set.gaps().collect();
		assert_eq![gaps, [15..20, 31..34]];
		range_set.consolidate();
		let gaps: Vec<_> = range_set.gaps().collect();
		assert_eq![gaps, [15..20, 31..34]];
	}
}
