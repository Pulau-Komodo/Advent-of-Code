use std::collections::HashMap;

fn main() {
	shared::print_answers(8, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let (instructions, nodes) = input.split_once("\n\n").unwrap();
	let node_map = make_node_map(nodes);
	let mut position = numberfy_node(b"AAA");
	let goal = numberfy_node(b"ZZZ");
	for (i, instruction) in instructions.bytes().cycle().enumerate() {
		if position == goal {
			return i;
		}
		let node = node_map.get(&position).expect("Position not found in map.");
		if instruction == b'L' {
			position = node.left;
		} else {
			position = node.right;
		}
	}
	panic!("No instructions.")
}

fn get_answer_2(input: &str) -> usize {
	let (instructions, nodes) = input.split_once("\n\n").unwrap();
	let instruction_cycle = instructions.len();
	let node_map = make_node_map(nodes);
	let starting_positions: Vec<_> = node_map
		.keys()
		.copied()
		.filter(|key| *key % 26 == 0)
		.collect();
	let mut positions = starting_positions.clone();
	let mut position_histories: Vec<_> = positions.iter().map(|_| HashMap::new()).collect();
	let mut cycle_lengths: Vec<_> = positions.iter().map(|_| None).collect();
	let mut z_points_in_cycle: Vec<_> = positions.iter().map(|_| Vec::new()).collect();

	let mut end_at = usize::MAX;
	for (i, instruction) in instructions.bytes().cycle().enumerate() {
		if positions.iter().all(|position| *position % 26 == 25) {
			return i; // We can dream.
		}
		for (position, z_points) in positions.iter().zip(&mut z_points_in_cycle) {
			if position % 26 == 25 {
				z_points.push(i / instruction_cycle);
			}
		}
		if i == end_at {
			break;
		}
		if end_at == usize::MAX && cycle_lengths.iter().all(|cycle| cycle.is_some()) {
			let max_cycle = cycle_lengths.iter().flatten().max().unwrap();
			end_at = i + max_cycle * instruction_cycle;
		}
		if i % instruction_cycle == 0 {
			for ((position, history), cycle) in positions
				.iter_mut()
				.zip(&mut position_histories)
				.zip(&mut cycle_lengths)
			{
				if let Some(last_time) = history.get(&*position) {
					*cycle = Some((i - last_time) / instruction_cycle);
				}
				history.insert(*position, i);
			}
		}
		for position in &mut positions {
			let node = node_map
				.get(&*position)
				.expect("Position not found in map.");
			if instruction == b'L' {
				*position = node.left;
			} else {
				*position = node.right;
			}
		}
	}

	for (cycle, z_points) in cycle_lengths.iter().zip(&z_points_in_cycle) {
		assert_eq!(*cycle, z_points.first().copied()); // If I'm going to make ugly assumptions, I'll at least assert one of them!
	}

	cycle_lengths.into_iter().flatten().product::<usize>() * instruction_cycle
}

struct Node {
	left: u16,
	right: u16,
}

impl Node {
	fn from_bytes(bytes: &[u8]) -> Self {
		Self {
			left: numberfy_node(&bytes[0..3]),
			right: numberfy_node(&bytes[5..8]),
		}
	}
}

fn numberfy_node(bytes: &[u8]) -> u16 {
	bytes
		.iter()
		.fold(0, |acc, byte| acc * 26 + (byte - b'A') as u16)
}

fn make_node_map(str: &str) -> HashMap<u16, Node> {
	str.lines()
		.map(str::as_bytes)
		.map(|node| {
			let name = numberfy_node(&node[0..3]);
			let node = Node::from_bytes(&node[7..15]);
			(name, node)
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_numberfication() {
		for node in ["AAA", "AAB", "BBA", "BBB", "AAY", "AAZ", "BBZ"] {
			let numberfied = numberfy_node(node.as_bytes());
			print!("{node}, {numberfied} ");
			match numberfied % 26 {
				0 => print!("ends with A."),
				25 => print!("ends with Z."),
				_ => (),
			}
			println!();
		}
	}
}
