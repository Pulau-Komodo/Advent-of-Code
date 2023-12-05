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
	let seed_ranges: Vec<Range<i64>> = seeds
		.chunks_exact(2)
		.map(|range| range[0]..range[0] + range[1])
		.collect();
	let maps: Vec<_> = sections.map(RangeMap::from_str).collect();
	seed_ranges
		.into_iter()
		.flatten()
		.map(|mut value| {
			for map in &maps {
				value = map.convert(value);
			}
			value
		})
		.min()
		.unwrap()
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
			if number >= range.start {
				if number < range.end {
					return number + range.offset;
				}
			} else {
				break;
			}
		}
		number
	}
}

#[derive(PartialEq, Eq)]
struct MapRange {
	start: i64,
	end: i64,
	offset: i64,
}

impl MapRange {
	fn from_line(line: &str) -> Self {
		let mut numbers = line.split_ascii_whitespace().map(|n| n.parse().unwrap());
		let start_to = numbers.next().unwrap();
		let start_from = numbers.next().unwrap();
		let range_size = numbers.next().unwrap();
		Self {
			start: start_from,
			end: start_from + range_size,
			offset: start_to - start_from,
		}
	}
}

impl PartialOrd for MapRange {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.start.cmp(&other.start))
	}
}

impl Ord for MapRange {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.start.cmp(&other.start)
	}
}
