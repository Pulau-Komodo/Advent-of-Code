type Range = std::ops::RangeInclusive<u32>;

#[derive(Clone)]
struct Rule<'l> {
	name: &'l str,
	ranges: (Range, Range),
}

impl<'l> Rule<'l> {
	fn from_str(str: &'l str) -> Self {
		let (name, ranges) = str.split_once(": ").unwrap();
		let ranges = ranges.split_once(" or ").unwrap();
		let ranges = (range_from_str(ranges.0), range_from_str(ranges.1));
		Self { name, ranges }
	}
	fn includes(&self, number: &u32) -> bool {
		self.ranges.0.contains(number) || self.ranges.1.contains(number)
	}
}

fn range_from_str(str: &str) -> Range {
	let (start, end) = str.split_once('-').unwrap();
	start.parse().unwrap()..=end.parse().unwrap()
}

fn process_input(input: &str) -> (Vec<Rule>, Vec<u32>, Vec<Vec<u32>>) {
	let (rules, rest) = input.split_once("\r\n\r\nyour ticket:\r\n").unwrap();
	let (own_ticket, tickets) = rest.split_once("\r\n\r\nnearby tickets:\r\n").unwrap();
	let rules = rules.lines().map(Rule::from_str).collect();
	let own_ticket = process_ticket(own_ticket);
	let tickets = tickets.lines().map(process_ticket).collect();
	(rules, own_ticket, tickets)
}

fn process_ticket(input: &str) -> Vec<u32> {
	input
		.split(',')
		.map(str::parse::<u32>)
		.collect::<Result<_, _>>()
		.unwrap()
}

fn sum_of_invalid_values(ticket: &[u32], rules: &[Rule]) -> u32 {
	ticket
		.iter()
		.map(|number| {
			if rules.iter().any(|rule| rule.includes(number)) {
				&0
			} else {
				number
			}
		})
		.sum()
}

fn validate_ticket(ticket: &[u32], rules: &[Rule]) -> bool {
	ticket
		.iter()
		.all(|number| rules.iter().any(|rule| rule.includes(number)))
}

pub fn get_answers(input: String) -> String {
	let (rules, own_ticket, tickets) = process_input(&input);
	let error_rate: u32 = tickets
		.iter()
		.map(|ticket| sum_of_invalid_values(ticket, &rules))
		.sum();
	let valid_tickets: Vec<Vec<u32>> = tickets
		.into_iter()
		.filter(|ticket| validate_ticket(ticket, &rules))
		.collect();
	let mut unsettled_columns: std::collections::HashSet<usize> = (0..rules.len()).collect();
	let mut unsettled_rules = rules;
	let mut settled_rules: std::collections::HashMap<usize, Rule> =
		std::collections::HashMap::with_capacity(20);
	while !unsettled_rules.is_empty() {
		for (index, rule) in unsettled_rules.clone().into_iter().enumerate() {
			//println!("Examining rule: {}", rule.name);
			let mut matching_columns: Vec<usize> = Vec::new();
			for i in unsettled_columns.iter() {
				if valid_tickets
					.iter()
					.map(|ticket| ticket.get(*i).unwrap())
					.all(|number| rule.includes(number))
				{
					matching_columns.push(*i);
				}
			}
			if matching_columns.len() == 1 {
				let column = matching_columns.get(0).unwrap();
				unsettled_columns.remove(column);
				unsettled_rules.remove(index);
				//println!("Found rule: {}", rule.name);
				settled_rules.insert(*column, rule);
				break;
			} else if matching_columns.is_empty() {
				panic!();
			}
		}
	}
	let departure_columns: std::collections::HashSet<usize> = settled_rules
		.iter()
		.filter_map(|(index, rule)| {
			if rule.name.starts_with("departure ") {
				Some(*index)
			} else {
				None
			}
		})
		.collect();
	let product: u64 = own_ticket
		.iter()
		.enumerate()
		.filter_map(|(column, value)| {
			if departure_columns.contains(&column) {
				Some(*value as u64)
			} else {
				None
			}
		})
		.product();
	format!("1: {}, 2: {}", error_rate, product)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn sample_input() {
		let input = String::from("class: 1-3 or 5-7\r\nrow: 6-11 or 33-44\r\nseat: 13-40 or 45-50\r\n\r\nyour ticket:\r\n7,1,14\r\n\r\nnearby tickets:\r\n7,3,47\r\n40,4,50\r\n55,2,20\r\n38,6,12");
		assert_eq!(get_answers(input), "1: 71, 2: 1");
	}
}
