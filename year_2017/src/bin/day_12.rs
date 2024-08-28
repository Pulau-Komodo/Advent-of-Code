fn main() {
	shared::print_answers(12, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let map = make_map(input);

	let mut visited = Vec::with_capacity(map.len());
	visited.extend(std::iter::repeat(false).take(map.len()));

	find_group(&map, &mut visited, 0);

	visited.into_iter().filter(|is_visited| *is_visited).count()
}

fn get_answer_2(input: &str) -> usize {
	let map = make_map(input);

	let mut visited = Vec::with_capacity(map.len());
	visited.extend(std::iter::repeat(false).take(map.len()));
	let mut group_count = 0;

	while let Some(unvisited) = visited
		.iter()
		.enumerate()
		.find_map(|(index, is_visited)| (!is_visited).then_some(index))
	{
		group_count += 1;
		find_group(&map, &mut visited, unvisited)
	}
	
	group_count
}

fn make_map(input: &str) -> Vec<Vec<usize>> {
	input
		.lines()
		.map(|line| {
			let (_program, connections) = line.split_once(" <-> ").unwrap();
			connections
				.split(", ")
				.map(|program| program.parse::<usize>().unwrap())
				.collect()
		})
		.collect()
}

fn find_group(map: &[Vec<usize>], visited: &mut [bool], start: usize) {
	visited[start] = true;
	let mut frontier = vec![start];
	let mut new_frontier = Vec::new();
	loop {
		for program in frontier.drain(..) {
			let connected = &map[program];
			for program in connected {
				let was_visited = visited.get_mut(*program).unwrap();
				if !*was_visited {
					*was_visited = true;
					new_frontier.push(*program)
				}
			}
		}
		if new_frontier.is_empty() {
			break;
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
}
