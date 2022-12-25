use std::fmt::Debug;

use shared::{div_ceil, split_number};

fn main() {
	shared::print_answers(19, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u16 {
	input
		.lines()
		.map(Blueprint::from_str)
		.enumerate()
		.map(|(index, blueprint)| blueprint.find_optimal_yield(24) * (index as u16 + 1))
		.sum()
}

fn get_answer_2(input: &str) -> u16 {
	input
		.lines()
		.take(3)
		.map(Blueprint::from_str)
		.map(|blueprint| blueprint.find_optimal_yield(32))
		.product()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Blueprint {
	ore: u16,
	clay: u16,
	obsidian: (u16, u16),
	geode: (u16, u16),
}

impl Blueprint {
	fn from_str(str: &str) -> Self {
		let (_, rest) = str["Blueprint 1".len()..].split_once(':').unwrap();
		let (ore, rest) = split_number(&rest[" Each ore robot costs ".len()..]);
		let (clay, rest) = split_number(&rest[" ore. Each clay robot costs ".len()..]);
		let (obsidian_ore, rest) = split_number(&rest[" ore. Each obsidian robot costs ".len()..]);
		let (obsidian_clay, rest) = split_number(&rest[" ore and ".len()..]);
		let (geode_ore, rest) = split_number(&rest[" clay. Each geode robot costs ".len()..]);
		let (geode_obsidian, _) = split_number(&rest[" ore and ".len()..]);
		Self {
			ore,
			clay,
			obsidian: (obsidian_ore, obsidian_clay),
			geode: (geode_ore, geode_obsidian),
		}
	}
	fn max_ore_cost(&self) -> u16 {
		self.ore
			.max(self.clay)
			.max(self.obsidian.0)
			.max(self.geode.0)
	}
	fn find_optimal_yield(&self, minutes: u16) -> u16 {
		let start = State::new(self, minutes);
		let mut frontier = Vec::from([start]);
		let mut new_frontier = Vec::new();
		let mut highest_yield = 0;
		loop {
			for state in frontier.drain(..) {
				for possibility in state.possibilities() {
					if possibility.outcome_upper_bound() > highest_yield {
						highest_yield = highest_yield.max(possibility.geodes);
						new_frontier.push(possibility);
					}
				}
			}
			if new_frontier.is_empty() {
				break;
			}
			std::mem::swap(&mut frontier, &mut new_frontier);
		}
		highest_yield
	}
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct State<'l> {
	ore_bots: u16,
	clay_bots: u16,
	obsidian_bots: u16,
	ore: u16,
	clay: u16,
	obsidian: u16,
	geodes: u16,
	time_left: u16,
	costs: &'l Blueprint,
}

impl<'l> State<'l> {
	fn new(blueprint: &'l Blueprint, minutes: u16) -> Self {
		Self {
			ore_bots: 1,
			clay_bots: 0,
			obsidian_bots: 0,
			ore: 0,
			clay: 0,
			obsidian: 0,
			geodes: 0,
			time_left: minutes,
			costs: blueprint,
		}
	}
	fn progress(&mut self, time: u16) {
		self.ore += self.ore_bots * time;
		self.clay += self.clay_bots * time;
		self.obsidian += self.obsidian_bots * time;
		self.time_left -= time;
	}
	fn try_build_ore_bot(mut self) -> Option<Self> {
		let ore_short = self.costs.ore.saturating_sub(self.ore);
		let wait_time = wait_time(ore_short, self.ore_bots)?;
		(wait_time < self.time_left && !self.is_ore_plenty()).then(|| {
			self.progress(wait_time + 1);
			self.ore -= self.costs.ore;
			self.ore_bots += 1;
			self
		})
	}
	fn try_build_clay_bot(mut self) -> Option<Self> {
		let ore_short = self.costs.clay.saturating_sub(self.ore);
		let wait_time = wait_time(ore_short, self.ore_bots)?;
		(wait_time < self.time_left && !self.is_clay_plenty()).then(|| {
			self.progress(wait_time + 1);
			self.ore -= self.costs.clay;
			self.clay_bots += 1;
			self
		})
	}
	fn try_build_obsidian_bot(mut self) -> Option<Self> {
		let ore_short = self.costs.obsidian.0.saturating_sub(self.ore);
		let clay_short = self.costs.obsidian.1.saturating_sub(self.clay);
		let wait_time =
			wait_time(ore_short, self.ore_bots)?.max(wait_time(clay_short, self.clay_bots)?);
		(wait_time < self.time_left && !self.is_obsidian_plenty()).then(|| {
			self.progress(wait_time + 1);
			self.clay -= self.costs.obsidian.1;
			self.ore -= self.costs.obsidian.0;
			self.obsidian_bots += 1;
			self
		})
	}
	fn try_build_geode_bot(mut self) -> Option<Self> {
		let ore_short = self.costs.geode.0.saturating_sub(self.ore);
		let obsidian_short = self.costs.geode.1.saturating_sub(self.obsidian);
		let wait_time = wait_time(ore_short, self.ore_bots)?
			.max(wait_time(obsidian_short, self.obsidian_bots)?);
		(wait_time < self.time_left).then(|| {
			self.progress(wait_time + 1);
			self.obsidian -= self.costs.geode.1;
			self.ore -= self.costs.geode.0;
			self.geodes += self.time_left;
			self
		})
	}
	fn is_ore_plenty(&self) -> bool {
		self.ore_bots >= self.costs.max_ore_cost()
	}
	fn is_clay_plenty(&self) -> bool {
		self.clay_bots >= self.costs.obsidian.1
	}
	fn is_obsidian_plenty(&self) -> bool {
		self.obsidian_bots >= self.costs.geode.1
	}
	fn possibilities(self) -> impl Iterator<Item = State<'l>> {
		self.try_build_geode_bot()
			.into_iter()
			.chain(self.try_build_obsidian_bot())
			.chain(self.try_build_clay_bot())
			.chain(self.try_build_ore_bot())
	}
	fn outcome_upper_bound(&self) -> u16 {
		self.geodes + self.time_left.saturating_sub(1) * self.time_left / 2
	}
}

impl<'l> Debug for State<'l> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("State")
			.field("ore_bots", &self.ore_bots)
			.field("clay_bots", &self.clay_bots)
			.field("obsidian_bots", &self.obsidian_bots)
			.field("ore", &self.ore)
			.field("clay", &self.clay)
			.field("obsidian", &self.obsidian)
			.field("geodes", &self.geodes)
			.field("time_left", &self.time_left)
			.finish()
	}
}

fn wait_time(short: u16, income: u16) -> Option<u16> {
	if short == 0 {
		Some(0)
	} else if income == 0 {
		None
	} else {
		Some(div_ceil(short, income))
	}
}
