fn main() {
	shared::print_answers(16, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let aunts: Vec<Aunt> = input.lines().map(Aunt::from_str).collect();
	aunts.iter().position(Aunt::is_match).unwrap() + 1
}

fn get_answer_2(input: &str) -> usize {
	let aunts: Vec<Aunt> = input.lines().map(Aunt::from_str).collect();
	aunts.iter().position(Aunt::is_match_2).unwrap() + 1
}

struct Aunt<'l> {
	properties: std::collections::HashMap<&'l str, u8>,
}

impl<'l> Aunt<'l> {
	fn from_str(str: &'l str) -> Self {
		let (_number, properties) = str.split_once(": ").unwrap();
		let properties = properties
			.split(", ")
			.map(|property| {
				let (name, value) = property.split_once(": ").unwrap();
				let value = value.parse().unwrap();
				(name, value)
			})
			.collect();
		Self { properties }
	}
	fn is_match(&self) -> bool {
		matches!(self.properties.get("children"), Some(3) | None)
			&& matches!(self.properties.get("cats"), Some(7) | None)
			&& matches!(self.properties.get("samoyeds"), Some(2) | None)
			&& matches!(self.properties.get("pomeranians"), Some(3) | None)
			&& matches!(self.properties.get("akitas"), Some(0) | None)
			&& matches!(self.properties.get("vizslas"), Some(0) | None)
			&& matches!(self.properties.get("goldfish"), Some(5) | None)
			&& matches!(self.properties.get("trees"), Some(3) | None)
			&& matches!(self.properties.get("cars"), Some(2) | None)
			&& matches!(self.properties.get("perfumes"), Some(1) | None)
	}
	fn is_match_2(&self) -> bool {
		matches!(self.properties.get("children"), Some(3) | None)
			&& matches!(self.properties.get("cats"), Some(8..) | None)
			&& matches!(self.properties.get("samoyeds"), Some(2) | None)
			&& matches!(self.properties.get("pomeranians"), Some(0..=2) | None)
			&& matches!(self.properties.get("akitas"), Some(0) | None)
			&& matches!(self.properties.get("vizslas"), Some(0) | None)
			&& matches!(self.properties.get("goldfish"), Some(0..=4) | None)
			&& matches!(self.properties.get("trees"), Some(4..) | None)
			&& matches!(self.properties.get("cars"), Some(2) | None)
			&& matches!(self.properties.get("perfumes"), Some(1) | None)
	}
}
