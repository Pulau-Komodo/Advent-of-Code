use std::{collections::HashSet, fmt::Debug};

fn main() {
	shared::print_answers(11, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let initial_state = State::from_str(input);
	let mut visited = HashSet::from([initial_state.clone()]);
	//let mut path_to_visited = std::collections::HashMap::from([(initial_state.clone(), vec![initial_state.clone()])]);
	let mut frontier = vec![initial_state];
	let mut steps = 0;
	'outer: loop {
		steps += 1;
		let mut new_frontier = Vec::new();
		for state in frontier.drain(..) {
			for new_state in state.possible_moves() {
				if new_state.has_completed() {
					//dbg!(path_to_visited.get(&state).unwrap());
					break 'outer steps;
				}
				if !visited.contains(&new_state) {
					visited.insert(new_state.clone());
					//let mut new_path = path_to_visited.get(&state).unwrap().clone();
					//new_path.push(new_state.clone());
					//path_to_visited.insert(new_state.clone(), new_path);
					new_frontier.push(new_state);
				}
			}
		}
		frontier.extend(new_frontier);
		if frontier.is_empty() {
			panic!("No moves left after step count: {steps}.");
		}
	}
}

fn get_answer_2(input: &str) -> u32 {
	let mut initial_state = State::from_str(input);
	initial_state.add_extra_items();
	let mut visited = HashSet::from([initial_state.clone()]);
	let mut frontier = vec![initial_state];
	let mut steps = 0;
	'outer: loop {
		steps += 1;
		let mut new_frontier = Vec::new();
		for state in frontier.drain(..) {
			for new_state in state.possible_moves() {
				if new_state.has_completed() {
					break 'outer steps;
				}
				if !visited.contains(&new_state) {
					visited.insert(new_state.clone());
					new_frontier.push(new_state);
				}
			}
		}
		frontier.extend(new_frontier);
		if frontier.is_empty() {
			panic!("No moves left after step count: {steps}.");
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Item {
	Chip(u8),
	Generator(u8),
}

impl Debug for Item {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Generator(n) => f.write_fmt(format_args!("Generator({n})")),
			Self::Chip(n) => f.write_fmt(format_args!("Chip({n})")),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
	elevator_floor: u8,
	floors: [Vec<Item>; 4],
}

impl State {
	fn from_str(str: &str) -> Self {
		let mut floors = [(); 4].map(|_| Vec::new());
		let mut kinds = Vec::new();
		for (index, line) in str.lines().enumerate() {
			let mut previous_word = "";
			for word in line.split_whitespace() {
				if word.starts_with("generator") {
					let position = if let Some(position) =
						kinds.iter().position(|kind| *kind == previous_word)
					{
						position
					} else {
						kinds.push(previous_word);
						kinds.len() - 1
					};
					floors[index].push(Item::Generator(position as u8));
				} else if word.starts_with("microchip") {
					let (new_kind, _) = previous_word.split_once('-').unwrap();
					let position =
						if let Some(position) = kinds.iter().position(|kind| *kind == new_kind) {
							position
						} else {
							kinds.push(new_kind);
							kinds.len() - 1
						};
					floors[index].push(Item::Chip(position as u8));
				} else {
					previous_word = word;
				}
			}
		}
		for floor in &mut floors {
			floor.sort();
		}
		Self {
			elevator_floor: 0,
			floors,
		}
	}
	fn add_extra_items(&mut self) {
		self.floors[0].extend([
			Item::Generator(u8::MAX - 1),
			Item::Chip(u8::MAX - 1),
			Item::Generator(u8::MAX),
			Item::Chip(u8::MAX),
		]);
		self.floors[0].sort();
	}
	fn current_floor(&self) -> &[Item] {
		&self.floors[self.elevator_floor as usize]
	}
	fn possible_moves(&self) -> impl Iterator<Item = Self> + '_ {
		(self.elevator_floor.saturating_sub(1)..=(self.elevator_floor + 1).min(3))
			.filter(|&new_floor| new_floor != self.elevator_floor)
			.flat_map(move |new_floor| {
				(0..self.current_floor().len()).flat_map(move |item| {
					(item..self.current_floor().len())
						.map(move |other_item| (new_floor, item, other_item))
				})
			})
			.filter_map(|(new_floor, item, other_item)| {
				let mut floors = self.floors.clone();
				if item == other_item {
					let item = floors[self.elevator_floor as usize].remove(item);
					floors[new_floor as usize].push(item);
				} else {
					let other_item = floors[self.elevator_floor as usize].remove(other_item);
					let item = floors[self.elevator_floor as usize].remove(item);
					floors[new_floor as usize].push(item);
					floors[new_floor as usize].push(other_item);
				}
				floors[new_floor as usize].sort();
				(is_legal_floor(&floors[new_floor as usize])
					&& is_legal_floor(&floors[self.elevator_floor as usize]))
				.then_some(State {
					elevator_floor: new_floor,
					floors,
				})
			})
	}
	fn has_completed(&self) -> bool {
		self.floors[0..3].iter().all(|floor| floor.is_empty())
	}
}

fn is_legal_floor(floor: &[Item]) -> bool {
	let mut unmatched_chips = HashSet::new();
	let mut any_generator = false;
	for item in floor {
		match item {
			Item::Chip(id) => {
				unmatched_chips.insert(id);
			}
			Item::Generator(id) => {
				any_generator = true;
				unmatched_chips.remove(id);
			}
		}
	}
	!any_generator || unmatched_chips.is_empty()
}
