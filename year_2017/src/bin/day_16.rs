use std::collections::VecDeque;

fn main() {
	shared::print_answers(16, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> String {
	let mut line: VecDeque<u8> = (0..16).map(|i| i as u8 + b'a').collect();
	for dance_move in input.trim().split(',').map(DanceMove::from_str) {
		dance_move.execute(&mut line);
	}
	line.into_iter().map(char::from).collect()
}

fn get_answer_2(input: &str) -> String {
	const ITERATIONS: usize = 1_000_000_000;
	let dance_moves: Vec<_> = input.trim().split(',').map(DanceMove::from_str).collect();

	let mut line: VecDeque<u8> = (0..16).map(|i| i as u8 + b'a').collect();

	let stable_loop = find_stable_loop(&dance_moves, &mut line);
	let remainder = ITERATIONS % stable_loop;
	for _ in 0..remainder {
		for dance_move in &dance_moves {
			dance_move.execute(&mut line);
		}
	}
	line.into_iter().map(char::from).collect()
}

#[derive(Debug, Clone, Copy)]
enum DanceMove {
	Spin(usize),
	Exchange(usize, usize),
	Partner(u8, u8),
}

impl DanceMove {
	fn from_str(str: &str) -> Self {
		let bytes = str.as_bytes();
		match bytes[0] {
			b's' => Self::Spin(str[1..].parse().unwrap()),
			b'x' => {
				let (a, b) = str[1..].split_once('/').unwrap();
				Self::Exchange(a.parse().unwrap(), b.parse().unwrap())
			}
			b'p' => Self::Partner(bytes[1], bytes[3]),
			_ => panic!(),
		}
	}
	fn execute(self, line: &mut VecDeque<u8>) {
		match self {
			DanceMove::Spin(amount) => line.rotate_right(amount),
			DanceMove::Exchange(a, b) => line.swap(a, b),
			DanceMove::Partner(a, b) => {
				let a = line.iter().position(|&dancer| dancer == a).unwrap();
				let b = line.iter().position(|&dancer| dancer == b).unwrap();
				line.swap(a, b);
			}
		}
	}
}

fn find_stable_loop(dance_moves: &[DanceMove], line: &mut VecDeque<u8>) -> usize {
	for iteration in 1.. {
		for dance_move in dance_moves {
			dance_move.execute(line);
		}
		if line
			.iter()
			.enumerate()
			.all(|(index, dancer)| (dancer - b'a') as usize == index)
		{
			return iteration;
		}
	}
	unreachable!()
}
