fn main() {
	shared::print_answers(24, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> i64 {
	let values = get_significant_values(input);
	try_all_codes(values, false)
}

fn get_answer_2(input: &str) -> i64 {
	let values = get_significant_values(input);
	try_all_codes(values, true)
}

fn get_significant_values(input: &str) -> [(i64, i64); 14] {
	let mut values = [(0, 0); 14];
	let mut x = 0;
	for (index, line) in input.lines().enumerate() {
		let segment_line = index % 18;
		let segment = index / 18;
		if segment_line == 5 {
			x = line.strip_prefix("add x ").unwrap().parse().unwrap();
		} else if segment_line == 15 {
			values[segment] = (x, line.strip_prefix("add y ").unwrap().parse().unwrap());
		}
	}
	values
}

fn try_all_codes(values: [(i64, i64); 14], incrementing: bool) -> i64 {
	let digit_map = generate_digit_map(&values);
	let mut code = if incrementing { [1; 7] } else { [9; 7] };
	loop {
		let mut alu = Alu::new(&code);
		for step in values {
			if !alu.step(step.0, step.1) {
				break;
			}
		}
		if alu.z == 0 && alu.current_digit == 14 {
			return make_code_into_number(alu.code);
		}
		if incrementing {
			increment_digit(&mut code, digit_map[alu.current_digit]);
		} else {
			decrement_digit(&mut code, digit_map[alu.current_digit]);
		}
	}
}

fn generate_digit_map(values: &[(i64, i64); 14]) -> [usize; 14] {
	let mut digit_map = [0; 14];
	let mut current_index = 0;
	for (&(x, _y), map_entry) in values.iter().zip(&mut digit_map).skip(1) {
		if x > 0 {
			current_index += 1;
		}
		*map_entry = current_index;
	}
	digit_map
}

struct Alu {
	code: [i64; 14],
	current_digit: usize,
	z: i64,
}

impl Alu {
	fn new(code: &[i64; 7]) -> Self {
		let code = [
			code[0], code[1], code[2], 0, code[3], 0, 0, code[4], code[5], code[6], 0, 0, 0, 0,
		];
		Self {
			code,
			current_digit: 0,
			z: 0,
		}
	}
	fn step(&mut self, add_to_x: i64, add_to_z: i64) -> bool {
		let x = self.z % 26 + add_to_x;
		if add_to_x <= 0 {
			self.z /= 26;
			if (1..10).contains(&x) {
				self.code[self.current_digit] = x;
			} else {
				return false;
			}
		} else {
			self.z *= 26;
			self.z += self.code[self.current_digit] + add_to_z;
		}
		self.current_digit += 1;
		true
	}
}

fn decrement_digit(code: &mut [i64; 7], which: usize) {
	for digit in code.iter_mut().rev().skip(6 - which) {
		if *digit > 1 {
			*digit -= 1;
			break;
		} else {
			*digit = 9;
		}
	}
}

fn increment_digit(code: &mut [i64; 7], which: usize) {
	for digit in code.iter_mut().rev().skip(6 - which) {
		if *digit < 9 {
			*digit += 1;
			break;
		} else {
			*digit = 1;
		}
	}
}

fn make_code_into_number(code: [i64; 14]) -> i64 {
	let mut number = 0;
	for digit in code {
		number *= 10;
		number += digit;
	}
	number
}
