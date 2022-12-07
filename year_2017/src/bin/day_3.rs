use std::fmt::Write;

fn main() {
	shared::print_answers(3, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let target: u32 = input.parse().unwrap();
	find_taxicab_distance_to_center_of_spiral(target)
}

fn get_answer_2(input: &str) -> u32 {
	let target: u32 = input.parse().unwrap();
	NumberSpiral::<11>::new().find(|n| *n > target).unwrap()
}

fn find_taxicab_distance_to_center_of_spiral(number: u32) -> u32 {
	let number = number - 1;
	let (eccentricity, ring_size, ring_position) = find_eccentricity_ring_size_and_position(number);
	let side_size = ring_size / 4;
	let side_position = ring_position % side_size;
	let towards_corner = wrapping_difference(side_position, eccentricity - 1, side_size);
	eccentricity + towards_corner
}

fn find_eccentricity_ring_size_and_position(number: u32) -> (u32, u32, u32) {
	let eccentricity = (0_u32..).find(|n| (*n * 2 + 1).pow(2) > number).unwrap();
	let square_size = (eccentricity * 2 + 1).pow(2);
	let previous_square_size = ((eccentricity - 1) * 2 + 1).pow(2);
	let ring_size = square_size - previous_square_size;
	let ring_position = number - previous_square_size;
	(eccentricity, ring_size, ring_position)
}

fn wrapping_difference(first: u32, second: u32, wrap: u32) -> u32 {
	let difference = first.abs_diff(second);
	if difference > wrap / 2 {
		wrap - difference
	} else {
		difference
	}
}

struct NumberSpiral<const SIZE: usize> {
	grid: [[u32; SIZE]; SIZE],
	current_index: usize,
	coordinates: (usize, usize),
}

impl<const SIZE: usize> NumberSpiral<SIZE> {
	fn new() -> Self {
		let mut grid = [[0; SIZE]; SIZE];
		grid[SIZE / 2][SIZE / 2] = 1;
		Self {
			grid,
			current_index: 1,
			coordinates: (SIZE / 2 + 1, SIZE / 2),
		}
	}
}

impl<const SIZE: usize> Iterator for NumberSpiral<SIZE> {
	type Item = u32;
	fn next(&mut self) -> Option<Self::Item> {
		let sum = [
			(2, 1),
			(2, 0),
			(1, 2),
			(0, 2),
			(0, 1),
			(0, 0),
			(1, 0),
			(2, 2),
		]
		.into_iter()
		.map(|(x, y)| self.grid[y + self.coordinates.1 - 1][x + self.coordinates.0 - 1])
		.sum();
		self.grid[self.coordinates.1][self.coordinates.0] = sum;
		let (_, ring_size, ring_position) =
			find_eccentricity_ring_size_and_position(self.current_index as u32);
		let side_size = ring_size / 4;
		let side = (ring_position + 1) / side_size;
		match side {
			0 => self.coordinates.1 -= 1,
			1 => self.coordinates.0 -= 1,
			2 => self.coordinates.1 += 1,
			3 | 4 => self.coordinates.0 += 1,
			_ => panic!(),
		}
		self.current_index += 1;
		Some(sum)
	}
}

impl<const SIZE: usize> std::fmt::Debug for NumberSpiral<SIZE> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in self.grid {
			f.write_char('\n')?;
			for cell in row {
				f.write_fmt(format_args!("{:3}", cell))?;
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn examples() {
		//assert_eq!(find_taxicab_distance_to_center_of_spiral(1), 0);
		assert_eq!(find_taxicab_distance_to_center_of_spiral(9), 2);
		assert_eq!(find_taxicab_distance_to_center_of_spiral(10), 3);
		assert_eq!(find_taxicab_distance_to_center_of_spiral(12), 3);
		assert_eq!(find_taxicab_distance_to_center_of_spiral(23), 2);
		assert_eq!(find_taxicab_distance_to_center_of_spiral(1024), 31);
	}
}
