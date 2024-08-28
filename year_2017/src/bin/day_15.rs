fn main() {
	shared::print_answers(15, &[get_answer_1, get_answer_2]);
}

const FACTORS: (u64, u64) = (16807, 48271);
const MODULO: u64 = 2147483647;

fn get_answer_1(input: &str) -> u32 {
	let (mut a, mut b) = get_starting_values(input);
	let mut match_count = 0;
	for _ in 0..40_000_000 {
		a = a * FACTORS.0 % MODULO;
		b = b * FACTORS.1 % MODULO;
		if a & 0b1111_1111_1111_1111 == b & 0b1111_1111_1111_1111 {
			match_count += 1;
		}
	}
	match_count
}

fn get_answer_2(input: &str) -> u32 {
	let (mut a, mut b) = get_starting_values(input);
	let mut match_count = 0;
	for _ in 0..5_000_000 {
		loop {
			a = a * FACTORS.0 % MODULO;
			if a % 4 == 0 {
				break;
			}
		}
		loop {
			b = b * FACTORS.1 % MODULO;
			if b % 8 == 0 {
				break;
			}
		}
		if a & 0b1111_1111_1111_1111 == b & 0b1111_1111_1111_1111 {
			match_count += 1;
		}
	}
	match_count
}

fn get_starting_values(input: &str) -> (u64, u64) {
	let mut numbers = input.lines().map(|line| {
		let (_, num) = line.rsplit_once(' ').unwrap();
		num.parse().unwrap()
	});
	let a = numbers.next().unwrap();
	let b = numbers.next().unwrap();
	(a, b)
}
