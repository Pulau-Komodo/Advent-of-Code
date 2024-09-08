use shared::IntoPairIterator;

fn main() {
	shared::print_answers(2, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> String {
	let mut doubles = 0;
	let mut triples = 0;
	for id in input.lines() {
		let mut id: Vec<u8> = id.bytes().collect();
		id.sort();
		let mut run = 1;
		let mut found_double = false;
		let mut found_triple = false;
		for (a, b) in id.iter().pairs() {
			if a == b {
				run += 1;
			} else {
				if run == 2 {
					found_double = true;
					if found_triple {
						break;
					}
				} else if run == 3 {
					found_triple = true;
					if found_double {
						break;
					}
				}
				run = 1;
			}
		}
		if found_double {
			doubles += 1;
		}
		if found_triple {
			triples += 1;
		}
	}
	format!("{}", doubles * triples)
}

fn get_answer_2(input: &str) -> String {
	let ids: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
	for a in 0..ids.len() - 1 {
		'b: for b in a + 1..ids.len() {
			let mut non_match_position = None;
			for (index, (byte_a, byte_b)) in ids[a].iter().zip(ids[b]).enumerate() {
				if byte_a != byte_b {
					if non_match_position.is_none() {
						non_match_position = Some(index);
					} else {
						continue 'b;
					}
				}
			}
			if let Some(index) = non_match_position {
				let mut id = ids[a].to_vec();
				id.remove(index);
				return id.into_iter().map(|n| n as char).collect();
			}
		}
	}
	panic!();
}
