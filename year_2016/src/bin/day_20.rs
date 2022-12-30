use std::ops::RangeInclusive;

use shared::RangeInclusiveSet;

fn main() {
	shared::print_answers(20, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let range_set: RangeInclusiveSet<_> = input.lines().map(parse_range).collect();
	let mut gaps = range_set.gaps().filter(|gap| gap.end - gap.start > 1);
	gaps.next().unwrap().start + 1
}

fn get_answer_2(input: &str) -> u32 {
	let range_set: RangeInclusiveSet<_> = input.lines().map(parse_range).collect();
	range_set
		.gaps()
		.map(|gap| gap.end - gap.start - 1)
		.sum::<u32>()
		+ range_set.start().unwrap()
		+ (u32::MAX - (range_set.end().unwrap()))
}

fn parse_range(str: &str) -> RangeInclusive<u32> {
	let (start, end) = str.split_once('-').unwrap();
	let start = start.parse().unwrap();
	let end = end.parse::<u32>().unwrap();
	start..=end
}
