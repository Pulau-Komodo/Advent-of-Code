use shared::{Grid, Point};

fn main() {
	shared::print_answers(21, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	const STEP_COUNT: u64 = 64;
	let start = input
		.lines()
		.enumerate()
		.flat_map(|(y, line)| {
			line.char_indices()
				.map(move |(x, char)| (Point::new(x + 1, y + 1), char))
		})
		.find_map(|(point, char)| (char == 'S').then_some(point))
		.unwrap();
	let grid = Grid::with_margin(
		input.lines().map(|line| line.chars().map(Tile::from_char)),
		Tile::Rock,
	);
	find_and_filter_plots(&grid, start, STEP_COUNT)
}

// Hardcoded: there is a straight path to the top, bottom, left and right sides of the repeating field, and no reachable plot takes more than (width + height) / 2 to reach.
// Also: start is in the middle.
// Also, field size is an odd number.

// Example grid of fields:
//    _E_
//   _.O._
//  _.OXO._
// _.OXOXO._
// EOXOXOXOE
// _.OXOXO._
//  _.OXO._
//   _.O._
//    _E_
// Saturated distance: 3
// Even fields (X): 9
// Odd fields (O): 16
// Inner diagonals (.) are in a straight diagonal line from the center: indeed (which also makes them even), and there are 3 * 4 = 12
// Outer diagonals (_): (3 + 1) * 4 = 16

// Example plots landed on:
//      11-11
//      1-1-1
//      -1-1-
//     -1-1-1-
//    - -1-1- -
// 11-1-0-0-0-1-11
// 1-1-1-0-0-1-1-1
// -1-1-0-S-0-1-1-
// 1-1-1-0-0-1-1-1
// 11-1-0-0-0-1-11
//    - -1-1- -
//     -1-1-1-
//      -1-1-
//      1-1-1
//      11-11
// 7 steps
// 7 - 5 / 2 - 1 = 4 past edge
// 2 in 4 outer corners
// 0 inner corners
// outcome: 64

fn get_answer_2(input: &str) -> u64 {
	// const STEP_COUNT: u64 = 7;
	const STEP_COUNT: u64 = 26_501_365;
	let grid = Grid::with_margin(
		input.lines().map(|line| line.chars().map(Tile::from_char)),
		Tile::Rock,
	);
	let field_height = grid.height() as u64 - 2;
	let field_width = grid.width() as u64 - 2;
	assert_eq!(field_height, field_width); // Hardcoded that the repeating field is square. Seemed to be true so I thought I'd make things easier for myself.

	let field_size = field_height;
	let steps_past_edge = STEP_COUNT - field_size / 2 - 1; // Cancel the steps happening inside the first field.
	let steps_past_corner = STEP_COUNT - field_size - 1; // Cancel the steps happening inside the first field.
	let saturated_distance = steps_past_corner / field_size; // How many fields away will all be saturated anyway.

	let (even_distance_fields, odd_distance_fields) = count_saturated_fields(saturated_distance);

	let visited = find_visited_plots(
		&grid,
		Point {
			x: grid.width() / 2,
			y: grid.height() / 2,
		},
		field_size,
	);
	let odd_field_reachable = filter_visited_plots(&visited, STEP_COUNT % 2 != 0);
	let even_field_reachable = filter_visited_plots(&visited, STEP_COUNT % 2 == 0);

	let steps_left_at_extremes = steps_past_edge % field_size;
	let right_middle = Point::new(grid.width() - 2, grid.height() / 2);
	let left_middle = Point::new(1, grid.height() / 2);
	let bottom_middle = Point::new(grid.width() / 2, grid.height() - 2);
	let top_middle = Point::new(grid.width() / 2, 1);
	let extremes = [right_middle, left_middle, bottom_middle, top_middle]
		.into_iter()
		.map(|start| find_and_filter_plots(&grid, start, steps_left_at_extremes))
		.sum::<u64>();

	let inner_diagonals_are_straight = saturated_distance % 2 == 1;
	let steps_left_at_inner_diagonals = if inner_diagonals_are_straight {
		steps_past_corner % (field_size * 2)
	} else {
		steps_past_corner % field_size
	};
	let steps_left_at_outer_diagonals = if inner_diagonals_are_straight {
		steps_past_corner % field_size
	} else {
		steps_past_corner % (field_size * 2)
	};
	let bottom_right = Point::new(grid.width() - 2, grid.height() - 2);
	let bottom_left = Point::new(1, grid.height() - 2);
	let top_right = Point::new(grid.width() - 2, 1);
	let top_left = Point::new(1, 1);

	let diagonals = [bottom_right, bottom_left, top_right, top_left]
		.into_iter()
		.map(|corner| {
			[steps_left_at_inner_diagonals, steps_left_at_outer_diagonals]
				.into_iter()
				.zip([saturated_distance, saturated_distance + 1])
				.map(|(steps_left, diagonal_count)| {
					find_and_filter_plots(&grid, corner, steps_left) * diagonal_count
				})
				.sum::<u64>()
		})
		.sum::<u64>();

	even_distance_fields * even_field_reachable
		+ odd_distance_fields * odd_field_reachable
		+ extremes
		+ diagonals
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
	Plot,
	Rock,
}

impl Tile {
	fn from_char(char: char) -> Self {
		if char == '#' {
			Self::Rock
		} else {
			Self::Plot
		}
	}
}

fn find_and_filter_plots(grid: &Grid<Tile>, start: Point<usize>, steps: u64) -> u64 {
	let visited = find_visited_plots(grid, start, steps);
	filter_visited_plots(&visited, (start.x + start.y + steps as usize) % 2 == 0)
}

fn find_visited_plots(grid: &Grid<Tile>, start: Point<usize>, steps: u64) -> Grid<bool> {
	let mut visited = Grid::empty(grid.width(), grid.height(), false);
	let mut frontier = vec![start];
	for _step in 0..steps {
		let mut new_frontier = Vec::new();
		for position in frontier.drain(..) {
			for neighbour in position.orthogonal_neighbours() {
				if grid.get_point(neighbour) == Tile::Plot && !visited.get_point(neighbour) {
					new_frontier.push(neighbour);
					*visited.get_point_mut(neighbour) = true;
				}
			}
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
	visited
}

fn filter_visited_plots(visited: &Grid<bool>, evens: bool) -> u64 {
	visited
		.iter_with_points::<usize>()
		.filter_map(|(point, visited)| visited.then_some(point))
		.filter(|point| ((point.x + point.y) % 2 == 0) == evens)
		.count() as u64
}

/// `field_steps` is the number of fields past the starting field in any straight direction.
/// (even distance fields, odd distance fields)
fn count_saturated_fields(field_steps: u64) -> (u64, u64) {
	let half_field_steps = field_steps / 2;
	let even_distance_tiles = 1 + half_field_steps * (half_field_steps + 1) * 4;
	let odd_distance_tiles = (field_steps - half_field_steps).pow(2) * 4;
	let saturated_width = 1 + field_steps * 2;
	let saturated_tiles = (saturated_width - 1) * field_steps + saturated_width;

	assert_eq!(saturated_tiles, even_distance_tiles + odd_distance_tiles); // Sanity check

	(even_distance_tiles, odd_distance_tiles)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_field_counting() {
		assert_eq!(count_saturated_fields(0), (1, 0));
		assert_eq!(count_saturated_fields(1), (1, 4));
		assert_eq!(count_saturated_fields(2), (9, 4));
		assert_eq!(count_saturated_fields(3), (9, 16));
		assert_eq!(count_saturated_fields(4), (25, 16));
	}
}
