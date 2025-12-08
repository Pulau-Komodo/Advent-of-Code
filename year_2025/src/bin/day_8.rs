use std::{array, cmp::Reverse, collections::BinaryHeap};

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

	let mut connections = BinaryHeap::from_iter(
		(0..junctions.len() - 1)
			.flat_map(|a| (a + 1..junctions.len()).map(move |b| (a, b)))
			.map(|(a, b)| (Reverse(distance_squared(junctions[a], junctions[b])), a, b)),
	);

	let mut connections = Vec::from_iter((0..1000).map(|_| {
		let (_, a, b) = connections.pop().unwrap();
		(a, b)
	}));

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

	let mut connections = BinaryHeap::from_iter(
		(0..junctions.len() - 1)
			.flat_map(|a| (a + 1..junctions.len()).map(move |b| (a, b)))
			.map(|(a, b)| (Reverse(distance_squared(junctions[a], junctions[b])), a, b)),
	);

	let mut network: Vec<Vec<usize>> = Vec::new();
	while let Some((_, a, b)) = connections.pop() {
		let mut touched_circuit = None;
		match (
			network.iter().position(|circuit| circuit.contains(&a)),
			network.iter().position(|circuit| circuit.contains(&b)),
		) {
			(None, None) => network.push(Vec::from([a, b])),
			(Some(circuit), None) => {
				network[circuit].push(b);
				touched_circuit = Some(circuit);
			}
			(None, Some(circuit)) => {
				network[circuit].push(a);
				touched_circuit = Some(circuit)
			}
			(Some(circuit_a), Some(circuit_b)) if circuit_a == circuit_b => (),
			(Some(circuit_a), Some(circuit_b)) => {
				let circuit = network.remove(circuit_a.max(circuit_b));
				network[circuit_a.min(circuit_b)].extend(circuit);
				touched_circuit = Some(circuit_a.min(circuit_b));
			}
		};
		if let Some(circuit) = touched_circuit
			&& network[circuit].len() == junctions.len()
		{
			return (junctions[a][0] * junctions[b][0]) as u32;
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
