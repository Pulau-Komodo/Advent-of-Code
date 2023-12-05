use std::ops::Range;

fn main() {
	shared::print_answers(5, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> i64 {
	let mut sections = input.split("\n\n");
	let (_, seeds) = sections.next().unwrap().split_once(": ").unwrap();
	let seeds: Vec<i64> = seeds
		.split_ascii_whitespace()
		.map(|n| n.parse().unwrap())
		.collect();
	let maps: Vec<_> = sections.map(RangeMap::from_str).collect();
	seeds
		.iter()
		.map(|&(mut value)| {
			for map in &maps {
				value = map.convert(value);
			}
			value
		})
		.min()
		.unwrap()
}

fn get_answer_2(input: &str) -> i64 {
	let mut sections = input.split("\n\n");
	let (_, seeds) = sections.next().unwrap().split_once(": ").unwrap();
	let seeds: Vec<i64> = seeds
		.split_ascii_whitespace()
		.map(|n| n.parse().unwrap())
		.collect();
	let mut ranges: Vec<Range<i64>> = seeds
		.chunks_exact(2)
		.map(|range| range[0]..range[0] + range[1])
		.collect();
	let mut new_ranges = Vec::new();
	for map in sections.map(RangeMap::from_str) {
		new_ranges.extend(ranges.drain(..).flat_map(|range| map.convert_range(range)));
		std::mem::swap(&mut ranges, &mut new_ranges);
	}
	ranges.into_iter().map(|range| range.start).min().unwrap()
}

struct RangeMap(Vec<MapRange>);

impl RangeMap {
	fn from_str(str: &str) -> Self {
		let mut map: Vec<_> = str.lines().skip(1).map(MapRange::from_line).collect();
		map.sort();
		Self(map)
	}
	fn convert(&self, number: i64) -> i64 {
		for range in &self.0 {
			if number >= range.range.start {
				if number < range.range.end {
					return number + range.offset;
				}
			} else {
				break;
			}
		}
		number
	}
	fn convert_range(&self, range: Range<i64>) -> Vec<Range<i64>> {
		let mut last_point = 0;
		let mut converted: Vec<_> = self
			.0
			.iter()
			.flat_map(|map_range| {
				let non_offset_overlap = overlap(&range, &(last_point..map_range.range.start));
				last_point = map_range.range.end;
				let mut offset_overlap = overlap(&range, &map_range.range);
				offset_overlap.start += map_range.offset;
				offset_overlap.end += map_range.offset;
				[non_offset_overlap, offset_overlap]
			})
			.filter(|range| !range.is_empty())
			.collect();

		let end_overlap = overlap(&range, &(last_point..range.end));
		if !end_overlap.is_empty() {
			converted.push(end_overlap);
		}
		converted
	}
}

fn overlap(a: &Range<i64>, b: &Range<i64>) -> Range<i64> {
	a.start.max(b.start)..a.end.min(b.end)
}

#[derive(PartialEq, Eq)]
struct MapRange {
	range: Range<i64>,
	offset: i64,
}

impl MapRange {
	fn from_line(line: &str) -> Self {
		let mut numbers = line.split_ascii_whitespace().map(|n| n.parse().unwrap());
		let start_to = numbers.next().unwrap();
		let start_from = numbers.next().unwrap();
		let range_size = numbers.next().unwrap();
		Self {
			range: start_from..start_from + range_size,
			offset: start_to - start_from,
		}
	}
}

impl PartialOrd for MapRange {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.range.start.cmp(&other.range.start))
	}
}

impl Ord for MapRange {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.range.start.cmp(&other.range.start)
	}
}
