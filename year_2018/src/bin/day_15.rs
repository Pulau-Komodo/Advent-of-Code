use shared::{Grid, Point};

fn main() {
	shared::print_answers(15, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let grid = get_starting_state(input);
	simulate_battle(grid, None).unwrap()
}

fn get_answer_2(input: &str) -> u32 {
	let grid = get_starting_state(input);

	for bonus_attack in 1.. {
		if let Some(victory) = simulate_battle(grid.clone(), Some(bonus_attack)) {
			return victory;
		}
	}
	unreachable!();
}

fn simulate_battle(mut state: Grid<Tile>, added_attack_power: Option<u8>) -> Option<u32> {
	let width = state.width();
	let height = state.height();
	for round in 0.. {
		for mut point in (0..height).flat_map(|y| (0..width).map(move |x| Point::new(x, y))) {
			let Some(mut creature) = state.get_point(point).get_moveable_creature() else {
				continue;
			};
			// Iterate over creatures who haven't had their turn yet.

			if is_battle_over(&state) {
				return Some(score_battle(&state, round));
			}

			if let Some(new_point) = find_point_towards_creature(&state, point, !creature.is_elf) {
				// Move if possible and necessary.
				*state.get_point_mut(point) = Tile::Empty;
				point = new_point;
			}

			if let Some((target_point, mut target_creature)) =
				find_creature_to_attack(&state, point, !creature.is_elf)
			{
				// Attack if possible.
				if target_creature.damage(creature.attack_power(added_attack_power)) {
					if added_attack_power.is_some() && target_creature.is_elf {
						// An elf died.
						return None;
					}
					*state.get_point_mut(target_point) = Tile::Empty;
				} else {
					*state.get_point_mut(target_point) = Tile::Creature(target_creature);
				}
			}

			// Update creature (including putting it in the new place if necessary).
			creature.has_moved = true;
			*state.get_point_mut(point) = Tile::Creature(creature);
		}

		reset_creature_moves(&mut state);
	}
	unreachable!()
}

fn is_battle_over(state: &Grid<Tile>) -> bool {
	let mut any_elves = false;
	let mut any_goblins = false;
	for tile in state.iter() {
		if let Tile::Creature(creature) = tile {
			any_elves |= creature.is_elf;
			any_goblins |= !creature.is_elf;
		}
	}
	!any_elves || !any_goblins
}

fn score_battle(state: &Grid<Tile>, round: u32) -> u32 {
	// state.print_with(|tile| match tile {
	// 	Tile::Creature(creature) if creature.is_elf => 'E',
	// 	Tile::Creature(_) => 'G',
	// 	Tile::Wall => '#',
	// 	Tile::Empty => '.',
	// });
	let total_hp: u32 = state
		.iter()
		.map(|tile| {
			if let Tile::Creature(creature) = tile {
				creature.hitpoints as u32
			} else {
				0
			}
		})
		.sum();
	total_hp * round
}

/// Set all creatures' `has_moved`s to false.
fn reset_creature_moves(state: &mut Grid<Tile>) {
	for tile in state.iter_mut() {
		if let Tile::Creature(creature) = tile {
			creature.has_moved = false;
		}
	}
}

fn find_creature_to_attack(
	state: &Grid<Tile>,
	point: Point<usize>,
	target_elves: bool,
) -> Option<(Point<usize>, Creature)> {
	point
		.orthogonal_neighbours()
		.into_iter()
		.filter_map(|neighbour| {
			state
				.get_point(neighbour)
				.get_creature(target_elves)
				.map(|creature| (neighbour, creature))
		})
		.min_by_key(|(_, creature)| creature.hitpoints)
}

/// Find the start of the shortest path to a creature. Returns `None` if no path is found, or a creature is already adjacent.
fn find_point_towards_creature(
	grid: &Grid<Tile>,
	starting_point: Point<usize>,
	target_elves: bool,
) -> Option<Point<usize>> {
	if starting_point
		.orthogonal_neighbours()
		.into_iter()
		.any(|neighbour| {
			grid.get_point(neighbour)
				.get_creature(target_elves)
				.is_some()
		}) {
		return None;
	}

	let mut frontiers: Vec<_> = starting_point
		.orthogonal_neighbours()
		.into_iter()
		.filter(|neighbour| grid.get_point(*neighbour).is_empty())
		.map(|neighbour| (vec![neighbour], neighbour))
		.collect();
	let mut visited: Vec<_> = frontiers.iter().map(|(_, point)| *point).collect();
	let mut valid_destinations = Vec::new();
	loop {
		let mut new_frontiers = [const { Vec::new() }; 4];

		for index in 0..frontiers.len() {
			let (frontier, direction) = &mut frontiers[index];
			for point in frontier.drain(..) {
				for neighbour in point.orthogonal_neighbours() {
					let tile = grid.get_point(neighbour);
					if tile.is_empty() && !visited.contains(&neighbour) {
						new_frontiers[index].push(neighbour);
						visited.push(neighbour);
					} else if tile.get_creature(target_elves).is_some() {
						valid_destinations.push((point, *direction));
					}
				}
			}
		}

		if !valid_destinations.is_empty() {
			return Some(
				valid_destinations
					.into_iter()
					.min_by_key(|(point, _destination)| (point.y, point.x))
					.unwrap()
					.1,
			);
		}
		if new_frontiers.iter().flatten().count() == 0 {
			return None;
		}
		for ((frontier, _), new_frontier) in frontiers.iter_mut().zip(&mut new_frontiers) {
			std::mem::swap(frontier, new_frontier);
		}
	}
}

const ATTACK_POWER: u8 = 3;
const STARTING_HP: u8 = 200;

#[derive(Debug, Clone, Copy)]
struct Creature {
	is_elf: bool,
	hitpoints: u8,
	has_moved: bool,
}

impl Creature {
	fn attack_power(&self, added_attack_power: Option<u8>) -> u8 {
		match (self.is_elf, added_attack_power) {
			(true, Some(power)) => ATTACK_POWER + power,
			_ => ATTACK_POWER,
		}
	}
	/// Returns whether the creature died.
	fn damage(&mut self, damage: u8) -> bool {
		self.hitpoints = self.hitpoints.saturating_sub(damage);
		self.hitpoints == 0
	}
}

#[derive(Debug, Clone, Copy)]
enum Tile {
	Empty,
	Wall,
	Creature(Creature),
}

impl Tile {
	fn from_byte(byte: u8) -> Self {
		match byte {
			b'.' => Self::Empty,
			b'#' => Self::Wall,
			b'E' => Self::Creature(Creature {
				is_elf: true,
				hitpoints: STARTING_HP,
				has_moved: false,
			}),
			b'G' => Self::Creature(Creature {
				is_elf: false,
				hitpoints: STARTING_HP,
				has_moved: false,
			}),
			_ => panic!(),
		}
	}
	fn get_moveable_creature(&self) -> Option<Creature> {
		match self {
			Self::Creature(creature) if !creature.has_moved => Some(*creature),
			_ => None,
		}
	}
	fn get_creature(&self, is_elf: bool) -> Option<Creature> {
		match self {
			Self::Creature(creature) if creature.is_elf == is_elf => Some(*creature),
			_ => None,
		}
	}
	fn is_empty(&self) -> bool {
		matches!(self, Self::Empty)
	}
}

fn get_starting_state(input: &str) -> Grid<Tile> {
	Grid::new(input.lines().map(|line| line.bytes().map(Tile::from_byte)))
}
