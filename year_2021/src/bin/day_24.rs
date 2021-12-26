fn main() {
	shared::print_answers(24, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let values = get_significant_values(input);
	math(values, false)
}

fn get_answer_2(input: &str) -> u64 {
	let values = get_significant_values(input);
	math(values, true)
}

fn get_significant_values(input: &str) -> [(i8, i8); 14] {
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

fn math(values: [(i8, i8); 14], find_lowest: bool) -> u64 {
	let mut pairs = Vec::with_capacity(7);
	let mut stack = Vec::with_capacity(7);
	for (index, (x, y)) in IntoIterator::into_iter(values).enumerate() {
		if x > 0 {
			stack.push((index, y));
		} else {
			let (first_index, value) = stack.pop().unwrap();
			pairs.push((first_index, index, value + x));
		}
	}
	let mut code = [0; 14];
	for (first, second, value) in pairs {
		let (first_value, second_value) = match (value < 0, find_lowest) {
			(true, false) => (9, 9 + value),
			(false, false) => (9 - value, 9),
			(true, true) => (1 - value, 1),
			(false, true) => (1, 1 + value),
		};
		code[first] = first_value;
		code[second] = second_value;
	}
	make_code_into_number(code)
}

fn make_code_into_number(code: [i8; 14]) -> u64 {
	let mut number = 0;
	for digit in code {
		number *= 10;
		number += digit as u64;
	}
	number
}
