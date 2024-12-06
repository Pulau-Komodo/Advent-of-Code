use std::collections::HashSet;

fn main() {
	shared::print_answers(5, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let (rules, updates) = input.split_once("\n\n").unwrap();
	let rules = rules.lines().map(Rule::from_line).collect::<HashSet<_>>();

	updates
		.lines()
		.map(Update::from_line)
		.filter(|update| update.test(&rules))
		.map(|update| update.middle() as u32)
		.sum()
}

fn get_answer_2(input: &str) -> u32 {
	let (rules, updates) = input.split_once("\n\n").unwrap();
	let rules = rules.lines().map(Rule::from_line).collect::<HashSet<_>>();

	updates
		.lines()
		.map(Update::from_line)
		.filter(|update| !update.test(&rules))
		.map(|mut update| {
			update.repair(&rules);
			update
		})
		.map(|update| update.middle() as u32)
		.sum()
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Rule {
	first: u8,
	last: u8,
}

impl Rule {
	fn from_line(line: &str) -> Self {
		let (first, last) = line.split_once('|').unwrap();
		Self {
			first: first.parse().unwrap(),
			last: last.parse().unwrap(),
		}
	}
}

struct Update {
	pages: Vec<u8>,
}

impl Update {
	fn from_line(line: &str) -> Self {
		Self {
			pages: line.split(',').map(|n| n.parse().unwrap()).collect(),
		}
	}
	fn test(&self, rules: &HashSet<Rule>) -> bool {
		for first in 0..self.pages.len() - 1 {
			for second in first + 1..self.pages.len() {
				if rules.contains(&Rule {
					first: self.pages[second],
					last: self.pages[first],
				}) {
					return false;
				}
			}
		}
		true
	}
	fn middle(&self) -> u8 {
		self.pages[self.pages.len() / 2]
	}
	fn repair(&mut self, rules: &HashSet<Rule>) {
		self.pages.sort_unstable_by(|&a, &b| {
			if rules.contains(&Rule { first: a, last: b }) {
				std::cmp::Ordering::Less
			} else if rules.contains(&Rule { first: b, last: a }) {
				std::cmp::Ordering::Greater
			} else {
				std::cmp::Ordering::Equal
			}
		});
	}
}
