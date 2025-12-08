use std::array;

use shared::IteratorTop;

fn main() {
	shared::print_answers(8, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let junctions: Vec<[u64; 3]> = input
		.lines()
		.map(|line| {
			let mut numbers = line.split(',').map(|n| n.parse().unwrap());
			array::from_fn(|_| numbers.next().unwrap())
		})
		.collect();

	let mut connections: Vec<_> = (0..junctions.len() - 1)
		.flat_map(|a| (a + 1..junctions.len()).map(move |b| (a, b)))
		.map(|(a, b)| (a, b, distance_squared(junctions[a], junctions[b])))
		.collect();
	connections.sort_by(|a, b| a.2.cmp(&b.2));

	let mut connections = Vec::from_iter(
		connections
			.into_iter()
			.map(|(a, b, _)| (a, b))
			.take(1000)
			.rev(),
	);
	let mut network = Vec::new();
	while let Some((a, b)) = connections.pop() {
		let mut circuit = Vec::from([a, b]);
		let mut frontier = Vec::from([a, b]);
		let mut new_frontier = Vec::new();
		loop {
			for node in frontier.drain(..) {
				for (new_a, new_b) in
					connections.extract_if(.., |(new_a, new_b)| *new_a == node || *new_b == node)
				{
					for new_node in [new_a, new_b] {
						if !circuit.contains(&new_node) {
							circuit.push(new_node);
							new_frontier.push(new_node);
						}
					}
				}
			}
			if new_frontier.is_empty() {
				break;
			}
			std::mem::swap(&mut frontier, &mut new_frontier);
		}
		network.push(circuit);
	}

	network
		.into_iter()
		.map(|circuit| circuit.len())
		.top::<3>()
		.into_iter()
		.product::<usize>() as u32
}

fn get_answer_2(input: &str) -> u32 {
	let junctions: Vec<[u64; 3]> = input
		.lines()
		.map(|line| {
			let mut numbers = line.split(',').map(|n| n.parse().unwrap());
			array::from_fn(|_| numbers.next().unwrap())
		})
		.collect();

	let mut connections: Vec<_> = (0..junctions.len() - 1)
		.flat_map(|a| (a + 1..junctions.len()).map(move |b| (a, b)))
		.map(|(a, b)| (a, b, distance_squared(junctions[a], junctions[b])))
		.collect();
	connections.sort_by(|a, b| a.2.cmp(&b.2));

	let connections: Vec<_> = connections.into_iter().map(|(a, b, _)| (a, b)).collect();
	let mut network: Vec<Vec<usize>> = Vec::new();
	for (a, b) in connections {
		if let Some(circuit) = network
			.iter()
			.position(|circuit| circuit.contains(&a) || circuit.contains(&b))
		{
			for node in [a, b] {
				if !network[circuit].contains(&node) {
					network[circuit].push(node);
				}
			}
			if let Some(other_circuit) =
				network
					.iter()
					.enumerate()
					.find_map(|(index, other_circuit)| {
						(index != circuit
							&& (other_circuit.contains(&a) || other_circuit.contains(&b)))
						.then_some(index)
					}) {
				let other_circuit = network.remove(other_circuit);
				for node in other_circuit {
					if !network[circuit].contains(&node) {
						network[circuit].push(node);
					}
				}
			}
			if network[circuit].len() == junctions.len() {
				return (junctions[a][0] * junctions[b][0]) as u32;
			}
		} else {
			network.push(Vec::from([a, b]));
		}
	}
	panic!();
}

fn distance_squared(a: [u64; 3], b: [u64; 3]) -> u64 {
	a.into_iter()
		.zip(b)
		.map(|(a, b)| a.abs_diff(b).pow(2))
		.sum()
}
