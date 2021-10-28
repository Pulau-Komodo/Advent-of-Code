fn main() {
	year_2020::print_answers(13, &[get_answers]);
}

fn parse_input(input: &str) -> (i32, Vec<(usize, i32)>) {
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

fn find_first_cool_timestamp(buses: &[(usize, i32)]) -> u64 {
	let mut cycle = 1;
	let mut starting_point = 0;
	for (reverse_target, interval) in buses {
		let interval = *interval as u64;
		let target = interval - *reverse_target as u64 % interval;
		let cycles = how_many_cycles_to_target(
			starting_point % interval,
			target % interval,
			cycle % interval,
			interval,
		);
		starting_point += cycles * cycle;
		cycle *= interval;
	}
	starting_point
}

fn how_many_cycles_to_target(start: u64, target: u64, step: u64, modulo: u64) -> u64 {
	let mut cycles = 0;
	let mut current = start;
	loop {
		if current == target {
			break cycles;
		}
		cycles += 1;
		current = (current + step) % modulo;
	}
}

fn get_answers(input: &str) -> String {
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
	fn sample_input() {
		let input = "939\n7,13,x,x,59,x,31,19";
		assert_eq!(get_answers(input), String::from("1: 295, 2: 1068781"));
	}
	#[test]
	fn non_zero_first() {
		let input = "0\nx,x,x,x,x,x,x,x,x,x,31,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,7";
		assert_eq!(get_answers(input), String::from("1: 49, 2: 145"));
	}
	#[test]
	fn full_input() {
		let input = year_2020::read_file(13);
		assert_eq!(
			get_answers(&input),
			String::from("1: 2298, 2: 783685719679632")
		);
	}
	#[test]
	fn cycle_counter() {
		assert_eq!(how_many_cycles_to_target(4, 0, 3, 7), 1);
		assert_eq!(how_many_cycles_to_target(63, 1, 10, 13), 12);
	}
}
