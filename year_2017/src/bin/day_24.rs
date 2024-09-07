use std::array;

fn main() {
	shared::print_answers(24, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let (components, symmetrical) = get_components(input);

	let mut frontier = vec![Bridge::new()];
	let mut strongest = 0;
	loop {
		let mut new_frontier = Vec::new();

		for bridge in frontier.drain(..) {
			let mut any_added = false;
			for new_bridge in bridge.extend(&components) {
				new_frontier.push(new_bridge);
				any_added = true;
			}
			if !any_added {
				strongest = strongest.max(bridge.score(&components, &symmetrical)[1])
			}
		}

		if new_frontier.is_empty() {
			break;
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
	strongest
}

fn get_answer_2(input: &str) -> u32 {
	let (components, symmetrical) = get_components(input);

	let mut frontier = vec![Bridge::new()];
	let mut strongest_longest = [0, 0];
	loop {
		let mut new_frontier = Vec::new();

		for bridge in frontier.drain(..) {
			let mut any_added = false;
			for new_bridge in bridge.extend(&components) {
				new_frontier.push(new_bridge);
				any_added = true;
			}
			if !any_added {
				strongest_longest = strongest_longest.max(bridge.score(&components, &symmetrical));
			}
		}

		if new_frontier.is_empty() {
			break;
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
	strongest_longest[1]
}

fn get_components(input: &str) -> (Vec<Component>, Vec<u8>) {
	// It's a reduction in search space to exclude the symmetrical ones from the pathfinding and just add them anywhere possible upon scoring. Only 7 out of 57 components in my input were symmetrical, yet it still reduced my execution time to around 1/8 of the pre-optimized one. Testing reveals it also reduces the number of bridges built to around 1/8 of the pre-optimized number.
	let (symmetrical, components): (Vec<_>, Vec<_>) = input
		.lines()
		.map(Component::from_line)
		.partition(Component::is_symmetrical);
	(
		components,
		symmetrical
			.into_iter()
			.map(|component| component.0[0])
			.collect(),
	)
}

struct Component([u8; 2]);

impl Component {
	fn from_line(line: &str) -> Self {
		let mut nums = line.split('/');
		Self(array::from_fn(|_| nums.next().unwrap().parse().unwrap()))
	}
	fn link(&self, pins: u8) -> Option<u8> {
		self.0
			.iter()
			.position(|num| *num == pins)
			.map(|i| self.0[(i + 1) % 2])
	}
	fn score(&self) -> u32 {
		self.0.iter().map(|n| *n as u32).sum()
	}
	fn is_symmetrical(&self) -> bool {
		self.0[0] == self.0[1]
	}
}

#[derive(Debug, Clone)]
struct Bridge {
	components: Vec<usize>,
	connector: u8,
}

impl Bridge {
	fn new() -> Self {
		Self {
			components: Vec::new(),
			connector: 0,
		}
	}
	fn extend<'l, 'c: 'l>(
		&'l self,
		components: &'c [Component],
	) -> impl Iterator<Item = Bridge> + 'l {
		components
			.iter()
			.enumerate()
			.filter(|(i, _component)| !self.components.contains(i))
			.filter_map(move |(i, component)| {
				component.link(self.connector).map(|connector| {
					let mut bridge = self.clone();
					bridge.components.push(i);
					bridge.connector = connector;
					bridge
				})
			})
	}
	fn score(&self, components: &[Component], symmetrical_components: &[u8]) -> [u32; 2] {
		let mut length = self.components.len() as u32;
		let mut strength = self
			.components
			.iter()
			.map(|i| components[*i].score())
			.sum::<u32>();
		strength += symmetrical_components
			.iter()
			.filter(|symmetrical| {
				self.components
					.iter()
					.any(|i| components[*i].0.contains(symmetrical))
			})
			.map(|symmetrical| {
				length += 1;
				*symmetrical as u32 * 2
			})
			.sum::<u32>();
		[length, strength]
	}
}
