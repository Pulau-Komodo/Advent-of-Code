use std::collections::VecDeque;

use shared::SmallMap;

fn main() {
	shared::print_answers(9, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let (player_count, marble_count) = get_counts(input);
	run_game(player_count, marble_count)
}

fn get_answer_2(input: &str) -> u32 {
	let (player_count, marble_count) = get_counts(input);
	run_game(player_count, marble_count * 100)
}

fn get_counts(input: &str) -> (u32, u32) {
	let mut words = input.split(' ');
	let player_count = words.next().unwrap().parse().unwrap();
	let marble_count = words.nth(5).unwrap().parse().unwrap();
	(player_count, marble_count)
}

fn run_game(player_count: u32, marble_count: u32) -> u32 {
	let mut marbles = VecDeque::new();
	marbles.push_back(0);
	let mut current_player = 0;
	let mut scores = SmallMap::new();
	for marble in 1..=marble_count {
		if marble % 23 == 0 {
			marbles.rotate_right(7);
			let score = marble + marbles.pop_back().unwrap();
			marbles.rotate_left(1);
			*scores.get_mut_or_insert(current_player, 0) += score;
		} else {
			marbles.rotate_left(1);
			marbles.push_back(marble);
		}
		current_player += 1;
		current_player %= player_count;
	}
	*scores.values().max().unwrap()
}
