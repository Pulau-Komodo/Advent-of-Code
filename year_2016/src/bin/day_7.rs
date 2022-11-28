use std::collections::VecDeque;

fn main() {
	shared::print_answers(7, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	input.lines().filter(|line| supports_tls(line)).count() as u32
}

fn get_answer_2(input: &str) -> u32 {
	input.lines().filter(|line| supports_ssh(line)).count() as u32
}

fn supports_tls(address: &str) -> bool {
	let mut found_abba = false;
	for (n, segment) in address.split(&['[', ']']).enumerate() {
		let outside_brackets = n % 2 == 0;
		if found_abba && outside_brackets {
			continue;
		}
		if has_abba(segment) {
			if outside_brackets {
				found_abba = true;
			} else {
				return false;
			}
		}
	}
	found_abba
}

fn has_abba(segment: &str) -> bool {
	let mut past_three_chars = VecDeque::from([' ', ' ', ' ']);
	for char in segment.chars() {
		if past_three_chars[1] == past_three_chars[2]
			&& char == past_three_chars[0]
			&& char != past_three_chars[1]
		{
			return true;
		}
		past_three_chars.pop_front();
		past_three_chars.push_back(char);
	}
	false
}

struct Aba {
	outer: char,
	inner: char,
}

impl Aba {
	fn corresponds_to(&self, other: &Self) -> bool {
		self.outer == other.inner && self.inner == other.outer
	}
}

fn supports_ssh(address: &str) -> bool {
	let mut abas = Vec::with_capacity(address.len() - 2);
	let mut babs = Vec::with_capacity(address.len() - 2);
	for (n, segment) in address.split(&['[', ']']).enumerate() {
		let outside_brackets = n % 2 == 0;
		for find in find_abas(segment) {
			let (list, other_list) = if outside_brackets {
				(&mut abas, &babs)
			} else {
				(&mut babs, &abas)
			};
			if other_list.iter().any(|item| find.corresponds_to(item)) {
				return true;
			}
			list.push(find);
		}
	}
	false
}

fn find_abas(segment: &str) -> Vec<Aba> {
	let mut past_two_chars = VecDeque::from([' ', ' ']);
	let mut found = Vec::with_capacity(segment.len() - 2);
	for char in segment.chars() {
		if char == past_two_chars[0] && char != past_two_chars[1] {
			let aba = Aba {
				outer: char,
				inner: past_two_chars[1],
			};
			found.push(aba);
		}
		past_two_chars.pop_front();
		past_two_chars.push_back(char);
	}
	found
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn examples() {
		assert!(supports_tls("abba[mnop]qrst"));
		assert!(!supports_tls("abcd[bddb]xyyx"));
		assert!(!supports_tls("aaaa[qwer]tyui"));
		assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
	}
}
