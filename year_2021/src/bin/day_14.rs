use std::collections::HashMap;

fn main() {
	shared::print_answers(14, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let (rules, mut polymer, last) = parse_input(input);
	for _ in 0..10 {
		polymer = insert_pairs(polymer, &rules);
	}
	let (min, max) = element_counts(polymer, last).fold((u64::MAX, u64::MIN), |(min, max), n| {
		(min.min(n), max.max(n))
	});
	max - min
}

fn get_answer_2(input: &str) -> u64 {
	let (rules, mut polymer, last) = parse_input(input);
	for _ in 0..40 {
		polymer = insert_pairs(polymer, &rules);
	}
	let (min, max) = element_counts(polymer, last).fold((u64::MAX, u64::MIN), |(min, max), n| {
		(min.min(n), max.max(n))
	});
	max - min
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pair {
	first: u8,
	second: u8,
}

type Ruleset = HashMap<Pair, (Pair, Pair)>;
type PairCount = HashMap<Pair, u64>;

fn parse_template(template: &str, size: usize) -> (PairCount, u8) {
	let mut pairs = HashMap::with_capacity(size);
	let mut bytes = template.bytes();
	let mut first = bytes.next().unwrap();
	for second in bytes {
		let pair = Pair { first, second };
		*pairs.entry(pair).or_insert(0) += 1;
		first = second;
	}
	(pairs, first)
}

fn parse_insertion_rule(rule: &str) -> (Pair, (Pair, Pair)) {
	let (pair, addition) = rule.split_once(" -> ").unwrap();
	let bytes = pair.as_bytes();
	let addition = addition.as_bytes()[0];
	let pair = Pair {
		first: bytes[0],
		second: bytes[1],
	};
	let new_pairs = (
		Pair {
			first: bytes[0],
			second: addition,
		},
		Pair {
			first: addition,
			second: bytes[1],
		},
	);
	(pair, new_pairs)
}

fn parse_input(input: &str) -> (Ruleset, PairCount, u8) {
	let (template, rules) = input.split_once("\r\n\r\n").unwrap();
	let rules: Ruleset = rules.lines().map(parse_insertion_rule).collect();
	let (template, last) = parse_template(template, rules.len());
	(rules, template, last)
}

fn insert_pairs(pairs: PairCount, rules: &Ruleset) -> PairCount {
	let mut new_pairs = HashMap::with_capacity(pairs.len());
	for (pair, count) in pairs {
		if let Some(&target) = rules.get(&pair) {
			*new_pairs.entry(target.0).or_insert(0) += count;
			*new_pairs.entry(target.1).or_insert(0) += count;
		} else {
			*new_pairs.entry(pair).or_insert(0) += count;
		}
	}
	new_pairs
}

fn element_counts(pairs: PairCount, last: u8) -> impl Iterator<Item = u64> {
	let mut elements = HashMap::new();
	for (Pair { first, second: _ }, count) in pairs {
		*elements.entry(first).or_insert(0) += count;
	}
	*elements.entry(last).or_insert(0) += 1;
	elements.into_values()
}
