fn main() {
	shared::print_answers(9, &[get_answers]);
}

fn parse_input(input: &str) -> Vec<u64> {
	input
		.lines()
		.map(str::parse)
		.collect::<Result<_, _>>()
		.unwrap()
}

fn test_valid(number: u64, preceding: &[u64]) -> bool {
	preceding.iter().enumerate().any(|(index, first_number)| {
		preceding
			.iter()
			.skip(index + 1)
			.any(|second_number| first_number + second_number == number)
	})
}

fn find_window(number: u64, all_numbers: &[u64]) -> &[u64] {
	let mut start = 0;
	let mut end = 2;
	let mut sum: u64 = all_numbers[start..end].iter().sum();
	loop {
		#[allow(clippy::comparison_chain)]
		if sum == number {
			return &all_numbers[start..end];
		} else if sum < number {
			sum += all_numbers[end];
			end += 1;
		} else if sum > number {
			sum -= all_numbers[start];
			start += 1;
			if start + 2 > end {
				sum += all_numbers[end];
				end += 1;
			}
		}
	}
}

fn get_answers(input: &str) -> String {
	let numbers = parse_input(input);
	let (_invalid_index, invalid_number) = numbers
		.iter()
		.skip(25)
		.enumerate()
		.find(|(index, &n)| !test_valid(n, &numbers[*index..*index + 25]))
		.unwrap();
	let window = find_window(*invalid_number, &numbers);
	let (min, max) = window.iter().fold((u64::MAX, u64::MIN), |(min, max), &n| {
		(min.min(n), max.max(n))
	});
	let sum = min + max;

	format!("1: {}, 2: {}", invalid_number, sum)
}
