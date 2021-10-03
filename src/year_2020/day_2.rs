fn split_password_line(password_line: &str) -> (usize, usize, char, &str) {
	let mut elements = password_line.split(' ');
	let numbers = elements.next().unwrap().split_once('-').unwrap();
	let first_number = numbers.0.parse::<usize>().unwrap();
	let second_number = numbers.1.parse::<usize>().unwrap();
	let letter = elements
		.next()
		.unwrap()
		.strip_suffix(':')
		.unwrap()
		.chars()
		.next()
		.unwrap();
	let password = elements.next().unwrap();
	(first_number, second_number, letter, password)
}

pub fn part_a(input: String) -> String {
	let count = input.lines().filter(validate_password_a).count();
	format!("{}", count)
}

fn validate_password_a(password_line: &&str) -> bool {
	let (min_length, max_length, letter, password) = split_password_line(password_line);
	let letter_count = password.chars().filter(|&char| char == letter).count();
	letter_count >= min_length && letter_count <= max_length
}

pub fn part_b(input: String) -> String {
	let count = input.lines().filter(validate_password_b).count();
	format!("{}", count)
}

fn validate_password_b(password_line: &&str) -> bool {
	let (first_position, second_position, letter, password) = split_password_line(password_line);
	let mut chars = password.chars().skip(first_position - 1);
	let first_char = chars.next().unwrap_or(' ');
	let second_char = chars
		.nth(second_position - first_position - 1)
		.unwrap_or(' ');
	first_char != second_char && (first_char == letter || second_char == letter)
}
