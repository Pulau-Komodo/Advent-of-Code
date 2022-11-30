use std::collections::HashMap;

fn main() {
	shared::print_answers(14, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut possible_keys: HashMap<char, Vec<u32>> = HashMap::new();
	let mut confirmed_keys = Vec::new();
	let mut final_thousand = None;
	for n in 0.. {
		let hash = string_hash(&format!("{input}{n}"));
		for (char, key_list) in &mut possible_keys {
			key_list.retain(|index| n <= index + 1000);
			if key_list.is_empty() {
				continue;
			}
			if hash.contains(&format!("{char}{char}{char}{char}{char}")) {
				if confirmed_keys.len() >= 64 && final_thousand.is_none() {
					final_thousand = Some(1000_u32);
				}
				confirmed_keys.append(key_list);
			}
		}
		if let Some(char) = find_possible_key(&hash) {
			possible_keys.entry(char).or_default().push(n);
		}
		final_thousand = final_thousand.map(|n| n - 1);
		if let Some(0) = final_thousand {
			break;
		}
	}
	confirmed_keys.sort_unstable();
	confirmed_keys[63]
}

fn get_answer_2(input: &str) -> u32 {
	let mut possible_keys: HashMap<char, Vec<u32>> = HashMap::new();
	let mut confirmed_keys = Vec::new();
	let mut final_thousand = None;
	for n in 0.. {
		let mut hash = string_hash(&format!("{input}{n}"));
		for _n in 0..2016 {
			hash = string_hash(&hash);
		}
		for (char, key_list) in &mut possible_keys {
			key_list.retain(|index| n <= index + 1000);
			if key_list.is_empty() {
				continue;
			}
			if hash.contains(&format!("{char}{char}{char}{char}{char}")) {
				if confirmed_keys.len() >= 64 && final_thousand.is_none() {
					final_thousand = Some(1000_u32);
				}
				confirmed_keys.append(key_list);
			}
		}
		if let Some(char) = find_possible_key(&hash) {
			possible_keys.entry(char).or_default().push(n);
		}
		final_thousand = final_thousand.map(|n| n - 1);
		if let Some(0) = final_thousand {
			break;
		}
	}
	confirmed_keys.sort_unstable();
	confirmed_keys[63]
}

fn find_possible_key(hash: &str) -> Option<char> {
	let mut chars = hash.chars();
	let mut previous_char = chars.next().unwrap();
	let mut two_equal = false;
	for char in chars {
		if char == previous_char {
			if two_equal {
				return Some(char);
			} else {
				two_equal = true;
			}
		} else {
			two_equal = false;
			previous_char = char;
		}
	}
	None
}

fn string_hash(input: &str) -> String {
	format!("{:032x}", shared::md5(input.as_bytes()))
}
