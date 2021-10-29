fn main() {
	shared::print_answers(15, &[get_answers]);
}

type NumberMap = std::collections::HashMap<u64, u32>;

fn process_input(input: &str) -> (NumberMap, u64) {
	let mut map = std::collections::HashMap::with_capacity(2020);
	let mut numbers = input
		.split(',')
		.map(|number| number.parse().unwrap())
		.collect::<Vec<u64>>();
	let last_number = numbers.pop().unwrap();
	map.extend(
		numbers
			.into_iter()
			.enumerate()
			.map(|(row, number)| (number, row as u32)),
	);
	(map, last_number)
}

fn get_nth_number(mut map: NumberMap, mut last_number: u64, n: u32) -> u64 {
	let mut turn = map.len() as u32;
	while turn < n - 1 {
		let new_number = map
			.get(&last_number)
			.map(|number| turn as u64 - *number as u64)
			.unwrap_or(0);
		map.insert(last_number, turn);
		last_number = new_number;
		turn += 1;
	}
	last_number
}

fn get_answers(input: &str) -> String {
	let (mut map, last_number) = process_input(input);
	let number_2020 = get_nth_number(map.clone(), last_number, 2020);
	map.reserve(30_000_000 - 2020);
	let number_3e7 = get_nth_number(map, last_number, 30_000_000);
	format!("1: {}, 2: {}", number_2020, number_3e7)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn sample_input() {
		let (map, last) = process_input("0,3,6");
		assert_eq!(get_nth_number(map, last, 2020), 436);
		let (map, last) = process_input("1,3,2");
		assert_eq!(get_nth_number(map, last, 2020), 1);
		let (map, last) = process_input("2,1,3");
		assert_eq!(get_nth_number(map, last, 2020), 10);
		let (map, last) = process_input("1,2,3");
		assert_eq!(get_nth_number(map, last, 2020), 27);
		let (map, last) = process_input("2,3,1");
		assert_eq!(get_nth_number(map, last, 2020), 78);
		let (map, last) = process_input("3,2,1");
		assert_eq!(get_nth_number(map, last, 2020), 438);
		let (map, last) = process_input("3,1,2");
		assert_eq!(get_nth_number(map, last, 2020), 1836);
	}
	#[test]
	fn sample_input_2() {
		let (map, last) = process_input("0,3,6");
		assert_eq!(get_nth_number(map, last, 30_000_000), 175594);
		/*let (map, last) = process_input("1,3,2");
		assert_eq!(get_nth_number(map, last, 30_000_000), 2578);
		let (map, last) = process_input("2,1,3");
		assert_eq!(get_nth_number(map, last, 30_000_000), 3544142);
		let (map, last) = process_input("1,2,3");
		assert_eq!(get_nth_number(map, last, 30_000_000), 261214);
		let (map, last) = process_input("2,3,1");
		assert_eq!(get_nth_number(map, last, 30_000_000), 6895259);
		let (map, last) = process_input("3,2,1");
		assert_eq!(get_nth_number(map, last, 30_000_000), 18);
		let (map, last) = process_input("3,1,2");
		assert_eq!(get_nth_number(map, last, 30_000_000), 362);*/
	}
}
