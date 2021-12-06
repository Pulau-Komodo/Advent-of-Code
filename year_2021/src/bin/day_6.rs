fn main() {
	shared::print_answers(6, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let mut state = FishState::from_str(input);
	for _ in 0..80 {
		state.advance_generation();
	}
	state.count()
}

fn get_answer_2(input: &str) -> u64 {
	let mut state = FishState::from_str(input);
	for _ in 0..256 {
		state.advance_generation();
	}
	state.count()
}

struct FishState {
	state: [u64; 9],
}

impl FishState {
	fn from_str(str: &str) -> Self {
		let mut state = [0; 9];
		for number in str.split(',').map(str::parse::<usize>).map(Result::unwrap) {
			state[number] += 1;
		}
		FishState { state }
	}
	fn advance_generation(&mut self) {
		self.state = [
			self.state[1],
			self.state[2],
			self.state[3],
			self.state[4],
			self.state[5],
			self.state[6],
			self.state[7] + self.state[0],
			self.state[8],
			self.state[0],
		];
	}
	fn count(&self) -> u64 {
		self.state.iter().sum()
	}
}
