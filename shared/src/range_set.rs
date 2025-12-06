use std::{
	iter::{Product, Sum},
	ops::{Add, Range, RangeBounds, RangeInclusive, Sub},
};

use crate::internal::one;

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
	T: PartialOrd,
{
	/// Whether the value is in any of the ranges.
	pub fn contains(&self, value: &T) -> bool {
		self.ranges.iter().any(|range| range.contains(value))
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
	/// Merges ranges that have no gap between them.
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
	/// Returns an iterator over the gaps between the ranges.
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
		let mut range_set = Self::with_capacity(iter.size_hint().0);
		for item in iter {
			range_set.insert(item);
		}
		range_set
	}
}

#[derive(Debug, Default)]
pub struct RangeInclusiveSet<T> {
	ranges: Vec<RangeInclusive<T>>,
}

impl<T> RangeInclusiveSet<T> {
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

impl<T> RangeInclusiveSet<T>
where
	T: Copy,
{
	pub fn start(&self) -> Option<T> {
		self.ranges.first().map(|range| range.start()).copied()
	}
	pub fn end(&self) -> Option<T> {
		self.ranges.last().map(|range| range.end()).copied()
	}
}

impl<T> RangeInclusiveSet<T>
where
	T: PartialOrd,
{
	/// Whether the value is in any of the ranges.
	pub fn contains(&self, value: &T) -> bool {
		self.ranges.iter().any(|range| range.contains(value))
	}
}

impl<T> RangeInclusiveSet<T>
where
	T: Ord + Copy,
{
	pub fn insert(&mut self, mut range: RangeInclusive<T>) {
		for index in 0..self.ranges.len() {
			if range.is_empty() {
				return;
			}
			let existing_range = self.ranges.get_mut(index).unwrap();
			if *range.end() < *existing_range.start() {
				self.ranges.insert(index, range);
				return;
			} else if *range.start() > *existing_range.end() {
				continue;
			}
			*existing_range = *existing_range.start().min(range.start())..=*existing_range.end();
			range = *existing_range.end().max(range.start())..=*range.end();
		}
		if !range.is_empty() {
			self.ranges.push(range);
		}
	}
	/// Merges ranges that have no gap between them.
	///
	/// `0..=2, 3..=5` is considered to have a gap, even though for integers there effectively is not one.
	pub fn consolidate(&mut self) {
		for i in (1..self.ranges.len()).rev() {
			let second = self.ranges[i].clone();
			let first = self.ranges.get_mut(i - 1).unwrap();
			if first.end() == second.start() {
				*first = *first.start()..=*second.end();
				self.ranges.remove(i);
			}
		}
	}
	/// Returns an iterator over the gaps between the ranges.
	///
	/// `0..=2, 3..=5` is considered to have a gap, even though for integers there effectively is not one.
	pub fn gaps(&self) -> impl Iterator<Item = RangeDoubleExclusive<T>> + '_ {
		self.ranges.windows(2).filter_map(|ranges| {
			if let [a, b] = ranges {
				(a.end() < b.start()).then_some(RangeDoubleExclusive::new(*a.end(), *b.start()))
			} else {
				None
			}
		})
	}
}

impl<T> RangeInclusiveSet<T>
where
	T: Copy + Add<Output = T> + Sub<Output = T> + Sum + Product,
{
	pub fn len_sum(&self) -> T {
		let one = one();
		self.ranges
			.iter()
			.map(|range| *range.end() - *range.start() + one)
			.sum()
	}
}

impl<T> FromIterator<RangeInclusive<T>> for RangeInclusiveSet<T>
where
	T: Ord + Copy,
{
	fn from_iter<I: IntoIterator<Item = RangeInclusive<T>>>(iter: I) -> Self {
		let iter = iter.into_iter();
		let mut range_set = Self::with_capacity(iter.size_hint().0);
		for item in iter {
			range_set.insert(item);
		}
		range_set
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RangeDoubleExclusive<T> {
	pub start: T,
	pub end: T,
}

impl<T> RangeDoubleExclusive<T> {
	pub fn new(start: T, end: T) -> Self {
		Self { start, end }
	}
}

impl<T> RangeBounds<T> for RangeDoubleExclusive<T> {
	fn start_bound(&self) -> std::ops::Bound<&T> {
		std::ops::Bound::Excluded(&self.start)
	}
	fn end_bound(&self) -> std::ops::Bound<&T> {
		std::ops::Bound::Excluded(&self.end)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const RANGES: [Range<u32>; 13] = [
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
		150..150,
	];

	#[test]
	fn construction() {
		let range_set: RangeSet<_> = RANGES.into_iter().collect();
		assert_eq!(
			range_set.ranges,
			[0..10, 10..15, 20..30, 30..31, 34..60, 60..65].to_vec()
		);
		assert_eq!(range_set.len_sum(), 57);
	}
	#[test]
	fn consolidation() {
		let mut range_set: RangeSet<_> = RANGES.into_iter().collect();
		assert_eq!(range_set.len_sum(), 57);
		range_set.consolidate();
		assert_eq!(range_set.ranges, [0..15, 20..31, 34..65]);
		assert_eq!(range_set.len_sum(), 57);
	}
	#[test]
	fn gaps() {
		let mut range_set: RangeSet<_> = RANGES.into_iter().collect();
		let gaps: Vec<_> = range_set.gaps().collect();
		assert_eq![gaps, [15..20, 31..34]];
		range_set.consolidate();
		let gaps: Vec<_> = range_set.gaps().collect();
		assert_eq![gaps, [15..20, 31..34]];
	}

	const RANGES_INCLUSIVE: [RangeInclusive<u32>; 13] = [
		20..=30,
		25..=27,
		26..=31,
		0..=10,
		5..=15,
		0..=10,
		5..=9,
		100..=99,
		1..=11,
		40..=60,
		35..=65,
		34..=35,
		150..=150,
	];
	#[test]
	fn construction_inclusive() {
		let range_set: RangeInclusiveSet<_> = RANGES_INCLUSIVE.into_iter().collect();
		assert_eq!(
			range_set.ranges,
			[
				0..=10,
				10..=15,
				20..=30,
				30..=31,
				34..=60,
				60..=65,
				150..=150
			]
			.to_vec()
		);
		assert_eq!(range_set.len_sum(), 57);
	}
	#[test]
	fn consolidation_inclusive() {
		let mut range_set: RangeInclusiveSet<_> = RANGES_INCLUSIVE.into_iter().collect();
		assert_eq!(range_set.len_sum(), 57);
		range_set.consolidate();
		assert_eq!(range_set.ranges, [0..=15, 20..=31, 34..=65, 150..=150]);
		assert_eq!(range_set.len_sum(), 57);
	}
	#[test]
	fn gaps_inclusive() {
		type R = RangeDoubleExclusive<u32>;
		let mut range_set: RangeInclusiveSet<_> = RANGES_INCLUSIVE.into_iter().collect();
		let gaps: Vec<_> = range_set.gaps().collect();
		assert_eq![gaps, [R::new(15, 20), R::new(31, 34), R::new(65, 150)]];
		range_set.consolidate();
		let gaps: Vec<_> = range_set.gaps().collect();
		assert_eq![gaps, [R::new(15, 20), R::new(31, 34), R::new(65, 150)]];
	}
}
