use std::collections::HashMap;

fn main() {
	shared::print_answers(20, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let mut modules = make_module_map(input);
	let mut low_pulses = 0;
	let mut high_pulses = 0;
	for _ in 0..1000 {
		let mut frontier = vec![("button", "broadcaster", false)];
		loop {
			let mut new_frontier = Vec::new();
			for (sender, target, pulse) in frontier.drain(..) {
				if pulse {
					high_pulses += 1;
				} else {
					low_pulses += 1;
				}
				let Some(module) = modules.get_mut(target) else {
					continue;
				};
				if let Some(pulse) = module.kind.process_pulse(pulse, sender) {
					for &new_target in &module.targets {
						new_frontier.push((target, new_target, pulse));
					}
				}
			}
			if new_frontier.is_empty() {
				break;
			}
			std::mem::swap(&mut frontier, &mut new_frontier);
		}
	}
	low_pulses as u64 * high_pulses as u64
}

// This solution ends up hardcoding a lot of facts about the input, not specified by the problem description. That seems to be the intent today, though. Though I probably hardcoded a bit in excess of necessity.
// Particularly, it hardcodes that the broadcaster's signal is split to four separate "computers", that all send their final outcome to "ln", which sends the final signal only if all input signals were high. It hardcodes that each computer resets fully after such a signal (or at least that the outcome is identical each cycle) and that each period is a prime number. It's also a fact that I'm depending upon that each high pulse to "ln" is immediately followed by a low pulse, but not before the other computers send their high pulses.
fn get_answer_2(input: &str) -> u64 {
	let mut modules = make_module_map(input);
	let mut tg = HashMap::from([("ln", None), ("db", None), ("vq", None), ("tf", None)]); // Hardcoded: "tg" is the penultimate module, and "ln", "db", "vq", "tf" are its senders.
	for cycle in 1.. {
		let mut frontier = vec![("button", "broadcaster", false)];
		loop {
			let mut new_frontier = Vec::new();
			for (sender, target, pulse) in frontier.drain(..) {
				if target == "tg" && pulse {
					*tg.get_mut(sender)
						.unwrap_or_else(|| panic!("Could not find sender {sender}")) = Some(cycle);
					if let Ok(value) = tg
						.values()
						.try_fold(1, |acc, period| period.ok_or(()).map(|val| acc * val))
					{
						return value;
					}
				}
				if target == "rx" && !pulse {
					return cycle;
				}
				let Some(module) = modules.get_mut(target) else {
					continue;
				};
				if let Some(pulse) = module.kind.process_pulse(pulse, sender) {
					for &new_target in &module.targets {
						new_frontier.push((target, new_target, pulse));
					}
				}
			}
			if new_frontier.is_empty() {
				break;
			}
			std::mem::swap(&mut frontier, &mut new_frontier);
		}
	}
	unreachable!()
}

enum ModuleKind<'l> {
	Broadcaster,
	FlipFlop(bool),
	Conjunction(HashMap<&'l str, bool>),
}

impl<'l> ModuleKind<'l> {
	fn from_byte(byte: u8) -> Self {
		match byte {
			b'%' => Self::FlipFlop(false),
			b'&' => Self::Conjunction(HashMap::new()),
			_ => Self::Broadcaster,
		}
	}
	fn insert_targeters(&mut self, targeters: &[&'l str]) {
		if let Self::Conjunction(history) = self {
			for targeter in targeters {
				history.insert(targeter, false);
			}
		}
	}
	fn process_pulse(&mut self, pulse: bool, sender: &'l str) -> Option<bool> {
		match self {
			Self::Broadcaster => Some(pulse),
			Self::FlipFlop(state) => {
				if !pulse {
					*state = !*state;
					Some(*state)
				} else {
					None
				}
			}
			Self::Conjunction(history) => {
				history.insert(sender, pulse);
				Some(history.values().copied().any(|pulse| !pulse))
			}
		}
	}
}

struct Module<'l> {
	kind: ModuleKind<'l>,
	targets: Vec<&'l str>,
}

fn make_module_map(input: &str) -> HashMap<&str, Module> {
	let mut targeter_map = HashMap::<&str, Vec<_>>::new();
	let mut map: HashMap<_, _> = input
		.lines()
		.map(|line| {
			let kind = ModuleKind::from_byte(line.as_bytes()[0]);
			let (name, targets) = line.split_once(" -> ").unwrap();
			let name = name.trim_start_matches(['%', '&']);
			let targets = targets.split(", ").collect();
			for &target in &targets {
				targeter_map.entry(target).or_default().push(name);
			}
			let module = Module { kind, targets };
			(name, module)
		})
		.collect();
	for (target, targeters) in targeter_map {
		if let Some(module) = map.get_mut(target) {
			module.kind.insert_targeters(&targeters);
		}
	}
	map
}
