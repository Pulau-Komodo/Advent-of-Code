pub fn part_a(input: String) -> String {
	let passports = input.split("\r\n\r\n");
	let count = passports.filter(validate_passport_a).count();
	format!("{}", count)
}

fn validate_passport_a(passport: &&str) -> bool {
	let elements = passport.split_ascii_whitespace();
	let mut birth_year = false;
	let mut issue_year = false;
	let mut expiration_year = false;
	let mut height = false;
	let mut hair_colour = false;
	let mut eye_colour = false;
	let mut passport_id = false;
	for element in elements {
		let (key, _) = element.split_once(':').unwrap();
		match key {
			"byr" => birth_year = true,
			"iyr" => issue_year = true,
			"eyr" => expiration_year = true,
			"hgt" => height = true,
			"hcl" => hair_colour = true,
			"ecl" => eye_colour = true,
			"pid" => passport_id = true,
			_ => (),
		}
	}
	birth_year
		&& issue_year
		&& expiration_year
		&& height
		&& hair_colour
		&& eye_colour
		&& passport_id
}

pub fn part_b(input: String) -> String {
	let passports = input.split("\r\n\r\n");
	let count = passports.filter(validate_passport_b).count();
	format!("{}", count)
}

fn validate_passport_b(passport: &&str) -> bool {
	let elements = passport.split_ascii_whitespace();
	let mut birth_year = false;
	let mut issue_year = false;
	let mut expiration_year = false;
	let mut height = false;
	let mut hair_colour = false;
	let mut eye_colour = false;
	let mut passport_id = false;
	for element in elements {
		let (key, value) = element.split_once(':').unwrap();
		match key {
			"byr" => birth_year = validate_number_field(value, 1920, 2002),
			"iyr" => issue_year = validate_number_field(value, 2010, 2020),
			"eyr" => expiration_year = validate_number_field(value, 2020, 2030),
			"hgt" => {
				if let Some(value) = value.strip_suffix("cm") {
					height = validate_number_field(value, 150, 193);
				} else if let Some(value) = value.strip_suffix("in") {
					height = validate_number_field(value, 59, 76);
				}
			}
			"hcl" => hair_colour = validate_hair_colour(value),
			"ecl" => eye_colour = validate_eye_colour(value),
			"pid" => passport_id = validate_passport_id(value),
			_ => (),
		}
	}
	birth_year
		&& issue_year
		&& expiration_year
		&& height
		&& hair_colour
		&& eye_colour
		&& passport_id
}

fn validate_number_field(number: &str, min: u16, max: u16) -> bool {
	let number = number.parse::<u16>().unwrap();
	number >= min && number <= max
}

fn validate_hair_colour(colour: &str) -> bool {
	const VALID_CHARS: [char; 16] = [
		'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
	];
	let colour = match colour.strip_prefix('#') {
		Some(colour) => colour,
		None => return false,
	};
	if colour.len() != 6 {
		return false;
	}
	colour.chars().all(|char| VALID_CHARS.contains(&char))
}

fn validate_eye_colour(colour: &str) -> bool {
	const VALID_COLOURS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
	VALID_COLOURS.contains(&colour)
}

fn validate_passport_id(id: &str) -> bool {
	const VALID_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
	if id.len() != 9 {
		return false;
	}
	id.chars().all(|char| VALID_CHARS.contains(&char))
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_part_a() {
		let input = std::fs::read_to_string(format!("./input/{}/{}.txt", 2020, 4))
			.expect("Could not read file");
		assert_eq!(part_a(input), format!("{}", 204))
	}
}
