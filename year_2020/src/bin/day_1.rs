fn main() {
	year_2020::print_answers(1, &[part_a, part_b]);
}

fn part_a(input: &str) -> String {
	let numbers = input
		.lines()
		.map(|line| {
			line.parse::<u64>()
				.expect("An input could not be parsed into u64")
		})
		.collect::<Vec<u64>>();

	for (index, number) in numbers.iter().enumerate() {
		for second_number in numbers.iter().skip(index + 1) {
			if number + second_number == 2020 {
				return format!("{}", number * second_number);
			}
		}
	}
	String::from("Found no solution")
}

fn part_b(input: &str) -> String {
	let numbers = input
		.lines()
		.map(|line| {
			line.parse::<u64>()
				.expect("An input could not be parsed into u64")
		})
		.collect::<Vec<u64>>();

	for (index, number) in numbers.iter().enumerate() {
		for (second_index, second_number) in numbers.iter().skip(index + 1).enumerate() {
			if number + second_number < 2020 {
				for third_number in numbers.iter().skip(index + second_index + 2) {
					if number + second_number + third_number == 2020 {
						return format!("{}", number * second_number * third_number);
					}
				}
			}
		}
	}
	String::from("Found no solution")
}
