fn main() {
	shared::print_answers(7, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	input
		.lines()
		.map(Equation::from_line)
		.filter(|equation| equation.test())
		.map(|equation| equation.test_value)
		.sum()
}

fn get_answer_2(input: &str) -> u64 {
	input
		.lines()
		.map(Equation::from_line)
		.filter(|equation| equation.test_v2())
		.map(|equation| equation.test_value)
		.sum()
}

struct Equation {
	test_value: u64,
	equation_values: Vec<u64>,
}

impl Equation {
	fn from_line(line: &str) -> Self {
		let (test_value, equation_values) = line.split_once(": ").unwrap();
		let test_value = test_value.parse().unwrap();
		let equation_values = equation_values
			.split(' ')
			.map(|n| n.parse().unwrap())
			.collect();
		Self {
			test_value,
			equation_values,
		}
	}
	fn test(&self) -> bool {
		for mut step in 0..2_u32.pow(self.equation_values.len() as u32 - 1) {
			let mut outcome = self.equation_values[0];
			for value in &self.equation_values[1..] {
				if step & 1 == 1 {
					outcome *= value;
				} else {
					outcome += value;
				}
				step >>= 1;
			}
			if outcome == self.test_value {
				return true;
			}
		}
		false
	}
	fn test_v2(&self) -> bool {
		'outer: for mut step in 0..3_u32.pow(self.equation_values.len() as u32 - 1) {
			let mut outcome = self.equation_values[0];
			for value in &self.equation_values[1..] {
				match step % 3 {
					0 => outcome += value,
					1 => outcome *= value,
					2 => outcome = outcome * 10_u64.pow(count_digits(*value)) + value,
					_ => unreachable!(),
				}
				step /= 3;
				if outcome > self.test_value {
					continue 'outer;
				}
			}
			if outcome == self.test_value {
				return true;
			}
		}
		false
	}
}

fn count_digits(mut n: u64) -> u32 {
	(1..)
		.find(|_| {
			n /= 10;
			n == 0
		})
		.unwrap()
}
