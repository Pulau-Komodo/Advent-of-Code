fn main() {
	shared::print_answers(7, &[get_answers]);
}

fn get_answers(input: &str) -> String {
	let now = std::time::Instant::now();
	let target = Identifier::from_str("a");
	let mut gates = parse_gates(input);
	gates.insert(Identifier::from_str("1"), Gate::Signal(1));
	let first = solve(gates, target);
	let first_time = now.elapsed().as_micros();
	let mut gates = parse_gates(input);
	gates.insert(Identifier::from_str("1"), Gate::Signal(1));
	gates.insert(Identifier::from_str("b"), Gate::Signal(first));
	let second = solve(gates, target);
	format!("1: {} ({} μs), 2: {}", first, first_time, second)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Identifier {
	id: u16,
}

impl Identifier {
	fn from_str(str: &str) -> Self {
		let mut bytes = str.as_bytes().to_vec();
		if bytes.len() < 2 {
			bytes.insert(0, 0)
		}
		Self {
			id: u16::from_be_bytes([bytes[0], bytes[1]]),
		}
	}
	fn _to_string(&self) -> String {
		let bytes = self.id.to_be_bytes();
		let mut string = String::new();
		string.push(bytes[0].into());
		string.push(bytes[1].into());
		string
	}
}

type GateMap = std::collections::HashMap<Identifier, Gate>;

#[derive(Clone, Copy, Debug)]
enum Gate {
	Signal(u16),
	Simple(Identifier),
	LShift(Identifier, u8),
	RShift(Identifier, u8),
	And(Identifier, Identifier),
	Or(Identifier, Identifier),
	Not(Identifier),
}

impl Gate {
	fn execute(&self, map: &GateMap) -> u16 {
		let execute = |id: &Identifier| map.get(id).map(|gate| gate.execute(map)).unwrap();
		use Gate::*;
		match self {
			Signal(signal) => *signal,
			Simple(id) => execute(id),
			LShift(id, shift) => execute(id) << shift,
			RShift(id, shift) => execute(id) >> shift,
			And(id_1, id_2) => execute(id_1) & execute(id_2),
			Or(id_1, id_2) => execute(id_1) | execute(id_2),
			Not(id) => !execute(id),
		}
	}
	fn gets_signal(&self, map: &GateMap) -> bool {
		use Gate::*;
		let is_signal = |id: &Identifier| matches!(map.get(id).unwrap(), Signal(_));
		match self {
			Signal(_) => false,
			Simple(id) => is_signal(id),
			LShift(id, _) => is_signal(id),
			RShift(id, _) => is_signal(id),
			And(id_1, id_2) => is_signal(id_1) && is_signal(id_2),
			Or(id_1, id_2) => is_signal(id_1) && is_signal(id_2),
			Not(id) => is_signal(id),
		}
	}
}

fn parse_gates(gates: &str) -> GateMap {
	gates
		.lines()
		.map(|gate| {
			let (gate, id) = gate.split_once(" -> ").unwrap();
			let mut parts = gate.split(' ');
			let gate = match (parts.next(), parts.next(), parts.next()) {
				(Some(item), None, None) => {
					if let Ok(number) = item.parse() {
						Gate::Signal(number)
					} else {
						Gate::Simple(Identifier::from_str(item))
					}
				}
				(Some(id), Some("LSHIFT"), Some(shift)) => {
					Gate::LShift(Identifier::from_str(id), shift.parse().unwrap())
				}
				(Some(id), Some("RSHIFT"), Some(shift)) => {
					Gate::RShift(Identifier::from_str(id), shift.parse().unwrap())
				}
				(Some(id_1), Some("AND"), Some(id_2)) => {
					Gate::And(Identifier::from_str(id_1), Identifier::from_str(id_2))
				}
				(Some(id_1), Some("OR"), Some(id_2)) => {
					Gate::Or(Identifier::from_str(id_1), Identifier::from_str(id_2))
				}
				(Some("NOT"), Some(id), None) => Gate::Not(Identifier::from_str(id)),
				_ => panic!("Invalid input"),
			};
			(Identifier::from_str(id), gate)
		})
		.collect()
}

fn solve(mut gates: GateMap, target: Identifier) -> u16 {
	loop {
		let signalled_gates: Vec<(Identifier, Gate)> = gates
			.iter()
			.filter(|(_, gate)| gate.gets_signal(&gates))
			.map(|(&identifier, &gate)| (identifier, gate))
			.collect();
		for (id, gate) in signalled_gates {
			let signal = gate.execute(&gates);
			if id == target {
				return signal;
			}
			let mut signal = Gate::Signal(signal);
			gates
				.entry(id)
				.and_modify(|gate| std::mem::swap(gate, &mut signal));
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn endian() {
		let id = Identifier::from_str("az");
		print!("{:04x}", id.id);
		println!(" - {}", id._to_string());
		let id = Identifier::from_str("za");
		print!("{:04x}", id.id);
		println!(" - {}", id._to_string());
		let id = Identifier::from_str("a");
		print!("{:04x}", id.id);
		println!(" - {}", id._to_string());
		let id = Identifier { id: 49 };
		print!("{:04x}", id.id);
		println!(" - {}", id._to_string());
		let id = Identifier { id: 97 };
		print!("{:04x}", id.id);
		println!(" - {}", id._to_string());
	}
}
