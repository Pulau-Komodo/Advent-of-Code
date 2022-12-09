use std::collections::HashSet;

fn main() {
	shared::print_answers(9, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let mut state = State::<2>::new();
	for motion in input.lines().map(Motion::from_str) {
		state.move_head(&motion);
	}
	state.visited.len()
}

fn get_answer_2(input: &str) -> usize {
	let mut state = State::<10>::new();
	for motion in input.lines().map(Motion::from_str) {
		state.move_head(&motion);
	}
	state.visited.len()
}

enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl Direction {
	fn from_str(str: &str) -> Self {
		match str {
			"U" => Self::Up,
			"R" => Self::Right,
			"D" => Self::Down,
			"L" => Self::Left,
			_ => panic!("Invalid direction"),
		}
	}
	fn as_offset(&self) -> (i32, i32) {
		match self {
			Self::Up => (0, -1),
			Self::Right => (1, 0),
			Self::Down => (0, 1),
			Self::Left => (-1, 0),
		}
	}
}

struct Motion {
	direction: Direction,
	amount: i32,
}

impl Motion {
	fn from_str(str: &str) -> Self {
		let (direction, amount) = str.split_once(' ').unwrap();
		let direction = Direction::from_str(direction);
		let amount = amount.parse().unwrap();
		Self { direction, amount }
	}
}

struct State<const T: usize> {
	knots: [(i32, i32); T],
	visited: HashSet<(i32, i32)>,
}

impl<const T: usize> State<T> {
	fn new() -> Self {
		let knots = [(0, 0); T];
		let visited = HashSet::new();
		Self { knots, visited }
	}
	fn move_head(&mut self, motion: &Motion) {
		let offset = motion.direction.as_offset();
		for _ in 0..motion.amount {
			self.knots[0].0 += offset.0;
			self.knots[0].1 += offset.1;
			self.catch_up_knots();
		}
	}
	fn catch_up_knots(&mut self) {
		for i in 1..T {
			let difference = (
				self.knots[i - 1].0 - self.knots[i].0,
				self.knots[i - 1].1 - self.knots[i].1,
			);
			if difference.0.abs() >= 2 || difference.1.abs() >= 2 {
				self.knots[i].0 += difference.0.signum();
				self.knots[i].1 += difference.1.signum();
			}
		}
		self.visited.insert(self.knots[T - 1]);
	}
}
