fn main() {
	shared::print_answers(6, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let mut values: Vec<Vec<_>> = input
		.lines()
		.map(|line| line.split_ascii_whitespace().collect())
		.collect();
	let numbers: Vec<Vec<u64>> = values[0..values.len() - 1]
		.iter()
		.map(|line| line.iter().map(|value| value.parse().unwrap()).collect())
		.collect();
	let operators = values.pop().unwrap();

	let mut sum = 0;
	for (i, operator) in operators.into_iter().enumerate() {
		let iter = numbers.iter().map(|line| line.get(i).unwrap());
		sum += match operator {
			"+" => iter.sum::<u64>(),
			"*" => iter.product(),
			_ => panic!(),
		};
	}
	sum
}

fn get_answer_2(input: &str) -> u64 {
	let values: Vec<_> = input
		.trim_end_matches('\n')
		.split('\n')
		.map(|line| line.as_bytes())
		.collect();

	let mut sum = 0;
	let mut numbers = Vec::new();
	for x in (0..values[0].len()).rev() {
		let mut number = 0;
		for y in 0..values.len() {
			let byte = values[y][x];
			if y == values.len() - 1 && number > 0 {
				numbers.push(number);
			}
			match byte {
				b' ' => (),
				b'0'..=b'9' => {
					number *= 10;
					number += (byte - b'0') as u64;
				}
				b'+' => sum += numbers.drain(..).sum::<u64>(),
				b'*' => sum += numbers.drain(..).product::<u64>(),
				_ => panic!("Found unexpected byte {byte}"),
			}
		}
	}
	sum
}
