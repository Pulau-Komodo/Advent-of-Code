use shared::Grid;
use year_2017::KnotHash;

fn main() {
	shared::print_answers(14, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let input = input.trim();
	(0..128)
		.map(|line| KnotHash::new(&format!("{}-{}", input, line)).count_ones())
		.sum()
}

fn get_answer_2(input: &str) -> u32 {
	let input = input.trim();

	let mut grid = Grid::with_margin(
		(0..128).map(|line| {
			let hash = KnotHash::new(&format!("{}-{}", input, line));
			hash.ones_iter().collect::<Vec<_>>()
		}),
		false,
	);

	let mut region_count = 0;
	while let Some(unvisited) = {
		let item = grid
			.iter_with_points::<usize>()
			.find_map(|(point, &visitable)| visitable.then_some(point));
		item
	} {
		region_count += 1;
		*grid.get_point_mut(unvisited) = false;
		let mut frontier = vec![unvisited];
		let mut new_frontier = Vec::new();
		while !frontier.is_empty() {
			for point in frontier.drain(..) {
				for neighbour in point.orthogonal_neighbours() {
					let visitable = grid.get_point_mut(neighbour);
					if *visitable {
						*visitable = false;
						new_frontier.push(neighbour);
					}
				}
			}
			std::mem::swap(&mut frontier, &mut new_frontier);
		}
	}

	region_count
}
