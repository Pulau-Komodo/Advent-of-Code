fn get_seat_id(seat: &str) -> u32 {
	let (row, column) = seat.split_at(7);
	let row = binary_partition(row, 'B');
	let column = binary_partition(column, 'R');
	row * 8 + column
}

fn binary_partition(input: &str, upper_char: char) -> u32 {
	input
		.chars()
		.rev()
		.enumerate()
		.filter(|(_, char)| char == &upper_char)
		.fold(0, |sum, (index, _)| sum + 2_u32.pow(index as u32))
}

pub fn part_a(input: String) -> String {
	let max = input.lines().map(get_seat_id).max().unwrap();
	format!("{}", max)
}

pub fn part_b(input: String) -> String {
	let mut seat_ids = input.lines().map(get_seat_id).collect::<Vec<u32>>();
	seat_ids.sort_unstable();
	let (_, target) = seat_ids
		.iter()
		.zip(seat_ids.iter().skip(1))
		.find(|(&prev, &next)| prev + 1 != next)
		.unwrap();
	format!("{}", target - 1)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_seat_id() {
		assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
	}
}
