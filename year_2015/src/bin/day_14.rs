fn main() {
	shared::print_answers(14, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u16 {
	input
		.lines()
		.map(Reindeer::from_str)
		.map(|reindeer| reindeer.distance_after(2503))
		.max()
		.unwrap()
}

fn get_answer_2(input: &str) -> u16 {
	let mut reindeers: Vec<Reindeer> = input.lines().map(Reindeer::from_str).collect();
	for _ in 1..=2503 {
		for reindeer in reindeers.iter_mut() {
			reindeer.progress();
		}
		let max = reindeers
			.iter()
			.map(|reindeer| reindeer.distance)
			.max()
			.unwrap();
		for reindeer in reindeers.iter_mut() {
			if reindeer.distance == max {
				reindeer.award_point();
			}
		}
	}
	reindeers
		.iter()
		.map(|reindeer| reindeer.score)
		.max()
		.unwrap()
}

struct Reindeer {
	speed: u16,
	flight_duration: u16,
	cycle_duration: u16,
	point_in_cycle: u16,
	distance: u16,
	score: u16,
}

impl Reindeer {
	fn from_str(str: &str) -> Self {
		let mut iter = str.split(' ');
		let speed = iter.nth(3).and_then(|n| n.parse().ok()).unwrap();
		let flight_duration = iter.nth(2).and_then(|n| n.parse().ok()).unwrap();
		let rest_duration: u16 = iter.nth(6).and_then(|n| n.parse().ok()).unwrap();
		Self {
			speed,
			flight_duration,
			cycle_duration: flight_duration + rest_duration,
			point_in_cycle: 0,
			distance: 0,
			score: 0,
		}
	}
	fn distance_after(&self, seconds: u16) -> u16 {
		let cycles = seconds / self.cycle_duration;
		let remaining_flight_time = self.flight_duration.min(seconds % self.cycle_duration);
		let flight_time = cycles * self.flight_duration + remaining_flight_time;
		flight_time * self.speed
	}
	fn progress(&mut self) {
		if self.point_in_cycle < self.flight_duration {
			self.distance += self.speed;
		}
		self.point_in_cycle = (self.point_in_cycle + 1) % self.cycle_duration;
	}
	fn award_point(&mut self) {
		self.score += 1;
	}
}
