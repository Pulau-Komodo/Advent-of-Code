fn main() {
	shared::print_answers(2, &[get_answers]);
}

fn parse_input(input: &str) -> Vec<[u32; 3]> {
	input
		.lines()
		.map(|line| {
			let mut dimensions = line.split('x').map(str::parse).map(Result::unwrap);
			[
				dimensions.next().unwrap(),
				dimensions.next().unwrap(),
				dimensions.next().unwrap(),
			]
		})
		.collect()
}

fn get_answers(input: &str) -> String {
	let packages = parse_input(input);
	let wrapping_paper = packages
		.iter()
		.map(|edges| {
			let sides = [
				edges[0] * edges[1],
				edges[1] * edges[2],
				edges[2] * edges[0],
			];
			let smallest_side = sides.iter().min().unwrap();
			sides
				.iter()
				.map(|&s| 2 * s)
				.chain(core::iter::once(*smallest_side))
				.sum::<u32>()
		})
		.sum::<u32>();
	let ribbon = packages
		.iter()
		.map(|edges| {
			let (largest_edge, _) = edges
				.iter()
				.enumerate()
				.max_by_key(|&(_, &edge)| edge)
				.unwrap();
			edges
				.iter()
				.enumerate()
				.filter_map(|(index, edge)| {
					if index == largest_edge {
						None
					} else {
						Some(2 * edge)
					}
				})
				.sum::<u32>() + edges.iter().product::<u32>()
		})
		.sum::<u32>();
	format!("1: {}, 2: {}", wrapping_paper, ribbon)
}
