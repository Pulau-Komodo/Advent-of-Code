fn main() {
	shared::print_answers(21, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let positions = parse_input(input);
	let mut positions = [positions[0] as u32, positions[1] as u32];
	let mut scores = [0; 2];
	let mut die_rolls = 0;
	'outer: loop {
		for (position, score) in positions.iter_mut().zip(scores.iter_mut()) {
			let roll = roll(die_rolls);
			die_rolls += 3;
			*position = (*position + roll) % 10;
			*score += *position + 1;
			if *score >= 1000 {
				break 'outer;
			}
		}
	}
	(scores.iter().min().unwrap() * die_rolls) as u64
}

fn get_answer_2(input: &str) -> u64 {
	let positions = parse_input(input);
	let completions_per_turn = positions.map(get_completions_per_turn);
	let non_completions_per_turn = completions_per_turn.map(get_non_completions_per_turn);
	let wins = get_wins(completions_per_turn, non_completions_per_turn);
	*wins.iter().max().unwrap()
}

fn get_wins(
	completions_per_turn: [[u64; 11]; 2],
	non_completions_per_turn: [[u64; 11]; 2],
) -> [u64; 2] {
	[
		completions_per_turn[0]
			.iter()
			.skip(1)
			.chain(std::iter::once(&0))
			.zip(non_completions_per_turn[1])
			.map(|(p1_completions, p2_non_completions)| p1_completions * p2_non_completions)
			.sum(),
		completions_per_turn[1]
			.iter()
			.zip(non_completions_per_turn[0])
			.map(|(p2_completions, p1_non_completions)| p2_completions * p1_non_completions)
			.sum(),
	]
}

fn parse_input(input: &str) -> [u8; 2] {
	let mut lines = input.lines();
	let player_one: u8 = lines
		.next()
		.and_then(|line| line.strip_prefix("Player 1 starting position: "))
		.and_then(|n| n.parse().ok())
		.unwrap();
	let player_two: u8 = lines
		.next()
		.and_then(|line| line.strip_prefix("Player 2 starting position: "))
		.and_then(|n| n.parse().ok())
		.unwrap();
	[player_one - 1, player_two - 1]
}

fn roll(die_rolls: u32) -> u32 {
	match die_rolls % 100 {
		98 => 200,
		99 => 103,
		n => n * 3 + 6,
	}
}

fn get_completions_per_turn(position: u8) -> [u64; 11] {
	let mut completions_per_turn = [0; 11];
	let mut non_completions_per_turn = [0; 11];
	do_solo_turn(
		position,
		0,
		1,
		1,
		&mut completions_per_turn,
		&mut non_completions_per_turn,
	);
	completions_per_turn
}

fn get_non_completions_per_turn(completions_per_turn: [u64; 11]) -> [u64; 11] {
	let mut prev = None;
	completions_per_turn.map(|completions| {
		if let Some(previous) = prev {
			let non_completions = previous * 27 - completions;
			prev = Some(non_completions);
			non_completions
		} else {
			prev = Some(1);
			1
		}
	})
}

const DURAC_ROLLS: [(u64, u8); 7] = [(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)];

fn do_solo_turn(
	position: u8,
	score: u8,
	turn: u8,
	branch_count: u64,
	mut completions_per_turn: &mut [u64; 11],
	mut non_completions_per_turn: &mut [u64; 11],
) {
	for (count, roll) in DURAC_ROLLS {
		let branch_count = branch_count * count;
		let position = (position + roll) % 10;
		let score = score + position + 1;
		if score >= 21 {
			completions_per_turn[turn as usize] += branch_count;
		} else {
			non_completions_per_turn[turn as usize] += branch_count;
			do_solo_turn(
				position,
				score,
				turn + 1,
				branch_count,
				&mut completions_per_turn,
				&mut non_completions_per_turn,
			);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample_input() {
		let positions = [4 - 1, 8 - 1];
		let completions_per_turn = positions.map(get_completions_per_turn);
		let non_completions_per_turn = completions_per_turn.map(get_non_completions_per_turn);
		let wins = get_wins(completions_per_turn, non_completions_per_turn);
		assert_eq!(wins, [444356092776315, 341960390180808]);
	}
}
