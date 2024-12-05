fn main() {
	shared::print_answers(5, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let (rules, updates) = input.split_once("\n\n").unwrap();
	let rules = rules.lines().map(Rule::from_line).collect::<Vec<_>>();

	updates
		.lines()
		.map(Update::from_line)
		.filter(|update| rules.iter().all(|rule| rule.test(update)))
		.map(|update| update.middle() as u32)
		.sum()
}

fn get_answer_2(input: &str) -> u32 {
	let (rules, updates) = input.split_once("\n\n").unwrap();
	let rules = rules.lines().map(Rule::from_line).collect::<Vec<_>>();

	updates
		.lines()
		.map(Update::from_line)
		.filter(|update| !rules.iter().all(|rule| rule.test(update)))
		.map(|mut update| {
			update.repair(&rules);
			update
		})
		.map(|update| update.middle() as u32)
		.sum()
}

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
	fn test(&self, update: &Update) -> bool {
		let mut last_found = false;
		for &page in &update.pages {
			if page == self.last {
				last_found = true;
			} else if page == self.first {
				return !last_found;
			}
		}
		true
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
	fn middle(&self) -> u8 {
		self.pages[self.pages.len() / 2]
	}
	fn repair(&mut self, rules: &[Rule]) {
		self.pages.sort_unstable_by(|&a, &b| {
			let Some(rule) = rules.iter().find(|rule| {
				rule.first == a && rule.last == b || rule.first == b && rule.last == a
			}) else {
				return std::cmp::Ordering::Equal;
			};
			if rule.first == a {
				std::cmp::Ordering::Less
			} else {
				std::cmp::Ordering::Greater
			}
		});
	}
}
