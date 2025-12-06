use shared::RangeInclusiveSet;

fn main() {
	shared::print_answers(5, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let (ranges, ids) = parse(input);
	ids.into_iter().filter(|id| ranges.contains(id)).count()
}

fn get_answer_2(input: &str) -> usize {
	let (ranges, _ids) = parse(input);
	ranges.len_sum() as usize + ranges.count()
}

fn parse(input: &str) -> (RangeInclusiveSet<u64>, Vec<u64>) {
	let (ranges, ids) = input.split_once("\n\n").unwrap();
	let mut ranges = RangeInclusiveSet::from_iter(ranges.lines().map(|line| {
		let (start, end) = line.split_once('-').unwrap();
		start.parse().unwrap()..=end.parse().unwrap()
	}));
	ranges.consolidate();
	let ids = ids.lines().map(|line| line.parse().unwrap()).collect();
	(ranges, ids)
}
