use std::{cmp::Ordering, collections::HashMap};

fn main() {
	shared::print_answers(19, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let (workflows, parts) = input.split_once("\n\n").unwrap();
	let workflows = make_workflow_map(workflows);

	let mut score = 0;
	for part in parts.lines().map(Part::from_line) {
		let mut workflow = workflows.get("in").unwrap();
		loop {
			let outcome = workflow.apply(&part);
			match outcome {
				Outcome::GoTo(label) => workflow = workflows.get(label).unwrap(),
				Outcome::Accept => {
					score += part.score();
					break;
				}
				Outcome::Reject => break,
			}
		}
	}
	score as u64
}

fn get_answer_2(input: &str) -> u64 {
	let (workflows, _parts) = input.split_once("\n\n").unwrap();
	let workflows = make_workflow_map(workflows);

	let mut frontier = vec![("in", RangePart::full())];
	let mut new_frontier = Vec::new();
	let mut accepted = Vec::new();
	loop {
		for (label, part) in frontier.drain(..) {
			let workflow = workflows.get(label).unwrap();
			new_frontier.extend(workflow.apply_range(part).into_iter().filter_map(
				|(outcome, part)| match outcome {
					Outcome::GoTo(label) => Some((label, part)),
					Outcome::Accept => {
						accepted.push(part);
						None
					}
					Outcome::Reject => None,
				},
			));
		}
		if new_frontier.is_empty() {
			break;
		} else {
			std::mem::swap(&mut frontier, &mut new_frontier);
		}
	}
	accepted.iter().map(RangePart::possibilities).sum()
}

#[derive(Clone, Copy)]
enum Property {
	Extreme,
	Musical,
	Aerodynamic,
	Shiny,
}

impl Property {
	fn from_byte(byte: u8) -> Self {
		match byte {
			b'x' => Self::Extreme,
			b'm' => Self::Musical,
			b'a' => Self::Aerodynamic,
			b's' => Self::Shiny,
			_ => panic!("Unexpected property byte"),
		}
	}
}

fn ordering_from_byte(byte: u8) -> Ordering {
	match byte {
		b'>' => Ordering::Greater,
		b'<' => Ordering::Less,
		_ => panic!("Unexpected ordering byte"),
	}
}

struct Test {
	property: Property,
	ordering: Ordering,
	threshold: u32,
}

impl Test {
	fn from_str(str: &str) -> Self {
		let bytes = str.as_bytes();
		let property = Property::from_byte(bytes[0]);
		let ordering = ordering_from_byte(bytes[1]);
		let threshold = str[2..].parse().unwrap();
		Self {
			property,
			ordering,
			threshold,
		}
	}
	fn apply(&self, part: &Part) -> bool {
		let value = part.get_property(self.property);
		value.cmp(&self.threshold) == self.ordering
	}
	/// Passing and failing parts, respectively. Parts are `None` if the range was empty.
	fn apply_range(&self, part: RangePart) -> (Option<RangePart>, Option<RangePart>) {
		let mut passing = part;
		let mut failing = part;
		let passing_range = passing.get_property_mut(self.property);
		let failing_range = failing.get_property_mut(self.property);
		match self.ordering {
			Ordering::Greater => {
				passing_range.start = passing_range.start.max(self.threshold + 1);
				failing_range.end = failing_range.end.min(self.threshold)
			}
			Ordering::Less => {
				passing_range.end = passing_range.end.min(self.threshold - 1);
				failing_range.start = failing_range.start.max(self.threshold)
			}
			_ => unreachable!(),
		}
		(
			(!passing_range.is_empty()).then_some(passing),
			(!failing_range.is_empty()).then_some(failing),
		)
	}
}

#[derive(Clone, Copy)]
enum Outcome<'l> {
	GoTo(&'l str),
	Accept,
	Reject,
}

impl<'l> Outcome<'l> {
	fn from_str(str: &'l str) -> Self {
		match str {
			"A" => Self::Accept,
			"R" => Self::Reject,
			label => Self::GoTo(label),
		}
	}
}

struct Workflow<'l> {
	rules: Vec<(Test, Outcome<'l>)>,
	otherwise: Outcome<'l>,
}

impl<'l> Workflow<'l> {
	fn from_str(str: &'l str) -> Self {
		let mut rules = Vec::new();
		for rule in str.split(',') {
			if let Some((test, outcome)) = rule.split_once(':') {
				rules.push((Test::from_str(test), Outcome::from_str(outcome)));
			} else {
				return Self {
					rules,
					otherwise: Outcome::from_str(rule),
				};
			}
		}
		panic!("Found no final outcome")
	}
	fn apply(&'l self, part: &Part) -> Outcome<'l> {
		for (test, outcome) in &self.rules {
			if test.apply(part) {
				return *outcome;
			}
		}
		self.otherwise
	}
	fn apply_range(&'l self, mut part: RangePart) -> Vec<(Outcome, RangePart)> {
		let mut output = Vec::new();
		for (test, outcome) in &self.rules {
			let (passing, failing) = test.apply_range(part);
			if let Some(passing) = passing {
				output.push((*outcome, passing));
			}
			if let Some(failing) = failing {
				part = failing;
			} else {
				return output;
			}
		}
		output.push((self.otherwise, part));
		output
	}
}

fn make_workflow_map(workflows: &str) -> HashMap<&str, Workflow> {
	workflows
		.lines()
		.map(|line| {
			let (name, workflow) = line.split_once('{').unwrap();
			let workflow = Workflow::from_str(&workflow[0..workflow.len() - 1]);
			(name, workflow)
		})
		.collect()
}

struct Part {
	extreme: u32,
	musical: u32,
	aerodynamic: u32,
	shiny: u32,
}

impl Part {
	fn from_line(line: &str) -> Self {
		let mut parts = line[1..line.len() - 1].split(',').map(|part| &part[2..]);
		Self {
			extreme: parts.next().unwrap().parse().unwrap(),
			musical: parts.next().unwrap().parse().unwrap(),
			aerodynamic: parts.next().unwrap().parse().unwrap(),
			shiny: parts.next().unwrap().parse().unwrap(),
		}
	}
	fn score(&self) -> u32 {
		self.extreme + self.musical + self.aerodynamic + self.shiny
	}
	fn get_property(&self, property: Property) -> u32 {
		match property {
			Property::Extreme => self.extreme,
			Property::Musical => self.musical,
			Property::Aerodynamic => self.aerodynamic,
			Property::Shiny => self.shiny,
		}
	}
}

#[derive(Debug, Clone, Copy)]
struct Range {
	start: u32,
	end: u32,
}

impl Range {
	fn size(&self) -> u32 {
		self.end - self.start + 1
	}
	fn is_empty(&self) -> bool {
		self.end < self.start
	}
}

#[derive(Debug, Clone, Copy)]
struct RangePart {
	extreme: Range,
	musical: Range,
	aerodynamic: Range,
	shiny: Range,
}

impl RangePart {
	fn full() -> Self {
		let full_range = Range {
			start: 1,
			end: 4000,
		};
		Self {
			extreme: full_range,
			musical: full_range,
			aerodynamic: full_range,
			shiny: full_range,
		}
	}
	fn get_property_mut(&mut self, property: Property) -> &mut Range {
		match property {
			Property::Extreme => &mut self.extreme,
			Property::Musical => &mut self.musical,
			Property::Aerodynamic => &mut self.aerodynamic,
			Property::Shiny => &mut self.shiny,
		}
	}
	fn possibilities(&self) -> u64 {
		[self.extreme, self.musical, self.aerodynamic, self.shiny]
			.into_iter()
			.map(|range| range.size() as u64)
			.product::<u64>()
	}
}
