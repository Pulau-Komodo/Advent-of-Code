use std::collections::{HashMap, HashSet};

fn main() {
	shared::print_answers(16, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let network = CompiledNetwork::from_network(PipeNetwork::from_str(input));
	let time = 30_u8;
	let mut frontier: Vec<_> = network
		.valves
		.iter()
		.enumerate()
		.map(|(index, valve)| CompiledState::starting_at([(index, valve)], time))
		.collect();
	let mut new_frontier = Vec::new();
	let mut greatest_flow_rate = 0;
	let mut visited = StateHistory::new(network.valves.len());
	loop {
		for state in frontier.drain(..) {
			for (index, valve) in network.open_valves(state.opened) {
				if let Some(new_state) = state.moved_to(index, valve, 0) {
					if visited.insert(new_state.positions, new_state.opened, new_state.flow_rate) {
						new_frontier.push(new_state);
					}
				}
			}
			greatest_flow_rate = greatest_flow_rate.max(state.flow_rate);
		}
		if new_frontier.is_empty() {
			break;
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
	greatest_flow_rate
}

fn get_answer_2(input: &str) -> u32 {
	let network = CompiledNetwork::from_network(PipeNetwork::from_str(input));
	let time = 26_u8;
	let mut frontier = Vec::new();
	for a in 0..network.valves.len() {
		for b in a + 1..network.valves.len() {
			let starts = [
				(a, network.valves.get(a).unwrap()),
				(b, network.valves.get(b).unwrap()),
			];
			frontier.push(CompiledState::starting_at(starts, time));
		}
	}
	let mut new_frontier = Vec::new();
	let mut greatest_flow_rate = 0;
	let mut visited = StateHistory::new(network.valves.len());
	for state in &frontier {
		visited.insert(state.positions, state.opened, state.flow_rate);
	}
	loop {
		for state in frontier.drain(..) {
			for (index, valve) in network.open_valves(state.opened) {
				let mut i = 0;
				let mut time_left = state.time_left.map(|time| {
					let p = i;
					i += 1;
					(p, time)
				});
				time_left.sort_unstable_by(|(_, a), (_, b)| b.cmp(a));
				if let Some(new_state) = time_left
					.into_iter()
					.filter_map(|(p, _)| state.moved_to(index, valve, p))
					.next()
				{
					if state.final_flow_rate_upper_bound(&network) > greatest_flow_rate
						&& visited.insert(
							new_state.positions,
							new_state.opened,
							new_state.flow_rate,
						) {
						new_frontier.push(new_state);
					}
				}
			}
			greatest_flow_rate = greatest_flow_rate.max(state.flow_rate);
		}
		if new_frontier.is_empty() {
			break;
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
	greatest_flow_rate
}

struct StateHistory<const N: usize> {
	states: Vec<Vec<u32>>,
	valve_count: usize,
}

impl<const N: usize> StateHistory<N> {
	fn new(valve_count: usize) -> Self {
		let capacity = (0..N).map(|n| valve_count - n).product::<usize>() / N;
		let open_states = 2_usize.pow(valve_count as u32);
		let state_flow_rate: Vec<u32> = (0..open_states).map(|_| 0).collect();
		let states = (0..capacity).map(|_| state_flow_rate.clone()).collect();
		Self {
			states,
			valve_count,
		}
	}
	fn insert(&mut self, mut pos: [usize; N], opened: u64, flow_rate: u32) -> bool {
		pos.sort_unstable();
		let index = if N == 1 {
			pos[0]
		} else if N == 2 {
			combination_index(pos[0], pos[1], self.valve_count)
		} else {
			unimplemented!()
		};
		let states = &mut self.states[index];
		if flow_rate > states[opened as usize] {
			states[opened as usize] = flow_rate;
			true
		} else {
			if N == 1 {}
			false
		}
	}
}

#[derive(Debug)]
struct Valve {
	flow_rate: u32,
	connections: Vec<usize>,
}

impl Valve {
	fn from_str_with_map<'l, 'm>(
		str: &'l str,
		valve_indices: &'m mut HashMap<&'l str, usize>,
	) -> (Self, usize) {
		let mut connections = Vec::with_capacity(2);
		let map_length = valve_indices.len();
		let valve_index = *valve_indices.entry(&str[6..8]).or_insert(map_length);
		let (flow_rate, rest) = str[23..].split_once(';').unwrap();
		let flow_rate = flow_rate.parse().unwrap();
		let (_, valves) = rest.rsplit_once(['e', 's']).unwrap();
		for valve in valves[1..].split(", ") {
			let map_length = valve_indices.len();
			let valve_index = *valve_indices.entry(valve).or_insert(map_length);
			connections.push(valve_index);
		}
		(
			Self {
				flow_rate,
				connections,
			},
			valve_index,
		)
	}
}

struct CompiledNetwork {
	valves: Vec<CompiledValve>,
}

impl CompiledNetwork {
	fn from_network(network: PipeNetwork) -> Self {
		let valve_count = network
			.valves
			.iter()
			.filter(|valve| valve.flow_rate > 0)
			.count();
		let mut valves: Vec<_> = (0..valve_count).map(|_| None).collect();
		let mut new_indices = HashMap::with_capacity(valve_count);
		for (index, valve) in network
			.valves
			.iter()
			.enumerate()
			.filter(|(_, valve)| valve.flow_rate > 0)
		{
			let mut distances: Vec<_> = [0].into_iter().cycle().take(valve_count).collect();
			let mut distance_to_start = 0;
			let mut frontier = Vec::from([index]);
			let mut new_frontier = Vec::new();
			let mut visited = HashSet::from([index]);
			for distance in 0.. {
				for old_valve_index in frontier.drain(..) {
					if old_valve_index == 0 {
						distance_to_start = distance;
					}
					let examined_valve = network.valves.get(old_valve_index).unwrap();
					if examined_valve.flow_rate > 0 {
						let indices_len = new_indices.len();
						let new_index = *new_indices.entry(old_valve_index).or_insert(indices_len);
						distances[new_index] = distance;
					}
					for &connection in &examined_valve.connections {
						if visited.insert(connection) {
							new_frontier.push(connection);
						}
					}
				}
				if new_frontier.is_empty() {
					break;
				}
				std::mem::swap(&mut frontier, &mut new_frontier);
			}
			let indices_len = new_indices.len();
			let new_index = *new_indices.entry(index).or_insert(indices_len);
			valves[new_index] = Some(CompiledValve {
				flow_rate: valve.flow_rate,
				distances,
				distance_to_start,
			});
		}
		let valves = valves.into_iter().collect::<Option<_>>().unwrap();
		Self { valves }
	}
	fn open_valves(&self, opened: u64) -> impl Iterator<Item = (usize, &CompiledValve)> {
		self.valves
			.iter()
			.enumerate()
			.filter(move |(index, _)| 1 << *index & opened == 0)
	}
}

#[derive(Debug)]
struct CompiledValve {
	flow_rate: u32,
	distances: Vec<u8>,
	distance_to_start: u8,
}

#[derive(Debug)]
struct CompiledState<const N: usize> {
	opened: u64,
	positions: [usize; N],
	flow_rate: u32,
	time_left: [u8; N],
}

impl<const N: usize> CompiledState<N> {
	fn starting_at(starts: [(usize, &CompiledValve); N], starting_time: u8) -> Self {
		let mut opened = 0;
		let mut flow_rate = 0;
		let mut time_left = [0; N];
		for ((position, valve), time_left) in starts.into_iter().zip(&mut time_left) {
			opened |= 1 << position;
			*time_left = starting_time - valve.distance_to_start - 1;
			flow_rate += valve.flow_rate * *time_left as u32;
		}
		let positions = starts.map(|(position, _)| position);
		Self {
			opened,
			positions,
			flow_rate,
			time_left,
		}
	}
	fn moved_to(&self, position: usize, valve: &CompiledValve, participant: usize) -> Option<Self> {
		let opened = self.opened | 1 << position;
		let mut positions = self.positions;
		positions[participant] = position;
		let time_spent = valve.distances[self.positions[participant]] + 1;
		let mut time_left = self.time_left;
		time_left[participant] = time_left[participant].saturating_sub(time_spent);
		if time_left[participant] == 0 {
			None
		} else {
			let flow_rate = self.flow_rate + valve.flow_rate * time_left[participant] as u32;
			Some(Self {
				opened,
				positions,
				flow_rate,
				time_left,
			})
		}
	}
	fn final_flow_rate_upper_bound(&self, network: &CompiledNetwork) -> u32 {
		let most_time_left = self.time_left.iter().max().unwrap();
		self.flow_rate
			+ network
				.valves
				.iter()
				.enumerate()
				.filter_map(|(index, valve)| {
					(1 << index & self.opened == 0)
						.then_some(valve.flow_rate * *most_time_left as u32)
				})
				.sum::<u32>()
	}
}

struct PipeNetwork {
	valves: Vec<Valve>,
}

impl PipeNetwork {
	fn from_str(str: &str) -> Self {
		let valve_count = str.lines().count();
		let mut valve_indices = HashMap::with_capacity(valve_count);
		valve_indices.insert("AA", 0);
		let mut valves = Vec::with_capacity(valve_count);
		for _ in 0..valve_count {
			valves.push(None);
		}
		for (valve, index) in str
			.lines()
			.map(|line| Valve::from_str_with_map(line, &mut valve_indices))
		{
			valves[index] = Some(valve);
		}
		let valves = valves.into_iter().map(Option::unwrap).collect();
		Self { valves }
	}
}

#[inline]
fn combination_index(a: usize, b: usize, max: usize) -> usize {
	a * max + b - (a + 2) * (a + 1) / 2
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn combinations() {
		assert_eq!(combination_index(0, 1, 5), 0);
		assert_eq!(combination_index(0, 2, 5), 1);
		assert_eq!(combination_index(0, 3, 5), 2);
		assert_eq!(combination_index(0, 4, 5), 3);
		assert_eq!(combination_index(1, 2, 5), 4);
		assert_eq!(combination_index(1, 3, 5), 5);
		assert_eq!(combination_index(1, 4, 5), 6);
		assert_eq!(combination_index(2, 3, 5), 7);
		assert_eq!(combination_index(2, 4, 5), 8);
		assert_eq!(combination_index(3, 4, 5), 9);
	}
}
