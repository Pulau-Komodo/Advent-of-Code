use shared::{count_digits, SmallMap};

fn main() {
	shared::print_answers(11, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let stones = input
		.trim()
		.split(' ')
		.map(|n| (n.parse().unwrap(), 1))
		.collect::<SmallMap<_, _>>();
	let stones = blink_n_times(stones, 25);
	stones.into_iter().map(|(_, count)| count).sum()
}

fn get_answer_2(input: &str) -> u64 {
	let stones = input
		.trim()
		.split(' ')
		.map(|n| (n.parse().unwrap(), 1))
		.collect::<SmallMap<_, _>>();
	let stones = blink_n_times(stones, 75);
	stones.into_iter().map(|(_, count)| count).sum()
}

fn step(n: u64) -> (u64, Option<u64>) {
	if n == 0 {
		return (1, None);
	}
	let digits = count_digits(n, 10);
	if digits % 2 == 0 {
		let divisor = 10_u64.pow(digits as u32 / 2);
		let left = n / divisor;
		let right = n % divisor;
		(left, Some(right))
	} else {
		(n * 2024, None)
	}
}

fn blink_n_times(mut stones: SmallMap<u64, u64>, n: u8) -> SmallMap<u64, u64> {
	for _ in 0..n {
		let mut new_stones = SmallMap::new();
		for (stone, count) in stones {
			let (a, b) = step(stone);
			*new_stones.get_mut_or_insert(a, 0) += count;
			if let Some(b) = b {
				*new_stones.get_mut_or_insert(b, 0) += count;
			}
		}
		stones = new_stones;
	}
	stones
}
