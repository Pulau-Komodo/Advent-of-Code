fn main() {
	shared::print_answers(11, &[get_answers]);
}

#[derive(PartialEq, Clone)]
enum Space {
	Floor,
	Seat(Seat),
}

#[derive(PartialEq, Clone)]
enum Seat {
	Empty,
	Occupied,
}

fn read_seats(input: &str) -> Vec<Vec<Space>> {
	input
		.lines()
		.map(|line| {
			line.chars()
				.map(|char| match char {
					'.' => Space::Floor,
					'L' => Space::Seat(Seat::Empty),
					'#' => Space::Seat(Seat::Occupied),
					_ => panic!("Invalid input character"),
				})
				.collect()
		})
		.collect()
}

fn update_space(
	space: &Space,
	seat_position: (u16, u16),
	seat_state: &[Vec<Space>],
	part_2: bool,
) -> Space {
	match (space, part_2) {
		(Space::Floor, _) => Space::Floor,
		(Space::Seat(seat), false) => Space::Seat(update_seat(seat, seat_position, seat_state)),
		(Space::Seat(seat), true) => Space::Seat(update_seat_2(seat, seat_position, seat_state)),
	}
}

fn update_seat(seat: &Seat, seat_position: (u16, u16), seat_state: &[Vec<Space>]) -> Seat {
	let x_range = seat_position.0.saturating_sub(1)..=seat_position.0 + 1;
	let y_range = seat_position.1.saturating_sub(1)..=seat_position.1 + 1;
	let mut occupied_count = 0;
	for y in y_range {
		for x in x_range.clone() {
			if (x, y) == seat_position {
				continue;
			}
			let nearby_space = seat_state
				.get(y as usize)
				.map(|row| row.get(x as usize))
				.flatten();
			match (seat, nearby_space, occupied_count) {
				(Seat::Empty, Some(Space::Seat(Seat::Occupied)), _)
				| (Seat::Occupied, Some(Space::Seat(Seat::Occupied)), 3) => return Seat::Empty,
				(Seat::Occupied, Some(Space::Seat(Seat::Occupied)), _) => occupied_count += 1,
				_ => {}
			}
		}
	}
	Seat::Occupied
}

fn update_seat_2(seat: &Seat, seat_position: (u16, u16), seat_state: &[Vec<Space>]) -> Seat {
	const DIRECTIONS: [i16; 3] = [-1, 0, 1];
	let mut occupied_count = 0;
	for y in DIRECTIONS {
		for x in DIRECTIONS {
			if (x, y) == (0, 0) {
				continue;
			}
			match (
				seat,
				is_direction_occupied((x, y), seat_position, seat_state),
				occupied_count,
			) {
				(Seat::Empty, true, _) | (Seat::Occupied, true, 4) => return Seat::Empty,
				(Seat::Occupied, true, _) => occupied_count += 1,
				_ => {}
			}
		}
	}
	Seat::Occupied
}

fn is_direction_occupied(
	direction: (i16, i16),
	seat_position: (u16, u16),
	seat_state: &[Vec<Space>],
) -> bool {
	let mut x = seat_position.0 as i16;
	let mut y = seat_position.1 as i16;
	loop {
		x += direction.0;
		y += direction.1;
		match seat_state
			.get(y as usize)
			.map(|row| row.get(x as usize))
			.flatten()
		{
			None | Some(Space::Seat(Seat::Empty)) => return false,
			Some(Space::Seat(Seat::Occupied)) => return true,
			_ => {}
		}
	}
}

fn update_spaces(seat_state: Vec<Vec<Space>>, part_2: bool) -> (Vec<Vec<Space>>, bool) {
	let mut changed = false;
	let new_state = seat_state
		.iter()
		.enumerate()
		.map(|(y, row)| {
			row.iter()
				.enumerate()
				.map(|(x, space)| {
					let new_space = update_space(space, (x as u16, y as u16), &seat_state, part_2);
					if new_space != *space {
						changed = true
					}
					new_space
				})
				.collect()
		})
		.collect();
	(new_state, changed)
}

fn count_occupied_seats(seat_state: &[Vec<Space>]) -> u32 {
	seat_state
		.iter()
		.map(|row| {
			row.iter()
				.filter(|space| matches!(space, Space::Seat(Seat::Occupied)))
				.count() as u32
		})
		.sum()
}

fn find_final_state(mut seat_state: Vec<Vec<Space>>, part_2: bool) -> Vec<Vec<Space>> {
	loop {
		let (new_seat_state, changed) = update_spaces(seat_state, part_2);
		if !changed {
			break new_seat_state;
		}
		seat_state = new_seat_state;
	}
}

fn get_answers(input: &str) -> String {
	let seat_state = read_seats(input);
	let final_state_1 = find_final_state(seat_state.clone(), false);
	let occupied_count_1 = count_occupied_seats(&final_state_1);
	let final_state_2 = find_final_state(seat_state, true);
	let occupied_count_2 = count_occupied_seats(&final_state_2);
	format!("1: {}, 2: {}", occupied_count_1, occupied_count_2)
}
