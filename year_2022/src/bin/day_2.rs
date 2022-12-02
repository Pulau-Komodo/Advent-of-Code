fn main() {
	shared::print_answers(2, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	input
		.lines()
		.map(RpsMatchV1::from_str)
		.map(|rps_match| rps_match.score())
		.sum()
}

fn get_answer_2(input: &str) -> u32 {
	input
		.lines()
		.map(RpsMatchV2::from_str)
		.map(|rps_match| rps_match.score())
		.sum()
}

enum Outcome {
	Win,
	Loss,
	Tie,
}
use Outcome::*;

impl Outcome {
	fn from_char(char: char) -> Self {
		match char {
			'X' => Loss,
			'Y' => Tie,
			'Z' => Win,
			_ => panic!("Invalid input."),
		}
	}
	fn score(&self) -> u32 {
		match self {
			Win => 6,
			Tie => 3,
			Loss => 0,
		}
	}
}

enum RpsPlay {
	Rock,
	Paper,
	Scissors,
}
use RpsPlay::*;

impl RpsPlay {
	fn from_char(char: char) -> Self {
		match char {
			'A' => Rock,
			'B' => Paper,
			'C' => Scissors,
			_ => panic!("Invalid input."),
		}
	}
	/// # Caution
	/// Based on hasty assumptions later proven wrong. This *must* not be used with the second part of the challenge.
	fn from_other_char(char: char) -> Self {
		match char {
			'X' => Rock,
			'Y' => Paper,
			'Z' => Scissors,
			_ => panic!("Invalid input."),
		}
	}
	fn score(&self) -> u32 {
		match self {
			Rock => 1,
			Paper => 2,
			Scissors => 3,
		}
	}
	fn play_against(&self, other: &Self) -> Outcome {
		match (self, other) {
			(Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Loss,
			(Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => Win,
			(Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Tie,
		}
	}
	fn find_play_with_desired_outcome(&self, outcome: &Outcome) -> Self {
		match (self, outcome) {
			(Rock, Tie) | (Scissors, Win) | (Paper, Loss) => Rock,
			(Paper, Tie) | (Rock, Win) | (Scissors, Loss) => Paper,
			(Scissors, Tie) | (Paper, Win) | (Rock, Loss) => Scissors,
		}
	}
}

struct RpsMatchV1 {
	plays: [RpsPlay; 2],
}

impl RpsMatchV1 {
	fn from_str(str: &str) -> Self {
		let mut chars = str.chars();
		let play_one = RpsPlay::from_char(chars.next().unwrap());
		let play_two = RpsPlay::from_other_char(chars.nth(1).unwrap());
		Self {
			plays: [play_one, play_two],
		}
	}
	fn score(&self) -> u32 {
		self.plays[1].play_against(&self.plays[0]).score() + self.plays[1].score()
	}
}

struct RpsMatchV2 {
	other_play: RpsPlay,
	outcome: Outcome,
}

impl RpsMatchV2 {
	fn from_str(str: &str) -> Self {
		let mut chars = str.chars();
		let other_play = RpsPlay::from_char(chars.next().unwrap());
		let outcome = Outcome::from_char(chars.nth(1).unwrap());
		Self {
			other_play,
			outcome,
		}
	}
	fn score(&self) -> u32 {
		self.other_play
			.find_play_with_desired_outcome(&self.outcome)
			.score() + self.outcome.score()
	}
}
