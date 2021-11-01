fn main() {
	shared::print_answers(15, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> i64 {
	let candies: Vec<Candy> = input.lines().map(Candy::from_str).collect();
	find_highest_score(&candies, None)
}

fn get_answer_2(input: &str) -> i64 {
	let candies: Vec<Candy> = input.lines().map(Candy::from_str).collect();
	find_highest_score(&candies, Some(500))
}

struct Candy {
	capacity: i8,
	durability: i8,
	flavour: i8,
	texture: i8,
	calories: i8,
}

impl Candy {
	fn from_str(str: &str) -> Self {
		let mut iter = str.split(' ');
		let capacity = iter
			.nth(2)
			.and_then(|n| n.strip_suffix(','))
			.and_then(|n| n.parse().ok())
			.unwrap();
		let durability = iter
			.nth(1)
			.and_then(|n| n.strip_suffix(','))
			.and_then(|n| n.parse().ok())
			.unwrap();
		let flavour = iter
			.nth(1)
			.and_then(|n| n.strip_suffix(','))
			.and_then(|n| n.parse().ok())
			.unwrap();
		let texture = iter
			.nth(1)
			.and_then(|n| n.strip_suffix(','))
			.and_then(|n| n.parse().ok())
			.unwrap();
		let calories = iter.nth(1).and_then(|n| n.parse().ok()).unwrap();
		Self {
			capacity,
			durability,
			flavour,
			texture,
			calories,
		}
	}
}

// I got the below off Google, but I adjusted it to just yield the counts of each item rather than
// actually bothering to make a Vec with all the items in it.

// Iterator for the combinations of `arr` with `k` elements with repetitions.
// Yields the combinations in lexicographical order.
struct CombinationsWithRepetitions {
	// length of the combinations
	k: u32,
	// current counts of each object that represent the next combination
	counts: Vec<u32>,
	// whether there are any combinations left
	remaining: bool,
}

impl CombinationsWithRepetitions {
	fn new(items: usize, target: u32) -> CombinationsWithRepetitions {
		let mut counts = vec![0; items];
		counts[items - 1] = target;
		CombinationsWithRepetitions {
			k: target,
			counts,
			remaining: true,
		}
	}
}

impl Iterator for CombinationsWithRepetitions {
	type Item = Vec<u32>;

	fn next(&mut self) -> Option<Vec<u32>> {
		if !self.remaining {
			return None;
		}
		// this is lexicographically largest, and thus the last combination
		if self.counts[0] == self.k {
			self.remaining = false;
		} else {
			let n = self.counts.len();
			for i in (1..n).rev() {
				if self.counts[i] > 0 {
					let original_value = self.counts[i];
					self.counts[i - 1] += 1;
					for j in i..(n - 1) {
						self.counts[j] = 0;
					}
					self.counts[n - 1] = original_value - 1;
					break;
				}
			}
		}
		Some(self.counts.clone())
	}
}

fn find_highest_score(candies: &[Candy], calorie_target: Option<i64>) -> i64 {
	let mut highest_score = 0;
	for counts in CombinationsWithRepetitions::new(candies.len(), 100) {
		let (capacity, durability, flavour, texture, calories) = counts.iter().zip(candies.iter()).fold(
			(0, 0, 0, 0, 0),
			|(capacity, durability, flavour, texture, calories), (&count, candy)| {
				(
					capacity + candy.capacity as i64 * count as i64,
					durability + candy.durability as i64 * count as i64,
					flavour + candy.flavour as i64 * count as i64,
					texture + candy.texture as i64 * count as i64,
					calories + candy.calories as i64 * count as i64,
				)
			},
		);
		if let Some(target) = calorie_target {
			if calories != target {
				continue;
			}
		}
		let score = capacity.max(0) * durability.max(0) * flavour.max(0) * texture.max(0);
		highest_score = highest_score.max(score);
	}
	highest_score
}