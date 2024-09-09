fn main() {
	shared::print_answers(5, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let mut chain: Vec<_> = input.trim().bytes().collect();
	chain = react_chain(chain);
	chain.len()
}

fn get_answer_2(input: &str) -> usize {
	let mut chain: Vec<_> = input.trim().bytes().collect();
	chain = react_chain(chain);
	(b'A'..=b'Z')
		.map(|unit_type| {
			let chain: Vec<_> = chain
				.iter()
				.filter(|&&byte| byte != unit_type && byte != unit_type + CASE_DISTANCE)
				.copied()
				.collect();
			react_chain(chain).len()
		})
		.min()
		.unwrap()
}

const CASE_DISTANCE: u8 = b'a' - b'A';

fn should_react(a: u8, b: u8) -> bool {
	a.abs_diff(b) == CASE_DISTANCE && a.min(b).is_ascii_uppercase()
}

fn react_chain(mut chain: Vec<u8>) -> Vec<u8> {
	let is_present = |byte: &u8| *byte != 0;
	loop {
		let mut a = 0;
		while a < chain.len() - 1 {
			let b = a + 1;
			if should_react(chain[a], chain[b]) {
				chain[a] = 0;
				chain[b] = 0;
				a += 2;
			} else {
				a += 1;
			}
		}
		let old_len = chain.len();
		chain = chain.into_iter().filter(is_present).collect();
		if chain.len() == old_len {
			break;
		}
	}
	chain
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_reaction() {
		assert!(should_react(b'a', b'A'));
		assert!(should_react(b'A', b'a'));
		assert!(!should_react(b'a', b'B'));
		assert!(!should_react(b'a', b'a'));
	}
}
