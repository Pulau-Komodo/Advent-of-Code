use shared::{Offset, Point};

fn main() {
	shared::print_answers(13, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> i64 {
	input
		.split("\n\n")
		.map(ClawMachine::from_str)
		.filter_map(|machine| machine.cost_to_win(Some(100)))
		.sum()
}

fn get_answer_2(input: &str) -> i64 {
	input
		.split("\n\n")
		.map(ClawMachine::from_str)
		.map(|mut machine| {
			machine.add_ten_trillion();
			machine
		})
		.filter_map(|machine| machine.cost_to_win(None))
		.sum()
}

struct ClawMachine {
	button_a: Offset<i64>,
	button_b: Offset<i64>,
	prize: Point<i64>,
}

impl ClawMachine {
	fn from_str(str: &str) -> Self {
		let mut lines = str.lines();
		let ab = [lines.next().unwrap(), lines.next().unwrap()];
		let [a, b] = ab.map(|line| {
			let offsets = &line[12..];
			let (x, y) = offsets.split_once(", Y+").unwrap();
			let [x, y] = [x, y].map(|num| num.parse().unwrap());
			Offset::new(x, y)
		});
		let prize = lines.next().unwrap();
		let (x, y) = prize[9..].split_once(", Y=").unwrap();
		let [x, y] = [x, y].map(|num| num.parse().unwrap());
		Self {
			button_a: a,
			button_b: b,
			prize: Point::new(x, y),
		}
	}
	// Does not handle the case that the slopes of the two buttons are the same. Doesn't appear to come up.
	fn cost_to_win(&self, limit: Option<i64>) -> Option<i64> {
		let denom = self.button_a.x * self.button_b.y - self.button_a.y * self.button_b.x;
		if denom == 0 {
			return None;
		}
		let a_part = self.prize.x * self.button_b.y - self.prize.y * self.button_b.x;
		let b_part = self.prize.y * self.button_a.x - self.prize.x * self.button_a.y;
		let a = a_part / denom;
		let b = b_part / denom;
		if a_part % denom != 0
			|| b_part % denom != 0
			|| a > limit.unwrap_or(i64::MAX)
			|| b > limit.unwrap_or(i64::MAX)
		{
			None
		} else {
			Some(a * 3 + b)
		}
	}
	fn add_ten_trillion(&mut self) {
		self.prize += Offset::new(10_000_000_000_000, 10_000_000_000_000);
	}
}
