fn main() {
	shared::print_answers(5, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> String {
	let mut password = String::with_capacity(8);
	for n in 0.. {
		let string = format!("{input}{n}");
		let hash = shared::md5(string.as_bytes());
		if hash < THRESHOLD {
			let char = char::from_digit((hash >> 104) as u32, 16).unwrap();
			password.push(char);
		}
		if password.len() >= 8 {
			break;
		}
	}
	password
}

fn get_answer_2(input: &str) -> String {
	let mut password = [None; 8];
	let mut found_count = 0;
	for n in 0.. {
		let string = format!("{input}{n}");
		let hash = shared::md5(string.as_bytes());
		if hash < THRESHOLD {
			let char = char::from_digit((hash >> 100 & 0xF) as u32, 16).unwrap();
			let pos = (hash >> 104) as usize;
			password.get_mut(pos).map(|old_char| {
				old_char.get_or_insert_with(|| {
					found_count += 1;
					char
				})
			});
		}
		if found_count >= 8 {
			break;
		}
	}
	password.into_iter().flatten().collect()
}

const THRESHOLD: u128 = 0x0000_1000_0000_0000_0000_0000_0000_0000;
