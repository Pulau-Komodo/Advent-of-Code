fn main() {
	shared::print_answers(25, &[get_answer]);
}

struct SubjectNumber {
	number: u32,
	value: u32,
}

impl SubjectNumber {
	fn with_number(number: u32) -> Self {
		Self { number, value: 1 }
	}
	fn transform_step(&mut self) {
		const MODULO: u32 = 20201227;
		let mut a = self.number % MODULO;
		let mut b = self.value;
		let mut result = 0;
		while b > 0 {
			if b % 2 == 1 {
				result = (result + a) % MODULO;
			}
			a = (2 * a) % MODULO;
			b >>= 1;
		}
		self.value = result;
	}
}

fn find_loops(target: u32) -> u32 {
	let mut subject = SubjectNumber::with_number(7);
	for i in 0.. {
		if subject.value == target {
			return i as u32;
		}
		subject.transform_step();
	}
	unreachable!();
}

fn get_keys(input: &str) -> (u32, u32) {
	let mut numbers = input.lines().map(str::parse).map(Result::unwrap);
	(numbers.next().unwrap(), numbers.next().unwrap())
}

fn get_answer(input: &str) -> String {
	let keys = get_keys(input);
	let door_loops = find_loops(keys.0);
	let mut subject = SubjectNumber::with_number(keys.1);
	for _ in 0..door_loops {
		subject.transform_step();
	}
	format!("{}", subject.value)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn card_loop() {
		let mut subject = SubjectNumber::with_number(7);
		for _ in 0..8 {
			subject.transform_step();
		}
		assert_eq!(subject.value, 5764801);
	}
	#[test]
	fn door_loop() {
		let mut subject = SubjectNumber::with_number(7);
		for _ in 0..11 {
			subject.transform_step();
		}
		assert_eq!(subject.value, 17807724);
	}
	#[test]
	fn card_key() {
		let mut subject = SubjectNumber::with_number(17807724);
		for _ in 0..8 {
			subject.transform_step();
		}
		assert_eq!(subject.value, 14897079);
	}
	#[test]
	fn door_key() {
		let mut subject = SubjectNumber::with_number(5764801);
		for _ in 0..11 {
			subject.transform_step();
		}
		assert_eq!(subject.value, 14897079);
	}
}
