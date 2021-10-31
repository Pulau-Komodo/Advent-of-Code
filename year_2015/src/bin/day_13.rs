use std::iter::FromIterator;

fn main() {
	shared::print_answers(13, &[get_answer]);
}

fn get_answer(input: &str) -> String {
	let now = std::time::Instant::now();
	let mut data = SeatingData::from_str(input);
	let happinesses = data.all_happinesses();
	let answer_1 = *happinesses.iter().max().unwrap();
	let time_1 = now.elapsed().as_micros();
	data.seating.push("me");
	let happinesses = data.all_happinesses();
	let answer_2 = *happinesses.iter().max().unwrap();
	format!("1: {} ({} μ), 2: {}", answer_1, time_1, answer_2)
}

struct SeatingData<'l> {
	seating: Vec<&'l str>,
	happiness: std::collections::HashMap<(&'l str, &'l str), i32>,
}

impl<'l> SeatingData<'l> {
	fn from_str(str: &'l str) -> Self {
		let mut set = std::collections::HashSet::new();
		let map = str
			.lines()
			.map(|line| {
				let mut words = line.split(' ');
				let person_1 = words.next().unwrap();
				set.insert(person_1);
				let negative = matches!(words.nth(1), Some("lose"));
				let mut amount = words.next().and_then(|n| n.parse().ok()).unwrap();
				if negative {
					amount *= -1;
				}
				let person_2 = words
					.nth(6)
					.and_then(|name| name.strip_suffix('.'))
					.unwrap();
				((person_1, person_2), amount)
			})
			.collect();
		Self {
			seating: Vec::from_iter(set),
			happiness: map,
		}
	}
	fn get_happiness(&self, first: &str, second: &str) -> i32 {
		if first == "me" || second == "me" {
			return 0;
		}
		self.happiness
			.get(&(first, second))
			.and_then(|n| self.happiness.get(&(second, first)).map(|h| h + n))
			.unwrap()
	}
	fn seating_happiness(&self) -> i32 {
		self.seating
			.windows(2)
			.map(|people| self.get_happiness(people[0], people[1]))
			.sum::<i32>()
			+ self.get_happiness(self.seating[0], self.seating.last().unwrap())
	}
	fn all_happinesses(&mut self) -> Vec<i32> {
		let mut happinesses = Vec::with_capacity(self.seating.len());
		let mut c = vec![0; self.seating.len()];
		let mut i = 0;
		happinesses.push(self.seating_happiness());
		while i < self.seating.len() {
			if c[i] < i {
				if i % 2 == 0 {
					self.seating.swap(0, i);
				} else {
					self.seating.swap(c[i], i);
				}
				happinesses.push(self.seating_happiness());
				c[i] += 1;
				i = 0;
			} else {
				c[i] = 0;
				i += 1;
			}
		}
		happinesses
	}
}
