fn parse_input(input: String) -> (i32, Vec<(usize, i32)>) {
	let mut input = input.lines();
	let time = input.next().unwrap().parse().unwrap();
	let buses = input
		.next()
		.unwrap()
		.split(',')
		.enumerate()
		.filter_map(|(index, bus)| {
			if bus == "x" {
				None
			} else {
				Some((index, bus.parse().unwrap()))
			}
		})
		.collect();
	(time, buses)
}

fn find_first_cool_timestamp(buses: &Vec<(usize, i32)>) -> u64 {
	let mut buses_iterator = buses.iter();
	let (first_timeslot, first_interval) = buses_iterator.next().unwrap();
	let mut starting_point = *first_timeslot as u64;
	let mut cycle = *first_interval as u64;
	for (target, interval) in buses_iterator {
		let target = *target as u64;
		let interval = *interval as u64;
		let cycles = how_many_cycles_to_target(starting_point % interval, target % interval, cycle % interval, interval);
		starting_point += cycles * cycle;
		cycle *= interval;
	}
	cycle - starting_point
}

fn how_many_cycles_to_target(start: u64, target: u64, step: u64, modulo: u64) -> u64 {
	let mut cycles = 0;
	let mut current = start;
	loop {
		if current == target {
			break cycles
		}
		cycles += 1;
		current = (current + step) % modulo;
	}
}

pub fn get_answers(input: String) -> String {
	let (time, buses) = parse_input(input);
	let (next_bus, wait_time) = buses
		.iter()
		.map(|(_, bus)| (bus, bus - time % bus))
		.min_by_key(|(_, wait)| *wait)
		.unwrap();
	let answer_1 = wait_time * next_bus;
	let answer_2 = find_first_cool_timestamp(&buses);
	format!("1: {}, 2: {}", answer_1, answer_2)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn bag_rule() {
		let input = String::from("939\n7,13,x,x,59,x,31,19");
		assert_eq!(get_answers(input), String::from("1: 295, 2: 1068781"));
	}
	#[test]
	fn cycle_counter() {
		assert_eq!(how_many_cycles_to_target(4, 0, 3, 7), 1);
		assert_eq!(how_many_cycles_to_target(63, 1, 10, 13), 12);
	}
}
