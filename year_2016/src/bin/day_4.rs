use std::collections::HashMap;

fn main() {
	shared::print_answers(4, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	input
		.lines()
		.map(Room::from_str)
		.filter(Room::validate_checksum)
		.map(|room| room.sector_id())
		.sum::<u32>()
}

fn get_answer_2(input: &str) -> u32 {
	input
		.lines()
		.map(Room::from_str)
		.filter(Room::validate_checksum)
		.find_map(|room| {
			room.decrypt_name()
				.contains("north")
				.then_some(room.sector_id)
		})
		.expect("Could not find a room containing \"north\".")
}

struct Room<'l> {
	name: &'l str,
	sector_id: u32,
	checksum: &'l str,
}

impl<'l> Room<'l> {
	fn from_str(str: &'l str) -> Self {
		let (name, rest) = str.rsplit_once('-').unwrap();
		let (sector_id, checksum) = rest.split_once('[').unwrap();
		let sector_id = sector_id.parse().unwrap();
		let checksum = &checksum[0..5];
		Self {
			name,
			sector_id,
			checksum,
		}
	}
	fn validate_checksum(&self) -> bool {
		let mut char_frequencies = HashMap::<char, u8>::new();
		for char in self.name.chars().filter(|&char| char != '-') {
			*char_frequencies.entry(char).or_default() += 1;
		}
		let mut last_char = (char::default(), u8::MAX);
		for char in self.checksum.chars() {
			if let Some(frequency) = char_frequencies.remove(&char) {
				if frequency > last_char.1 || frequency == last_char.1 && char < last_char.0 {
					return false;
				}
				last_char = (char, frequency);
			} else {
				return false;
			}
		}
		for (char, frequency) in char_frequencies.into_iter() {
			if frequency > last_char.1 || frequency == last_char.1 && char < last_char.0 {
				return false;
			}
		}
		true
	}
	fn sector_id(&self) -> u32 {
		self.sector_id
	}
	fn decrypt_name(&self) -> String {
		self.name
			.as_bytes()
			.iter()
			.map(|&c| {
				if c == b'-' {
					' '
				} else {
					let num = c as u16 - 97;
					let c = (num + self.sector_id as u16) % 26 + 97;
					char::from(c as u8)
				}
			})
			.collect()
	}
}
