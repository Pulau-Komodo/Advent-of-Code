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
	let wins = wins(positions);
	*wins.iter().max().unwrap()
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

fn wins(positions: [u8; 2]) -> [u64; 2] {
	let mut wins = [0, 0];
	turn(false, 1, positions, [0, 0], &mut wins);
	wins
}

const DURAC_ROLLS: [(u64, u8); 7] = [(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)];

fn turn(
	player_two_turn: bool,
	branch_count: u64,
	positions: [u8; 2],
	scores: [u8; 2],
	mut wins: &mut [u64; 2],
) {
	for (count, roll) in DURAC_ROLLS {
		let branch_count = branch_count * count;
		let position = (positions[0] + roll) % 10;
		let score = scores[0] + position + 1;
		if score >= 21 {
			wins[player_two_turn as usize] += branch_count;
		} else {
			let positions = [positions[1], position];
			let scores = [scores[1], score];
			turn(!player_two_turn, branch_count, positions, scores, &mut wins);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample_input() {
		let wins = wins([4 - 1, 8 - 1]);
		assert_eq!(wins, [444356092776315, 341960390180808]);
	}
}
