use std::ops::RangeInclusive;

fn main() {
	shared::print_answers(21, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> String {
	let mut password = Vec::from("abcdefgh".as_bytes());
	for operation in input.lines().map(Operation::from_str) {
		operation.apply(&mut password);
	}
	String::from_utf8(password).unwrap()
}

fn get_answer_2(input: &str) -> String {
	let mut password = Vec::from("fbgdceah".as_bytes());
	for operation in input.lines().rev().map(Operation::from_str) {
		operation.reverse(&mut password);
	}
	String::from_utf8(password).unwrap()
}

enum Operation {
	SwapPos(usize, usize),
	SwapChar(u8, u8),
	RotateLeft(usize),
	RotateRight(usize),
	RotateBasedOnChar(u8),
	ReverseRange(RangeInclusive<usize>),
	Move { from: usize, to: usize },
}

impl Operation {
	fn from_str(str: &str) -> Self {
		let mut parts = str.split(' ');
		match (parts.next().unwrap(), parts.next().unwrap()) {
			("swap", "position") => {
				let a = parts.next().unwrap().parse().unwrap();
				let b = parts.nth(2).unwrap().parse().unwrap();
				Self::SwapPos(a, b)
			}
			("swap", "letter") => {
				let a = parts.next().unwrap().as_bytes()[0];
				let b = parts.nth(2).unwrap().as_bytes()[0];
				Self::SwapChar(a, b)
			}
			("rotate", "left") => {
				let steps = parts.next().unwrap().parse().unwrap();
				Self::RotateLeft(steps)
			}
			("rotate", "right") => {
				let steps = parts.next().unwrap().parse().unwrap();
				Self::RotateRight(steps)
			}
			("rotate", "based") => {
				let char = parts.nth(4).unwrap().as_bytes()[0];
				Self::RotateBasedOnChar(char)
			}
			("reverse", "positions") => {
				let a = parts.next().unwrap().parse().unwrap();
				let b = parts.nth(1).unwrap().parse().unwrap();
				Self::ReverseRange(a..=b)
			}
			("move", "position") => {
				let from = parts.next().unwrap().parse().unwrap();
				let to = parts.nth(2).unwrap().parse().unwrap();
				Self::Move { from, to }
			}
			_ => panic!("Invalid input."),
		}
	}
	fn apply(self, password: &mut Vec<u8>) {
		match self {
			Self::SwapPos(a, b) => {
				password.swap(a, b);
			}
			Self::SwapChar(a, b) => {
				let pos_a = password.iter().position(|&byte| byte == a).unwrap();
				let pos_b = password.iter().position(|&byte| byte == b).unwrap();
				Self::SwapPos(pos_a, pos_b).apply(password);
			}
			Self::RotateLeft(amount) => {
				for _ in 0..amount {
					let byte = password.remove(0);
					password.push(byte);
				}
			}
			Self::RotateRight(amount) => {
				for _ in 0..amount {
					let byte = password.pop().unwrap();
					password.insert(0, byte);
				}
			}
			Self::RotateBasedOnChar(char) => {
				let pos = password.iter().position(|&byte| byte == char).unwrap();
				let amount = if pos >= 4 { pos + 2 } else { pos + 1 };
				Self::RotateRight(amount).apply(password);
			}
			Self::ReverseRange(range) => {
				let mut start = *range.start();
				let mut end = *range.end();
				while start < end {
					Self::SwapPos(start, end).apply(password);
					start += 1;
					end -= 1;
				}
			}
			Self::Move { from, to } => {
				let byte = password.remove(from);
				password.insert(to, byte);
			}
		}
	}
	fn reverse(self, password: &mut Vec<u8>) {
		match self {
			Self::SwapPos(_, _) | Self::SwapChar(_, _) | Self::ReverseRange(_) => {
				self.apply(password);
			}
			Self::RotateLeft(amount) => {
				Self::RotateRight(amount).apply(password);
			}
			Self::RotateRight(amount) => {
				Self::RotateLeft(amount).apply(password);
			}
			Self::RotateBasedOnChar(char) => {
				let pos = password.iter().position(|&byte| byte == char).unwrap();
				let amount = if pos == 0 {
					1
				} else if pos % 2 == 0 {
					5 + pos / 2
				} else {
					(pos - 1) / 2 + 1
				} % password.len();
				Self::RotateLeft(amount).apply(password);
			}
			Self::Move { from, to } => {
				let byte = password.remove(to);
				password.insert(from, byte);
			}
		}
	}
}
