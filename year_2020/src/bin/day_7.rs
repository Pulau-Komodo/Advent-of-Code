fn main() {
	shared::print_answers(7, &[part_a, part_b]);
}

fn part_a(input: &str) -> String {
	let bags = input
		.lines()
		.map(parse_bag_rule)
		.collect::<std::collections::HashMap<&str, Vec<&str>>>();
	let count = count_bags_containing_bag("shiny gold", bags);
	format!("{}", count)
}

fn parse_bag_rule(rule: &str) -> (&str, Vec<&str>) {
	let (bag, rest) = rule.split_once(" bags contain ").unwrap();
	if rest == "no other bags." {
		return (bag, Vec::new());
	}
	let mut contents: Vec<&str> = Vec::new();
	for item in rest.split(", ") {
		let (_number, rest) = item.split_once(" ").unwrap();
		let (bag, _) = rest.split_once(" bag").unwrap();
		contents.push(bag);
	}
	(bag, contents)
}

fn count_bags_containing_bag(
	bag: &str,
	mut bags: std::collections::HashMap<&str, Vec<&str>>,
) -> u32 {
	let mut count = 0;
	let mut previous_bags = vec![bag];
	loop {
		let mut containing_bags = Vec::new();
		for (bag, _) in bags.iter().filter(|(_, contents)| {
			previous_bags
				.iter()
				.any(|prev_bag| contents.contains(prev_bag))
		}) {
			containing_bags.push(*bag)
		}
		if containing_bags.is_empty() {
			break;
		}
		count += containing_bags.len() as u32;
		for bag in &containing_bags {
			bags.remove(*bag);
		}
		previous_bags = containing_bags;
	}
	count
}

fn part_b(input: &str) -> String {
	let bags = input
		.lines()
		.map(parse_bag_rule_b)
		.collect::<std::collections::HashMap<&str, Vec<(u32, &str)>>>();
	let count = count_bags_in_bag("shiny gold", bags);
	format!("{}", count)
}

fn parse_bag_rule_b(rule: &str) -> (&str, Vec<(u32, &str)>) {
	let (bag, rest) = rule.split_once(" bags contain ").unwrap();
	if rest == "no other bags." {
		return (bag, Vec::new());
	}
	let mut contents: Vec<(u32, &str)> = Vec::new();
	for item in rest.split(", ") {
		let (number, rest) = item.split_once(" ").unwrap();
		let (bag, _) = rest.split_once(" bag").unwrap();
		contents.push((number.parse::<u32>().unwrap(), bag));
	}
	(bag, contents)
}

fn count_bags_in_bag(bag: &str, bags: std::collections::HashMap<&str, Vec<(u32, &str)>>) -> u32 {
	let mut count = 0;
	let mut previous_bags = vec![(1, bag)];
	loop {
		let mut contained_bags = Vec::new();
		for (amount, bag) in &previous_bags {
			if let Some(bag_contents) = bags.get(bag) {
				for (contained_amount, contained_bag) in bag_contents {
					contained_bags.push((amount * contained_amount, *contained_bag))
				}
			}
		}
		if contained_bags.is_empty() {
			break;
		}
		count += contained_bags.iter().map(|(amount, _)| amount).sum::<u32>();
		previous_bags = contained_bags;
	}
	count
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn bag_rule() {
		assert_eq!(parse_bag_rule("light white bags contain 1 vibrant fuchsia bag, 3 posh tomato bags, 4 muted chartreuse bags."), ("light white", vec!["vibrant fuchsia", "posh tomato", "muted chartreuse"]));
	}
}
