fn main() {
	year_2020::print_answers(19, &[get_answer_1, get_answer_2]);
}

#[derive(PartialEq, Debug, Clone)]
enum Character {
	A,
	B,
}

#[derive(Debug)]
enum Rule {
	Reference(Vec<Vec<u8>>),
	Literal(Character),
}

type RuleSet = std::collections::HashMap<u8, Rule>;
type IndexSet = std::collections::HashSet<u8>;

fn parse_rule(rule: &str) -> (u8, Rule) {
	let (rule_id, rule) = rule.split_once(": ").unwrap();
	let rule_id: u8 = rule_id.parse().expect("Invalid rule ID");
	if rule == "\"a\"" {
		(rule_id, Rule::Literal(Character::A))
	} else if rule == "\"b\"" {
		(rule_id, Rule::Literal(Character::B))
	} else {
		let rule_reference: Vec<Vec<u8>> = rule
			.split(" | ")
			.map(|alternative| {
				alternative
					.split(' ')
					.map(str::parse)
					.collect::<Result<_, _>>()
					.expect("Invalid rule content")
			})
			.collect();
		(rule_id, Rule::Reference(rule_reference))
	}
}

fn parse_rules(rules: &str) -> RuleSet {
	rules.lines().map(parse_rule).collect()
}

fn override_rules(new_rules: &str, mut rules: RuleSet) -> RuleSet {
	for (rule_id, rule) in new_rules.lines().map(parse_rule) {
		rules.insert(rule_id, rule);
	}
	rules
}

fn parse_message(message: &str) -> Vec<Character> {
	message
		.chars()
		.map(|char| match char {
			'a' => Character::A,
			'b' => Character::B,
			_ => panic!(),
		})
		.collect()
}

fn parse_messages(messages: &str) -> Vec<Vec<Character>> {
	messages.lines().map(parse_message).collect()
}

const DEBUG: bool = false;

fn validate_message(message: &[Character], rule_id: u8, rules: &RuleSet) -> IndexSet {
	let mut output = IndexSet::new();
	match rules.get(&rule_id).unwrap() {
		Rule::Literal(character) => {
			if message.get(0) == Some(character) {
				output.insert(1);
				if DEBUG {
					println!("{:?} matched rule {} ({:?})", message, rule_id, character);
				}
			} else if DEBUG {
				println!(
					"{:?} did not match rule {} ({:?})",
					message, rule_id, character
				);
			}
		}
		Rule::Reference(rule) => {
			if DEBUG {
				println!(
					"Validating {:?} against rule {} ({:?})",
					message, rule_id, rule
				);
			}
			for alternative in rule.iter() {
				let mut last_indices = IndexSet::new();
				last_indices.insert(0);
				for item in alternative {
					let mut new_indices = IndexSet::new();
					for last_index in last_indices {
						let indices =
							validate_message(&message[last_index as usize..], *item, rules);
						for index in indices {
							new_indices.insert(last_index + index);
						}
					}
					last_indices = new_indices;
				}
				for last_index in last_indices {
					if last_index != 0 {
						output.insert(last_index); // Insert all possibilities of valid alternative
					}
				}
			}
			if DEBUG {
				if output.is_empty() {
					println!("{:?} failed against rule {}", message, rule_id);
				} else {
					println!(
						"{:?} passed against rule {} in {} way(s)",
						message,
						rule_id,
						output.len()
					);
				}
			}
		}
	}
	output
}

fn get_answer_1(input: &str) -> String {
	let (rules, messages) = input.split_once("\r\n\r\n").unwrap();
	let rules = parse_rules(rules);
	let messages = parse_messages(messages);
	let valid_count = messages
		.iter()
		.filter(|message| validate_message(message, 0, &rules).contains(&(message.len() as u8)))
		.count();
	format!("{}", valid_count)
}

fn get_answer_2(input: &str) -> String {
	let (rules, messages) = input.split_once("\r\n\r\n").unwrap();
	let rules = parse_rules(rules);
	let rules = override_rules("8: 42 | 42 8\n11: 42 31 | 42 11 31", rules);
	let messages = parse_messages(messages);
	let valid_count = messages
		.iter()
		.filter(|message| validate_message(message, 0, &rules).contains(&(message.len() as u8)))
		.count();
	format!("{}", valid_count)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn validate_one() {
		let rules = "0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"";
		let ruleset = parse_rules(rules);
		let message = parse_message("ababbb");
		assert!(validate_message(&message, 0, &ruleset).contains(&(message.len() as u8)));
	}
	#[test]
	fn invalidate_one() {
		let rules = "0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"";
		let ruleset = parse_rules(rules);
		let message = parse_message("aaabbb");
		assert!(!validate_message(&message, 0, &ruleset).contains(&(message.len() as u8)));
	}
	#[test]
	fn validate_one_long() {
		let input = year_2020::read_file(19);
		let (rules, _) = input.split_once("\r\n\r\n").unwrap();
		let rules = parse_rules(rules);
		let message = parse_message("babbaabbbabaaabbababaaaa");
		assert!(validate_message(&message, 0, &rules).contains(&(message.len() as u8)),);
	}
	#[test]
	fn sample_input() {
		let input = "0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\r\n\r\nababbb\nbababa\nabbbab\naaabbb\naaaabbb";
		assert_eq!(get_answer_1(input), "2");
	}
	#[test]
	fn sample_input_2() {
		let rules = parse_rules("42: 9 14 | 10 1\n9: 14 27 | 1 26\n10: 23 14 | 28 1\n1: \"a\"\n11: 42 31\n5: 1 14 | 15 1\n19: 14 1 | 14 14\n12: 24 14 | 19 1\n16: 15 1 | 14 14\n31: 14 17 | 1 13\n6: 14 14 | 1 14\n2: 1 24 | 14 4\n0: 8 11\n13: 14 3 | 1 12\n15: 1 | 14\n17: 14 2 | 1 7\n23: 25 1 | 22 14\n28: 16 1\n4: 1 1\n20: 14 14 | 1 15\n3: 5 14 | 16 1\n27: 1 6 | 14 18\n14: \"b\"\n21: 14 1 | 1 14\n25: 1 1 | 1 14\n22: 14 14\n8: 42\n26: 14 22 | 1 20\n18: 15 15\n7: 14 5 | 1 21\n24: 14 1");
		let messages = parse_messages("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\nbbabbbbaabaabba\nbabbbbaabbbbbabbbbbbaabaaabaaa\naaabbbbbbaaaabaababaabababbabaaabbababababaaa\nbbbbbbbaaaabbbbaaabbabaaa\nbbbababbbbaaaaaaaabbababaaababaabab\nababaaaaaabaaab\nababaaaaabbbaba\nbaabbaaaabbaaaababbaababb\nabbbbabbbbaaaababbbbbbaaaababb\naaaaabbaabaaaaababaa\naaaabbaaaabbaaa\naaaabbaabbaaaaaaabbbabbbaaabbaabaaa\nbabaaabbbaaabaababbaabababaaab\naabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba");
		let answer_1: Vec<Vec<Character>> = messages
			.clone()
			.into_iter()
			.filter(|message| validate_message(message, 0, &rules).contains(&(message.len() as u8)))
			.collect();
		assert_eq!(
			answer_1,
			parse_messages("bbabbbbaabaabba\nababaaaaaabaaab\nababaaaaabbbaba")
		);

		let rules = override_rules("8: 42 | 42 8\n11: 42 31 | 42 11 31", rules);
		let answer_2: Vec<Vec<Character>> = messages
			.into_iter()
			.filter(|message| validate_message(message, 0, &rules).contains(&(message.len() as u8)))
			.collect();
		assert_eq!(answer_2, parse_messages("bbabbbbaabaabba\nbabbbbaabbbbbabbbbbbaabaaabaaa\naaabbbbbbaaaabaababaabababbabaaabbababababaaa\nbbbbbbbaaaabbbbaaabbabaaa\nbbbababbbbaaaaaaaabbababaaababaabab\nababaaaaaabaaab\nababaaaaabbbaba\nbaabbaaaabbaaaababbaababb\nabbbbabbbbaaaababbbbbbaaaababb\naaaaabbaabaaaaababaa\naaaabbaabbaaaaaaabbbabbbaaabbaabaaa\naabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"));
	}
	#[test]
	fn sample_input_2_single() {
		let rules = parse_rules("42: 9 14 | 10 1\n9: 14 27 | 1 26\n10: 23 14 | 28 1\n1: \"a\"\n11: 42 31\n5: 1 14 | 15 1\n19: 14 1 | 14 14\n12: 24 14 | 19 1\n16: 15 1 | 14 14\n31: 14 17 | 1 13\n6: 14 14 | 1 14\n2: 1 24 | 14 4\n0: 8 11\n13: 14 3 | 1 12\n15: 1 | 14\n17: 14 2 | 1 7\n23: 25 1 | 22 14\n28: 16 1\n4: 1 1\n20: 14 14 | 1 15\n3: 5 14 | 16 1\n27: 1 6 | 14 18\n14: \"b\"\n21: 14 1 | 1 14\n25: 1 1 | 1 14\n22: 14 14\n8: 42\n26: 14 22 | 1 20\n18: 15 15\n7: 14 5 | 1 21\n24: 14 1");
		let rules = override_rules("8: 42 | 42 8\n11: 42 31 | 42 11 31", rules);
		let message = parse_message("babbbbaabbbbbabbbbbbaabaaabaaa");
		assert!(validate_message(&message, 0, &rules).contains(&(message.len() as u8)));
	}
}
